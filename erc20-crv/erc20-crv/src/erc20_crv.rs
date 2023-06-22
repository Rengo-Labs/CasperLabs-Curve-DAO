use crate::data::{self, AdminWhitelist};

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, ContractHash, ContractPackageHash, Key, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::{errors::*, utils::*};
use crv20::{self, Address, CURVEERC20};
use curve_casper_erc20::Error as Erc20Error;

pub enum Erc20CrvEvent {
    Transfer {
        from: Address,
        to: Address,
        value: U256,
    },

    UpdateMiningParameters {
        time: U256,
        rate: U256,
        supply: U256,
    },
    SetMinter {
        minter: Key,
    },
    SetAdmin {
        admin: Key,
    },
    RemoveAdmin {
        admin: Key,
    },
}

impl Erc20CrvEvent {
    pub fn type_name(&self) -> String {
        match self {
            Erc20CrvEvent::Transfer {
                from: _,
                to: _,
                value: _,
            } => "transfer",
            Erc20CrvEvent::UpdateMiningParameters {
                time: _,
                rate: _,
                supply: _,
            } => "update_mining_parameters",
            Erc20CrvEvent::SetMinter { minter: _ } => "set_minter",
            Erc20CrvEvent::SetAdmin { admin: _ } => "set_admin",
            Erc20CrvEvent::RemoveAdmin { admin: _ } => "remove_admin",
        }
        .to_string()
    }
}
pub trait ERC20CRV<Storage: ContractStorage>:
    ContractContext<Storage> + CURVEERC20<Storage>
{
    fn init(&mut self, contract_hash: ContractHash, package_hash: ContractPackageHash) {
        let base: i32 = 10;
        data::set_init_supply(
            data::INITIAL_SUPPLY * (base.pow(u32::from(CURVEERC20::decimals(self)))),
        );
        data::set_is_updated(false);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        AdminWhitelist::init();
        AdminWhitelist::instance().set(&self.get_caller(), true);
        CURVEERC20::init(self, data::get_hash(), data::get_package_hash());
        CURVEERC20::mint(
            self,
            Address::from(self.get_caller()),
            data::get_init_supply(),
        )
        .unwrap_or_revert();
        self.erc20_crv_emit(&Erc20CrvEvent::Transfer {
            from: Address::from(zero_address()),
            to: Address::from(self.get_caller()),
            value: data::get_init_supply(),
        });
        let blocktime_u64: u64 = runtime::get_blocktime().into();
        let blocktime: U256 = U256::from(blocktime_u64);
        let start_epoch_time: U256 = blocktime
            .checked_add(data::INFLATION_DELAY)
            .unwrap_or_revert_with(Error::Erc20CRVOverFlow8)
            .checked_sub(data::RATE_REDUCTION_TIME)
            .unwrap_or_revert_with(Error::Erc20CRVUnderFlow3);
        data::set_start_epoch_time(start_epoch_time);
        data::set_mining_epoch(0.into());
        data::set_rate(0.into());
        data::set_start_epoch_supply(data::get_init_supply());
    }

    fn migrate(&mut self) {
      if data::get_owner() == zero_address() {
        data::set_owner(self.get_caller())
      }
      
      if self.get_caller() != data::get_owner() {
        runtime::revert(ApiError::from(Error::Erc20CRVInvalidOwner));
      }      

      let base: i32 = 10;
      data::set_is_updated(false);
      data::set_init_supply(
        CURVEERC20::total_supply(self),
      );
      AdminWhitelist::init();
      AdminWhitelist::instance().set(&self.get_caller(), true);
      let blocktime_u64: u64 = runtime::get_blocktime().into();
      let blocktime: U256 = U256::from(blocktime_u64);
      let start_epoch_time: U256 = blocktime
          .checked_sub(data::INFLATION_DELAY)
          .unwrap_or_revert_with(Error::Erc20CRVOverFlow8)
          .checked_sub(data::RATE_REDUCTION_TIME)
          .unwrap_or_revert_with(Error::Erc20CRVUnderFlow3);
      data::set_start_epoch_time(start_epoch_time);
      data::set_mining_epoch(0.into());
      data::set_rate(0.into());
      data::set_start_epoch_supply(data::get_init_supply());
      ERC20CRV::burn(self, U256::from(1081656954) * (base.pow(u32::from(CURVEERC20::decimals(self)))));
    }

    /// @dev Update mining rate and supply at the start of the epoch
    /// Any modifying mining call must also call this
    #[inline(always)]
    fn _update_mining_parameters(&self) {
        let mut rate: U256 = data::get_rate();
        let mut start_epoch_supply = data::get_start_epoch_supply();
        data::set_start_epoch_time(
            data::get_start_epoch_time()
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow9),
        );

        if data::get_is_updated() {
            data::set_mining_epoch(
                data::get_mining_epoch()
                    .checked_add(1.into())
                    .unwrap_or_revert_with(Error::Erc20CRVOverFlow10),
            );
        } else {
            data::set_is_updated(true);
        }

        if rate == 0.into() {
            rate = data::INITIAL_RATE;
        } else {
            start_epoch_supply = start_epoch_supply
                .checked_add(
                    rate.checked_mul(data::RATE_REDUCTION_TIME)
                        .unwrap_or_revert_with(Error::Erc20CRVOverFlow22),
                )
                .unwrap_or_revert_with(Error::Erc20CRVAirthmeticError4);
            data::set_start_epoch_supply(start_epoch_supply);
            rate = (rate
                .checked_mul(data::RATE_DENOMINATOR)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow23))
            .checked_div(data::RATE_REDUCTION_COEFFICIENT)
            .unwrap_or_revert_with(Error::Erc20CRVAirthmeticError5);
        }
        data::set_rate(rate);
        let blocktime: u64 = runtime::get_blocktime().into();
        self.erc20_crv_emit(&Erc20CrvEvent::UpdateMiningParameters {
            time: U256::from(blocktime),
            rate,
            supply: data::get_start_epoch_supply(),
        });
    }

    /// @notice Update mining rate and supply at the start of the epoch
    /// @dev Callable by any address, but only once per epochTotal supply becomes slightly larger if this function is called late
    /// Total supply becomes slightly larger if this function is called late
    #[inline(always)]
    fn update_mining_parameters(&self) {
        let blocktime: u64 = runtime::get_blocktime().into();
        if U256::from(blocktime)
            < data::get_start_epoch_time()
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow11)
        {
            runtime::revert(ApiError::from(Error::Erc20CRVTooSoon));
        }
        self._update_mining_parameters();
    }
    ///@notice Get timestamp of the current mining epoch start
    ///        while simultaneously updating mining parameters
    ///@return Timestamp of the epoch
    #[inline(always)]
    fn start_epoch_time_write(&self) -> U256 {
        let start_epoch_time = data::get_start_epoch_time();
        let blocktime: u64 = runtime::get_blocktime().into();
        if U256::from(blocktime)
            >= data::get_start_epoch_time()
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow12)
        {
            self._update_mining_parameters();
            data::get_start_epoch_time()
        } else {
            start_epoch_time
        }
    }
    ///@notice Get timestamp of the next mining epoch start
    ///        while simultaneously updating mining parameters
    ///@return Timestamp of the next epoch
    #[inline(always)]
    fn future_epoch_time_write(&self) -> U256 {
        let start_epoch_time = data::get_start_epoch_time();
        let blocktime: u64 = runtime::get_blocktime().into();
        if U256::from(blocktime)
            >= data::get_start_epoch_time()
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow13)
        {
            self._update_mining_parameters();
            data::get_start_epoch_time()
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow14)
        } else {
            start_epoch_time
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow15)
        }
    }

    #[inline(always)]
    fn _available_supply(&self) -> U256 {
        let blocktime: u64 = runtime::get_blocktime().into();
        let blocktime_sub_st_epoch: U256 = U256::from(blocktime)
            .checked_sub(data::get_start_epoch_time())
            .unwrap_or_revert_with(Error::Erc20CRVUnderFlow4);
        let ans: U256 = blocktime_sub_st_epoch
            .checked_mul(data::get_rate())
            .unwrap_or_revert_with(Error::Erc20CRVOverFlow24);
        data::get_start_epoch_supply()
            .checked_add(ans)
            .unwrap_or_revert_with(Error::Erc20CRVOverFlow16)
    }
    ///@notice Current number of tokens in existence (claimed or unclaimed)
    #[inline(always)]
    fn available_supply(&self) -> U256 {
        self._available_supply()
    }
    ///@notice How much supply is mintable from start timestamp till end timestamp
    ///@param start Start of the time interval (timestamp)
    ///@param end End of the time interval (timestamp)
    ///@return Tokens mintable from `start` till `end`
    #[inline(always)]
    fn mintable_in_timeframe(&self, start: U256, end: U256) -> U256 {
        if start > end {
            runtime::revert(ApiError::from(Error::Erc20CRVStartGreaterThanEnd));
        }
        let mut to_mint: U256 = 0.into();
        let mut current_epoch_time: U256 = data::get_start_epoch_time();
        let mut current_rate: U256 = data::get_rate();
        if end
            > current_epoch_time
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow1)
        {
            current_epoch_time = current_epoch_time
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow2);
            current_rate = current_rate
                .checked_mul(
                    data::RATE_DENOMINATOR
                        .checked_div(data::RATE_REDUCTION_COEFFICIENT)
                        .unwrap_or_revert_with(Error::Erc20CRVUnderFlow2),
                )
                .unwrap_or_revert_with(Error::Erc20CRVAirthmeticError1);
        }
        if end
            > current_epoch_time
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow3)
        {
            runtime::revert(ApiError::from(Error::Erc20CRVTooFarInFuture));
        }
        let mut current_end: U256;
        let mut current_start: U256;

        for _i in 0..999 {
            if end >= current_epoch_time {
                current_end = end;
                if current_end
                    > current_epoch_time
                        .checked_add(data::RATE_REDUCTION_TIME)
                        .unwrap_or_revert_with(Error::Erc20CRVOverFlow4)
                {
                    current_end = current_epoch_time
                        .checked_add(data::RATE_REDUCTION_TIME)
                        .unwrap_or_revert_with(Error::Erc20CRVOverFlow5);
                }
                current_start = start;
                if current_start
                    >= current_epoch_time
                        .checked_add(data::RATE_REDUCTION_TIME)
                        .unwrap_or_revert_with(Error::Erc20CRVOverFlow6)
                {
                    break;
                } else if current_start < current_epoch_time {
                    current_start = current_epoch_time;
                }
                let current_end_sub_current_st: U256 = current_end
                    .checked_sub(current_start)
                    .unwrap_or_revert_with(Error::Erc20CRVOverFlow7);
                to_mint = to_mint
                    .checked_add(current_rate)
                    .unwrap_or_revert_with(Error::Erc20CRVOverFlow21)
                    .checked_mul(current_end_sub_current_st)
                    .unwrap_or_revert_with(Error::Erc20CRVAirthmeticError2);
                if start >= current_epoch_time {
                    break;
                }
            }
            current_epoch_time = current_epoch_time
                .checked_sub(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVUnderFlow1);
            current_rate = current_rate
                .checked_mul(
                    data::RATE_REDUCTION_COEFFICIENT
                        .checked_div(data::RATE_DENOMINATOR)
                        .unwrap_or_revert_with(Error::Erc20CRVUnderFlow5),
                )
                .unwrap_or_revert_with(Error::Erc20CRVAirthmeticError3);
            if current_rate > data::INITIAL_RATE {
                runtime::revert(ApiError::from(Error::Erc20CRVCurrRateLessThanInitRate));
            }
        }
        to_mint
    }
    ///@notice Set the minter address
    ///@dev Only callable once, when minter has not yet been set
    ///@param minter Address of the minter
    #[inline(always)]
    fn set_minter(&self, minter: Key) {
        if !AdminWhitelist::instance().get(&self.get_caller()) {
            runtime::revert(ApiError::from(Error::Erc20CRVInvalidAdmin1));
        }
        if minter == zero_address() || minter == account_zero_address() {
            runtime::revert(ApiError::from(Error::Erc20CRVZeroAddress1));
        }
        data::set_minter(minter);
        self.erc20_crv_emit(&Erc20CrvEvent::SetMinter { minter });
    }
    ///@notice Set the new admin.
    ///@dev After all is set up, admin only can change the token name
    ///@param _admin New admin address
    #[inline(always)]
    fn set_admin(&self, admin: Key) {
        if !AdminWhitelist::instance().get(&self.get_caller()) {
            runtime::revert(ApiError::from(Error::Erc20CRVAdminOnly2));
        }
        if AdminWhitelist::instance().get(&admin) {
            runtime::revert(ApiError::from(Error::Erc20CRVAlreadyAdded));
        }
        AdminWhitelist::instance().set(&admin, true);
        self.erc20_crv_emit(&Erc20CrvEvent::SetAdmin { admin });
    }
    ///@notice Remove an admin.
    ///@dev After all is set up, admin only can change the token name
    ///@param _admin Removal admin address
    #[inline(always)]
    fn remove_admin(&self, admin: Key) {
        if !AdminWhitelist::instance().get(&self.get_caller()) {
            runtime::revert(ApiError::from(Error::Erc20CRVAdminOnly1));
        }
        if !AdminWhitelist::instance().get(&admin) {
            runtime::revert(ApiError::from(Error::Erc20CRVAlreadyRemoved));
        }
        AdminWhitelist::instance().set(&admin, false);
        self.erc20_crv_emit(&Erc20CrvEvent::RemoveAdmin { admin });
    }
    ///@notice Mint `amount` tokens and assign them to `to`
    ///@dev Emits a Transfer event originating from 0x00
    ///@param to The account that will receive the created tokens
    ///@param amount The amount that will be created
    #[inline(always)]
    fn mint(&self, to: Address, amount: U256) -> Result<(), Error> {
        if Address::from(self.get_caller()) != Address::from(data::get_minter()) {
            runtime::revert(ApiError::from(Error::Erc20CRVMinterOnly));
        }
        if to == Address::from(zero_address()) {
            runtime::revert(ApiError::from(Error::Erc20CRVZeroAddress));
        }
        let blocktime: u64 = runtime::get_blocktime().into();
        if U256::from(blocktime)
            >= data::get_start_epoch_time()
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow17)
        {
            self._update_mining_parameters();
        }
        let total_supply: U256 = CURVEERC20::total_supply(self)
            .checked_add(amount)
            .unwrap_or_revert_with(Error::Erc20CRVOverFlow18);
        if total_supply > self.available_supply() {
            runtime::revert(ApiError::from(Error::Erc20CRVExceedsAllowableMint));
        }
        CURVEERC20::mint(self, to, amount).unwrap_or_revert();

        self.erc20_crv_emit(&Erc20CrvEvent::Transfer {
            from: Address::from(zero_address()),
            to,
            value: amount,
        });
        Ok(())
    }
    ///@notice Change the token name and symbol to `name` and `symbol`
    ///@dev Only callable by the admin account
    ///@param name New token name
    ///@param symbol New token symbol
    #[inline(always)]
    fn set_name(&self, name: String, symbol: String) {
        if !AdminWhitelist::instance().get(&self.get_caller()) {
            runtime::revert(ApiError::from(Error::Erc20CRVInvalidAdmin2));
        }
        CURVEERC20::set_name(self, name);
        CURVEERC20::set_symbol(self, symbol);
    }
    ///@notice Burn `value` tokens belonging to `msg.sender`
    ///@dev Emits a Transfer event with a destination of 0x00
    ///@param value The amount that will be burned
    #[inline(always)]
    fn burn(&self, value: U256) -> Result<(), Error> {
        CURVEERC20::burn(self, Address::from(self.get_caller()), value).unwrap_or_revert();
        self.erc20_crv_emit(&Erc20CrvEvent::Transfer {
            from: Address::from(self.get_caller()),
            to: Address::from(zero_address()),
            value,
        });
        Ok(())
    }
    #[inline(always)]
    fn named_keys_erc20crv(
        &self,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> Result<BTreeMap<String, Key>, Erc20Error> {
        CURVEERC20::named_keys(self, name, symbol, decimals, 0.into())
    }
    fn erc20_crv_emit(&self, erc20_crv_event: &Erc20CrvEvent) {
        let package = data::get_package_hash();
        match erc20_crv_event {
            Erc20CrvEvent::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc20_crv_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            Erc20CrvEvent::UpdateMiningParameters { time, rate, supply } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc20_crv_event.type_name());
                event.insert("time", time.to_string());
                event.insert("rate", rate.to_string());
                event.insert("supply", supply.to_string());
                storage::new_uref(event);
            }
            Erc20CrvEvent::SetMinter { minter } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc20_crv_event.type_name());
                event.insert("minter", minter.to_string());
                storage::new_uref(event);
            }
            Erc20CrvEvent::SetAdmin { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc20_crv_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            Erc20CrvEvent::RemoveAdmin { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc20_crv_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
        };
    }
}
