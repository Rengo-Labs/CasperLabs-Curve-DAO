use crate::data::*;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs, U256,
};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::{errors::*, utils::*};

pub enum VestingEscrowSimpleEvent {
    Fund { recipient: Key, amount: U256 },
    Claim { recipient: Key, claimed: U256 },
    ToggleDisable { recipient: Key, disabled: bool },
    CommitOwnership { admin: Key },
    ApplyOwnership { admin: Key },
}

impl VestingEscrowSimpleEvent {
    pub fn type_name(&self) -> String {
        match self {
            VestingEscrowSimpleEvent::Fund {
                recipient: _,
                amount: _,
            } => "fund",
            VestingEscrowSimpleEvent::Claim {
                recipient: _,
                claimed: _,
            } => "claim",
            VestingEscrowSimpleEvent::ToggleDisable {
                recipient: _,
                disabled: _,
            } => "toggle_disable",
            VestingEscrowSimpleEvent::CommitOwnership { admin: _ } => "commit_ownership",
            VestingEscrowSimpleEvent::ApplyOwnership { admin: _ } => "apply_ownership",
        }
        .to_string()
    }
}
#[allow(clippy::too_many_arguments)]
pub trait VESTINGESCROWSIMPLE<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&self, contract_hash: ContractHash, package_hash: ContractPackageHash) {
        set_hash(contract_hash);
        set_package_hash(package_hash);
        DisableddAt::init();
        InitialLocked::init();
        TotalClaimed::init();

