use crate::{data::*, event::LiquidityGaugeWrapperEvent};
use alloc::string::String;
use alloc::{collections::BTreeMap, string::ToString};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs, U256,
};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::{errors::*, utils::*};
use crv20::{self, Address, CURVEERC20};
use curve_casper_erc20::Error as Erc20Error;
pub trait LIQUIDITYGAUGEWRAPPER<Storage: ContractStorage>:
    ContractContext<Storage> + CURVEERC20<Storage>
{
    /// @notice Contract constructor
    /// @param _name Token full name
    /// @param _symbol Token symbol
    /// @param _gauge Liquidity gauge contract address
    /// @param _admin Admin who can kill the gauge
    fn init(
        &self,
        name: String,
        symbol: String,
        gauge: Key,
        admin: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        CURVEERC20::init(self, contract_hash, package_hash);
        self.set_name(name);
        self.set_symbol(symbol);
        let lp_token: Key = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "lp_token",
            runtime_args! {},
        );
        let () = runtime::call_versioned_contract(
            lp_token.into_hash().unwrap_or_revert().into(),
            None,
            "approve",
            runtime_args! {
                "spender" => gauge,
                "amount" => U256::MAX
            },
        );
        let minter: Key = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "minter",
            runtime_args! {},
        );
        set_minter(minter);
        let crv_token: Key = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "crv_token",
            runtime_args! {},
        );
        set_crv_token(crv_token);
        set_lp_token(lp_token);
        set_gauge(gauge);
        set_admin(admin);
        CrvIntegralFor::init();
        ClaimableCrv::init();
        ApprovedToDeposit::init();
        set_contract_hash(contract_hash);
        set_package_hash(package_hash);
    }
    #[inline(always)]
    fn _checkpoint(&self, addr: Key) {
        let crv_token: Key = get_crv_token();
        let mut d_reward: U256 = runtime::call_versioned_contract(
            crv_token.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "address" =>  Key::from(get_package_hash())
            },
        );
        let () = runtime::call_versioned_contract(
            get_minter().into_hash().unwrap_or_revert().into(),
            None,
            "mint",
            runtime_args! {
                "gauge_addr" => get_gauge()
            },
        );
        let d_reward_updated: U256 = runtime::call_versioned_contract(
            crv_token.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "address" =>  Key::from(get_package_hash())
            },
        );
        d_reward = d_reward_updated
            .checked_sub(d_reward)
            .unwrap_or_revert_with(Error::GaugeWrapperSubtractionError1);
        let total_balance: U256 = self.total_supply();
        let mut di: U256 = 0.into();
        if total_balance > 0.into() {
            di = U256::from(TEN_E_NINE)
                .checked_mul(d_reward)
                .unwrap_or_revert_with(Error::GaugeWrapperMultiplyError1)
                .checked_div(total_balance)
                .unwrap_or_revert_with(Error::GaugeWrapperDivisionError1);
        }
        let i: U256 = get_crv_integral()
            .checked_add(di)
            .unwrap_or_revert_with(Error::GaugeWrapperAdditionError1);
        set_crv_integral(i);
        let balance_of: U256 = self.balance_of(Address::from(addr));
        let crv_integral_for: U256 = CrvIntegralFor::instance().get(&addr);
        ClaimableCrv::instance().set(
            &addr,
            ClaimableCrv::instance()
                .get(&addr)
                .checked_add(balance_of)
                .unwrap_or_revert_with(Error::GaugeWrapperAdditionError2)
                .checked_mul(
                    i.checked_sub(crv_integral_for)
                        .unwrap_or_revert_with(Error::GaugeWrapperSubtractionError2),
                )
                .unwrap_or_revert_with(Error::GaugeWrapperMultiplyError2)
                .checked_div(U256::from(TEN_E_NINE))
                .unwrap_or_revert_with(Error::GaugeWrapperDivisionError2),
        );
        CrvIntegralFor::instance().set(&addr, i);
    }
    /// @notice Record a checkpoint for `addr`
    /// @param addr User address
    /// @return bool success
    #[inline(always)]
    fn user_checkpoint(&self, addr: Key) -> bool {
        if !((self.get_caller() == addr) || (self.get_caller() == get_minter())) {
            runtime::revert(ApiError::from(Error::GaugeWrapperUnauthorized));
        }
        self._checkpoint(addr);
        true
    }

    /// @notice Get the number of claimable tokens per user
    /// @dev This function should be manually changed to "view" in the ABI
    /// @return uint256 number of claimable tokens per user
    #[inline(always)]
    fn claimable_tokens(&self, addr: Key) -> U256 {
        let d_reward: U256 = runtime::call_versioned_contract(
            get_gauge().into_hash().unwrap_or_revert().into(),
            None,
            "claimable_tokens",
            runtime_args! {
                "addr" => Key::from(get_package_hash())
            },
        );
        let total_balance: U256 = self.total_supply();
        let mut di: U256 = 0.into();
        if total_balance > 0.into() {
            di = U256::from(TEN_E_NINE)
                .checked_mul(d_reward)
                .unwrap_or_revert_with(Error::GaugeWrapperMultiplyError3)
                .checked_div(total_balance)
                .unwrap_or_revert_with(Error::GaugeWrapperDivisionError3);
        }
        let i: U256 = get_crv_integral()
            .checked_add(di)
            .unwrap_or_revert_with(Error::GaugeWrapperAdditionError3);
        let balance_of: U256 = self.balance_of(Address::from(addr));
        let crv_integral_for: U256 = CrvIntegralFor::instance().get(&addr);
        let claimable_crv: U256 = ClaimableCrv::instance().get(&addr);
        claimable_crv
            .checked_add(balance_of)
            .unwrap_or_revert_with(Error::GaugeWrapperAdditionError4)
            .checked_mul(
                i.checked_sub(crv_integral_for)
                    .unwrap_or_revert_with(Error::GaugeWrapperSubtractionError3),
            )
            .unwrap_or_revert_with(Error::GaugeWrapperMultiplyError4)
            .checked_div(U256::from(TEN_E_NINE))
            .unwrap_or_revert_with(Error::GaugeWrapperDivisionError4)
    }
    /// @notice Claim mintable CR
    /// @param addr Address to claim for
    #[inline(always)]
    fn claim_tokens(&self, addr: Option<Key>) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::GaugeWrapperIsLocked1));
        }
        set_lock(true);
        let addr: Key = match addr {
            Some(val) => val,
            None => self.get_caller(),
        };
        self._checkpoint(addr);
        let () = runtime::call_versioned_contract(
            get_crv_token().into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => addr,
                "amount" => ClaimableCrv::instance().get(&addr)
            },
        );
        ClaimableCrv::instance().set(&addr, 0.into());
        set_lock(false);
    }

    /// @notice Set whether `addr` can deposit tokens for `self.get_caller()`
    /// @param addr Address to set approval on
    /// @param can_deposit bool - can this account deposit for `self.get_caller()`?
    #[inline(always)]
    fn set_approve_deposit(&self, addr: Key, can_deposit: bool) {
        ApprovedToDeposit::instance().set(&addr, &self.get_caller(), can_deposit);
    }

    /// @notice Deposit `_value` LP tokens
    /// @param _value Number of tokens to deposit
    /// @param addr Address to deposit for
    #[inline(always)]
    fn deposit(&self, value: U256, addr: Option<Key>) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::GaugeWrapperIsLocked2));
        }
        set_lock(true);
        let addr: Key = match addr {
            Some(val) => val,
            None => self.get_caller(),
        };
        if get_is_killed() {
            runtime::revert(ApiError::from(Error::GaugeWrapperIsKilled1));
        }
        if addr != self.get_caller()
            && !(ApprovedToDeposit::instance().get(&self.get_caller(), &addr))
        {
            runtime::revert(ApiError::from(Error::GaugeWrapperNotApproved));
        }
        self._checkpoint(addr);
        if value != 0.into() {
            let balance: U256 = self
                .balance_of(Address::from(addr))
                .checked_add(value)
                .unwrap_or_revert_with(Error::GaugeWrapperAdditionError5);
            let supply: U256 = self
                .total_supply()
                .checked_add(value)
                .unwrap_or_revert_with(Error::GaugeWrapperAdditionError6);
            self.set_balance(Address::from(addr), balance);
            self.set_total_supply(supply);
            let () = runtime::call_versioned_contract(
                get_lp_token().into_hash().unwrap_or_revert().into(),
                None,
                "transfer_from",
                runtime_args! {
                    "owner" => self.get_caller(),
                    "recipient" => Key::from(get_package_hash()),
                    "amount" => value
                },
            );
            let () = runtime::call_versioned_contract(
                get_gauge().into_hash().unwrap_or_revert().into(),
                None,
                "deposit",
                runtime_args! {
                    "value" => value,
                    "addr" => None::<Key>,
                    "claim_rewards" => None::<bool>,
                },
            );
        }
        LIQUIDITYGAUGEWRAPPER::emit(
            self,
            &LiquidityGaugeWrapperEvent::Deposit {
                provider: addr,
                value,
            },
        );
        LIQUIDITYGAUGEWRAPPER::emit(
            self,
            &LiquidityGaugeWrapperEvent::Transfer {
                from: zero_address(),
                to: addr,
                value,
            },
        );
        set_lock(false);
    }

    /// @notice Withdraw `_value` LP tokens
    /// @param _value Number of tokens to withdraw
    #[inline(always)]
    fn withdraw(&self, value: U256) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::GaugeWrapperIsLocked3));
        }
        set_lock(true);
        self._checkpoint(self.get_caller());
        if value != 0.into() {
            let balance: U256 = self
                .balance_of(Address::from(self.get_caller()))
                .checked_sub(value)
                .unwrap_or_revert_with(Error::GaugeWrapperSubtractionError4);
            let supply: U256 = self
                .total_supply()
                .checked_sub(value)
                .unwrap_or_revert_with(Error::GaugeWrapperSubtractionError5);
            self.set_balance(Address::from(self.get_caller()), balance);
            self.set_total_supply(supply);
            let () = runtime::call_versioned_contract(
                get_gauge().into_hash().unwrap_or_revert().into(),
                None,
                "withdraw",
                runtime_args! {
                    "value" => value,
                    "claim_rewards" => None::<bool>
                },
            );
            let () = runtime::call_versioned_contract(
                get_lp_token().into_hash().unwrap_or_revert().into(),
                None,
                "transfer",
                runtime_args! {
                    "recipient" => self.get_caller(),
                    "amount" => value
                },
            );
        }
        LIQUIDITYGAUGEWRAPPER::emit(
            self,
            &LiquidityGaugeWrapperEvent::Withdraw {
                provider: self.get_caller(),
                value,
            },
        );
        LIQUIDITYGAUGEWRAPPER::emit(
            self,
            &LiquidityGaugeWrapperEvent::Transfer {
                from: self.get_caller(),
                to: zero_address(),
                value,
            },
        );
        set_lock(false);
    }
    /// @dev Function to check the amount of tokens that an owner allowed to a spender.
    /// @param _owner The address which owns the funds.
    /// @param _spender The address which will spend the funds.
    /// @return An uint256 specifying the amount of tokens still available for the spender.
    #[inline(always)]
    fn allowance(&self, owner: Address, spender: Address) -> U256 {
        CURVEERC20::allowance(self, owner, spender)
    }
    #[inline(always)]
    fn _transfer(&self, owner: Key, recipient: Key, amount: U256) {
        if get_is_killed() {
            runtime::revert(ApiError::from(Error::GaugeWrapperIsKilled2));
        }
        self._checkpoint(owner);
        self._checkpoint(recipient);
        if amount != 0.into() {
            let balance_owner: U256 = self
                .balance_of(Address::from(owner))
                .checked_sub(amount)
                .unwrap_or_revert_with(Error::GaugeWrapperSubtractionError6);
            self.set_balance(Address::from(owner), balance_owner);
            let balance_recipient: U256 = self
                .balance_of(Address::from(recipient))
                .checked_add(amount)
                .unwrap_or_revert_with(Error::GaugeWrapperAdditionError7);

            self.set_balance(Address::from(recipient), balance_recipient);
        }
        LIQUIDITYGAUGEWRAPPER::emit(
            self,
            &LiquidityGaugeWrapperEvent::Transfer {
                from: owner,
                to: recipient,
                value: amount,
            },
        );
    }
    /// @dev Transfer token for a specified address
    /// @param _to The address to transfer to.
    /// @param _value The amount to be transferred.
    #[inline(always)]
    fn transfer(&mut self, recipient: Address, amount: U256) -> Result<(), Error> {
        self._transfer(self.get_caller(), Key::from(recipient), amount);
        Ok(())
    }
    /// @dev Transfer tokens from one address to another.
    /// @param _from address The address which you want to send tokens from
    /// @param _to address The address which you want to transfer to
    /// @param _value uint256 the amount of tokens to be transferred
    #[inline(always)]
    fn transfer_from(
        &mut self,
        owner: Address,
        recipient: Address,
        amount: U256,
    ) -> Result<(), Error> {
        let allowance: U256 = CURVEERC20::allowance(self, owner, Address::from(self.get_caller()));
        if allowance != U256::MAX {
            CURVEERC20::set_allowance(
                self,
                owner,
                Address::from(self.get_caller()),
                allowance
                    .checked_sub(amount)
                    .unwrap_or_revert_with(Error::GaugeWrapperSubtractionError7),
            );
        }
        self._transfer(Key::from(owner), Key::from(recipient), amount);
        Ok(())
    }
    /// @notice Approve the passed address to transfer the specified amount of
    ///  tokens on behalf of msg.sender
    ///  @dev Beware that changing an allowance via this method brings the risk
    ///  that someone may use both the old and new allowance by unfortunate
    ///  transaction ordering. This may be mitigated with the use of
    ///  {increaseAllowance} and {decreaseAllowance}.
    ///  https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
    /// @param _spender The address which will transfer the funds
    /// @param _value The amount of tokens that may be transferred
    #[inline(always)]
    fn approve(&self, spender: Address, amount: U256) -> Result<(), Erc20Error> {
        CURVEERC20::approve(self, spender, amount)
    }
    /// @notice Increase the allowance granted to `_spender` by the caller
    /// @dev This is alternative to {approve} that can be used as a mitigation for
    ///      the potential race condition
    /// @param _spender The address which will transfer the funds
    /// @param _added_value The amount of to increase the allowance
    /// @return Result on success
    #[inline(always)]
    fn increase_allowance(&self, spender: Address, amount: U256) -> Result<(), Erc20Error> {
        let res = CURVEERC20::increase_allowance(self, spender, amount);
        LIQUIDITYGAUGEWRAPPER::emit(
            self,
            &LiquidityGaugeWrapperEvent::Approval {
                owner: self.get_caller(),
                spender: Key::from(spender),
                value: CURVEERC20::allowance(self, Address::from(self.get_caller()), spender),
            },
        );
        res
    }
    /// @notice Decrease the allowance granted to `_spender` by the caller
    /// @dev This is alternative to {approve} that can be used as a mitigation for
    ///      the potential race condition
    /// @param _spender The address which will transfer the funds
    /// @param _subtracted_value The amount of to decrease the allowance
    /// @return Result on success
    #[inline(always)]
    fn decrease_allowance(&self, spender: Address, amount: U256) -> Result<(), Erc20Error> {
        let res = CURVEERC20::decrease_allowance(self, spender, amount);
        LIQUIDITYGAUGEWRAPPER::emit(
            self,
            &LiquidityGaugeWrapperEvent::Approval {
                owner: self.get_caller(),
                spender: Key::from(spender),
                value: CURVEERC20::allowance(self, Address::from(self.get_caller()), spender),
            },
        );
        res
    }
    #[inline(always)]
    fn kill_me(&self) {
        if self.get_caller() != get_admin() {
            runtime::revert(ApiError::from(Error::GaugeWrapperAdminOnly1));
        }
        set_is_killed(!get_is_killed());
    }

    /// @notice Transfer ownership of GaugeController to `addr`
    /// @param addr Address to have ownership transferred to
    #[inline(always)]
    fn commit_transfer_ownership(&self, addr: Key) {
        if self.get_caller() != get_admin() {
            runtime::revert(ApiError::from(Error::GaugeWrapperAdminOnly2));
        }
        set_future_admin(addr);
        LIQUIDITYGAUGEWRAPPER::emit(
            self,
            &LiquidityGaugeWrapperEvent::CommitOwnership { admin: addr },
        );
    }

    /// @notice Apply pending ownership transfer
    #[inline(always)]
    fn apply_transfer_ownership(&self) {
        if self.get_caller() != get_admin() {
            runtime::revert(ApiError::from(Error::GaugeWrapperAdminOnly3));
        }
        let admin: Key = get_future_admin();
        if admin == zero_address() {
            runtime::revert(ApiError::from(Error::GaugeWrapperAdminNotSet));
        }
        set_admin(admin);
        LIQUIDITYGAUGEWRAPPER::emit(self, &LiquidityGaugeWrapperEvent::ApplyOwnership { admin });
    }

    fn emit(&self, liquidity_gauge_wrapper_event: &LiquidityGaugeWrapperEvent) {
        match liquidity_gauge_wrapper_event {
            LiquidityGaugeWrapperEvent::Deposit { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_wrapper_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeWrapperEvent::Withdraw { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_wrapper_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeWrapperEvent::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_wrapper_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeWrapperEvent::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_wrapper_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeWrapperEvent::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_wrapper_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeWrapperEvent::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_wrapper_event.type_name());
                event.insert("from", owner.to_string());
                event.insert("to", spender.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
        };
    }
}