        set_admin(self.get_caller());
    }

    /// @notice Initialize the contract.
    /// @dev This function is seperate from `__init__` because of the factory pattern
    ///      used in `VestingEscrowFactory.deploy_vesting_contract`. It may be called
    ///      once per deployment.
    /// @param _admin Admin address
    /// @param _token Address of the ERC20 token being distributed
    /// @param _recipient Address to vest tokens for
    /// @param _amount Amount of tokens being vested for `_recipient`
    /// @param _start_time Epoch time at which token distribution starts
    /// @param _end_time Time until everything should be vested
    /// @param _can_disable Can admin disable recipient's ability to claim tokens?
    #[inline(always)]
    fn initialize(
        &self,
        admin: Key,
        token: Key,
        recipient: Key,
        amount: U256,
        start_time: U256,
        end_time: U256,
        can_disable: bool,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) -> bool {
        let lock = get_lock();
        if lock {
            runtime::revert(Error::VestingEscrowSimpleLocked1);
        }
        DisableddAt::init();
        InitialLocked::init();
        TotalClaimed::init();
        set_lock(true);
        set_hash(contract_hash);
        set_package_hash(package_hash);
        if get_admin() != zero_address() {
            runtime::revert(ApiError::from(Error::VestingEscrowSimpleOnlyInitializeOnce));
        }
        set_token(token);
        set_admin(admin);
        set_start_time(start_time);
        set_end_time(end_time);
        set_can_disable(can_disable);

        let () = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "transfer_from",
            runtime_args! {
                "owner" =>self.get_caller(),
                "recipient" => Key::from(get_package_hash()),
                "amount" => amount
            },
        );
        InitialLocked::instance().set(&recipient, amount);
        set_initial_locked_supply(amount);
        self.vesting_escrow_simple_emit(&VestingEscrowSimpleEvent::Fund { recipient, amount });
        set_lock(false);
        true
    }

    ///  @notice Disable or re-enable a vested address's ability to claim tokens
    /// @dev When disabled, the address is only unable to claim tokens which are still
    ///      locked at the time of this call. It is not possible to block the claim
    ///      of tokens which have already vested.
    /// @param _recipient address to disable or enable
    #[inline(always)]
    fn toggle_disable(&self, recipient: Key) {
        if get_admin() != self.get_caller() {
            runtime::revert(ApiError::from(Error::VestingEscrowSimpleAdminOnly1));
        }
        if !(get_can_disable()) {
            runtime::revert(ApiError::from(Error::VestingEscrowSimpleCannotDisable));
        }
        let mut is_disabled: bool = false;
        let blocktime: u64 = runtime::get_blocktime().into();
        if DisableddAt::instance().get(&recipient) == 0.into() {
            is_disabled = true;
        }
        if is_disabled {
            DisableddAt::instance().set(&recipient, U256::from(blocktime))
        } else {
            DisableddAt::instance().set(&recipient, U256::from(0))
        }
        self.vesting_escrow_simple_emit(&VestingEscrowSimpleEvent::ToggleDisable {
            recipient,
            disabled: is_disabled,
        });
    }

    /// @notice Disable the ability to call `toggle_disable`
    #[inline(always)]
    fn disable_can_disable(&self) {
        if get_admin() != self.get_caller() {
            runtime::revert(ApiError::from(Error::VestingEscrowSimpleAdminOnly2));
        }
        set_can_disable(false);
    }
    #[inline(always)]
    fn _total_vested_of(&self, recipient: Key, time: Option<U256>) -> U256 {
        let blocktime: u64 = runtime::get_blocktime().into();
        let _time: U256 = if let Some(..) = time {
            time.unwrap()
        } else {
            U256::from(blocktime)
        };
        let start: U256 = get_start_time();
        let end: U256 = get_end_time();
        let locked: U256 = InitialLocked::instance().get(&recipient);
        if _time < start {
            return 0.into();
        }
        let ans: U256 = locked
            .checked_mul(_time.checked_sub(start).unwrap_or_revert())
            .unwrap_or_revert()
            .checked_div(end.checked_sub(start).unwrap_or_revert())
            .unwrap_or_revert_with(Error::VestingEscrowSimpleAirthmeticError1);
        ans.min(locked)
    }

    #[inline(always)]
    fn _total_vested(&self) -> U256 {
        let start: U256 = get_start_time();
        let end: U256 = get_end_time();
        let locked: U256 = get_initial_locked_supply();
        let temp_blocktime: u64 = runtime::get_blocktime().into();
        let blocktime: U256 = U256::from(temp_blocktime);
        if blocktime < start {
            return 0.into();
        }
        let ans: U256 = locked
            .checked_mul(blocktime.checked_sub(start).unwrap_or_revert())
            .unwrap_or_revert()
            .checked_div(end.checked_sub(start).unwrap_or_revert())
            .unwrap_or_revert_with(Error::VestingEscrowSimpleAirthmeticError2);
        ans.min(locked)
    }

    /// @notice Get the total number of tokens which have vested, that are held
    /// by this contract
    #[inline(always)]
    fn vested_supply(&self) -> U256 {
        self._total_vested()
    }

    /// @notice Get the total number of tokens which are still locked
    /// (have not yet vested)
    #[inline(always)]
    fn locked_supply(&self) -> U256 {
        let initial_locked_supply = get_initial_locked_supply();
        let total_vested: U256 = self._total_vested();
        initial_locked_supply
            .checked_sub(total_vested)
            .unwrap_or_revert_with(Error::VestingEscrowSimpleUnderFlow1)
    }

    /// @notice Get the number of tokens which have vested for a given address
    /// @param _recipient address to check
    #[inline(always)]
    fn vested_of(&self, recipient: Key) -> U256 {
        let blocktime: u64 = runtime::get_blocktime().into();
        self._total_vested_of(recipient, Some(U256::from(blocktime)))
    }

    /// @notice Get the number of unclaimed, vested tokens for a given address
    /// @param _recipient address to check
    #[inline(always)]
    fn balance_of(&self, recipient: Key) -> U256 {
        let blocktime: u64 = runtime::get_blocktime().into();
        let total_vested_of: U256 = self._total_vested_of(recipient, Some(U256::from(blocktime)));
        let self_total_claimed: U256 = TotalClaimed::instance().get(&recipient);
        total_vested_of
            .checked_sub(self_total_claimed)
            .unwrap_or_revert_with(Error::VestingEscrowSimpleUnderFlow2)
    }

    /// @notice Get the number of locked tokens for a given address
    /// @param _recipient address to check
    #[inline(always)]
    fn locked_of(&self, recipient: Key) -> U256 {
        let initial_locked = InitialLocked::instance().get(&recipient);
        let blocktime: u64 = runtime::get_blocktime().into();
        let total_vested_of: U256 = self._total_vested_of(recipient, Some(U256::from(blocktime)));
        initial_locked
            .checked_sub(total_vested_of)
            .unwrap_or_revert_with(Error::VestingEscrowSimpleUnderFlow3)
    }

    /// @notice Transfer ownership of GaugeController to `addr`
    /// @param addr Address to have ownership transferred to
    #[inline(always)]
    fn commit_transfer_ownership(&self, addr: Key) -> bool {
        if get_admin() != self.get_caller() {
            runtime::revert(ApiError::from(Error::VestingEscrowSimpleAdminOnly3));
        }
        set_future_admin(addr);
        self.vesting_escrow_simple_emit(&VestingEscrowSimpleEvent::CommitOwnership { admin: addr });

        true
    }

    /// @notice Claim tokens which have vested
    /// @param addr Address to claim tokens for
    #[inline(always)]
    fn claim(&self, addr: Option<Key>) {
        //Locked
        if get_lock() {
            runtime::revert(ApiError::from(Error::VestingEscrowSimpleLocked2));
        }
        set_lock(true);
        let _addr: Key = if let Some(..) = addr {
            addr.unwrap()
        } else {
            self.get_caller()
        };
        let mut t: U256 = DisableddAt::instance().get(&_addr);
        let blocktime: U256 = 1000.into();
        if t == U256::from(0) {
            t = blocktime;
        }
        let _total_vested_of: U256 = self._total_vested_of(_addr, Some(t));
        let _total_claimed: U256 = TotalClaimed::instance().get(&_addr);
        let claimable: U256 = _total_vested_of
            .checked_sub(_total_claimed)
            .unwrap_or_revert_with(Error::VestingEscrowSimpleUnderFlow4);
        let updated_total_claimed = _total_claimed.checked_add(claimable).unwrap_or_revert();
        TotalClaimed::instance().set(&_addr, updated_total_claimed);
        let token: Key = get_token();
        let () = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" =>addr,
                "amount" => claimable,
            },
        );
        self.vesting_escrow_simple_emit(&VestingEscrowSimpleEvent::Claim {
            recipient: _addr,
            claimed: claimable,
        });
    }

    /// @notice Apply pending ownership transfer
    #[inline(always)]
    fn apply_transfer_ownership(&self) -> bool {
        if self.get_caller() != get_admin() {
            runtime::revert(ApiError::from(Error::VestingEscrowSimpleAdminOnly4));
        }
        let mut _admin: Key = get_future_admin();
        if _admin == zero_address() {
            runtime::revert(ApiError::from(Error::VestingEscrowSimpleAdminNotSet));
        }
        set_admin(_admin);
        self.vesting_escrow_simple_emit(&VestingEscrowSimpleEvent::ApplyOwnership {
            admin: _admin,
        });
        true
    }

    fn vesting_escrow_simple_emit(&self, vesting_escrow_simple_event: &VestingEscrowSimpleEvent) {
        let package = get_package_hash();
        match vesting_escrow_simple_event {
            VestingEscrowSimpleEvent::Fund { recipient, amount } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", vesting_escrow_simple_event.type_name());
                event.insert("recipient", recipient.to_string());
                event.insert("amount", amount.to_string());
                storage::new_uref(event);
            }
            VestingEscrowSimpleEvent::Claim { recipient, claimed } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", vesting_escrow_simple_event.type_name());
                event.insert("recipient", recipient.to_string());
                event.insert("claimed", claimed.to_string());
                storage::new_uref(event);
            }
            VestingEscrowSimpleEvent::ToggleDisable {
                recipient,
                disabled,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", vesting_escrow_simple_event.type_name());
                event.insert("recipient", recipient.to_string());
                event.insert("disabled", disabled.to_string());
                storage::new_uref(event);
            }
            VestingEscrowSimpleEvent::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", vesting_escrow_simple_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            VestingEscrowSimpleEvent::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", vesting_escrow_simple_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
        };
    }
}
