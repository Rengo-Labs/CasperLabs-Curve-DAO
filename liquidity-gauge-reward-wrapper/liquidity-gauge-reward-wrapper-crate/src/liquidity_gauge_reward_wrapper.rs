use crate::{data::*, event::LiquidityGaugeRewardWrapperEvent};
use alloc::string::String;
use alloc::{collections::BTreeMap, string::ToString};
use casper_contract::{
    contract_api::{
        runtime::{self},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs, U256,
};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::{errors::*, utils::*};
use crv20::{self, Address, CURVEERC20};
use curve_casper_erc20::Error as Erc20Error;

pub trait LIQUIDITYGAUGEREWARDWRAPPER<Storage: ContractStorage>:
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
        self.set_name(name);
        self.set_symbol(symbol);
        // self.set_decimals(9);
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
        let rewarded_token: Key = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "rewarded_token",
            runtime_args! {},
        );
        set_rewarded_token(rewarded_token);
        set_lp_token(lp_token);
        set_gauge(gauge);
        set_admin(admin);
        RewardIntegralFor::init();
        CrvIntegralFor::init();
        ClaimableCrv::init();
        ClaimableRewards::init();
        ApprovedToDeposit::init();
        set_contract_hash(contract_hash);
        set_package_hash(package_hash);
    }
    #[inline(always)]
    fn _checkpoint(&self, addr: Key) {
        let gauge: Key = get_gauge();
        let mut token: Key = get_crv_token();
        let total_balance: U256 = self.total_supply();
        let mut d_reward: U256 = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "address" => Key::from(get_package_hash())
            },
        );
        let () = runtime::call_versioned_contract(
            get_minter().into_hash().unwrap_or_revert().into(),
            None,
            "mint",
            runtime_args! {
                "gauge_addr" => gauge
            },
        );
        let mut d_reward_updated: U256 = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "address" =>Key::from(get_package_hash())
            },
        );
        d_reward = d_reward_updated
            .checked_sub(d_reward)
            .unwrap_or_revert_with(Error::RewardWrapperSubtractionError1);
        let mut di: U256 = 0.into();
        if total_balance > 0.into() {
            di = U256::from(TEN_E_NINE)
                .checked_mul(d_reward)
                .unwrap_or_revert_with(Error::RewardWrapperMultiplyError1)
                .checked_div(total_balance)
                .unwrap_or_revert_with(Error::RewardWrapperDivisionError1);
        }
        let mut i: U256 = get_crv_integral()
            .checked_add(di)
            .unwrap_or_revert_with(Error::RewardWrapperAdditionError1);
        set_crv_integral(i);
        let balance_of: U256 = self.balance_of(Address::from(addr));
        let crv_integral_for: U256 = CrvIntegralFor::instance().get(&addr);
        ClaimableCrv::instance().set(
            &addr,
            ClaimableCrv::instance()
                .get(&addr)
                .checked_add(balance_of)
                .unwrap_or_revert_with(Error::RewardWrapperAdditionError2)
                .checked_mul(
                    i.checked_sub(crv_integral_for)
                        .unwrap_or_revert_with(Error::RewardWrapperSubtractionError2),
                )
                .unwrap_or_revert_with(Error::RewardWrapperMultiplyError2)
                .checked_div(U256::from(TEN_E_NINE))
                .unwrap_or_revert_with(Error::RewardWrapperDivisionError2),
        );
        CrvIntegralFor::instance().set(&addr, i);
        token = get_rewarded_token();
        d_reward = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "address" => Key::from(get_package_hash()),
            },
        );
        let () = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "claim_rewards",
            runtime_args! {
                "addr" => None::<Key>
            },
        );
        d_reward_updated = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "address" =>Key::from(get_package_hash())
            },
        );
        d_reward = d_reward_updated
            .checked_sub(d_reward)
            .unwrap_or_revert_with(Error::RewardWrapperSubtractionError3);
        if total_balance > 0.into() {
            di = U256::from(TEN_E_NINE)
                .checked_mul(d_reward)
                .unwrap_or_revert_with(Error::RewardWrapperMultiplyError3)
                .checked_div(total_balance)
                .unwrap_or_revert_with(Error::RewardWrapperDivisionError3);
        }
        i = get_reward_integral()
            .checked_add(di)
            .unwrap_or_revert_with(Error::RewardWrapperAdditionError3);
        set_reward_integral(i);
        let reward_integral_for: U256 = CrvIntegralFor::instance().get(&addr);
        ClaimableRewards::instance().set(
            &addr,
            ClaimableRewards::instance()
                .get(&addr)
                .checked_add(balance_of)
                .unwrap_or_revert_with(Error::RewardWrapperAdditionError4)
                .checked_mul(
                    i.checked_sub(reward_integral_for)
                        .unwrap_or_revert_with(Error::RewardWrapperSubtractionError4),
                )
                .unwrap_or_revert_with(Error::RewardWrapperMultiplyError4)
                .checked_div(U256::from(TEN_E_NINE))
                .unwrap_or_revert_with(Error::RewardWrapperDivisionError4),
        );
        RewardIntegralFor::instance().set(&addr, i);
    }
    /// @notice Record a checkpoint for `addr`
    /// @param addr User address
    /// @return bool success
    #[inline(always)]
    fn user_checkpoint(&self, addr: Key) -> bool {
        if !((self.get_caller() == addr) || (self.get_caller() == get_minter())) {
            runtime::revert(ApiError::from(Error::RewardWrapperUnauthorized));
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
                .unwrap_or_revert_with(Error::RewardWrapperMultiplyError5)
                .checked_div(total_balance)
                .unwrap_or_revert_with(Error::RewardWrapperDivisionError5);
        }
        let i: U256 = get_crv_integral()
            .checked_add(di)
            .unwrap_or_revert_with(Error::RewardWrapperAdditionError5);
        let balance_of: U256 = self.balance_of(Address::from(addr));
        let crv_integral_for: U256 = CrvIntegralFor::instance().get(&addr);
        let claimable_crv: U256 = ClaimableCrv::instance().get(&addr);
        claimable_crv
            .checked_add(balance_of)
            .unwrap_or_revert_with(Error::RewardWrapperAdditionError6)
            .checked_mul(
                i.checked_sub(crv_integral_for)
                    .unwrap_or_revert_with(Error::RewardWrapperSubtractionError5),
            )
            .unwrap_or_revert_with(Error::RewardWrapperMultiplyError6)
            .checked_div(U256::from(TEN_E_NINE))
            .unwrap_or_revert_with(Error::RewardWrapperDivisionError6)
    }

    /// @notice Get the number of claimable reward tokens per user
    /// @dev This function should be manually changed to "view" in the ABI
    /// @return uint256 number of claimable tokens per user
    #[inline(always)]
    fn claimable_reward(&self, addr: Key) -> U256 {
        let gauge: Key = get_gauge();
        let claimable_reward: U256 = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "claimable_reward",
            runtime_args! {
                "addr" => Key::from(get_package_hash())
            },
        );
        let claimed_rewards_for: U256 = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "claimed_rewards_for",
            runtime_args! {
                "owner" => Key::from(get_package_hash())
            },
        );
        let d_reward: U256 = claimable_reward
            .checked_sub(claimed_rewards_for)
            .unwrap_or_revert_with(Error::RewardWrapperSubtractionError6);
        let total_balance: U256 = self.total_supply();
        let mut di: U256 = 0.into();
        if total_balance > 0.into() {
            di = U256::from(TEN_E_NINE)
                .checked_mul(d_reward)
                .unwrap_or_revert_with(Error::RewardWrapperMultiplyError7)
                .checked_div(total_balance)
                .unwrap_or_revert_with(Error::RewardWrapperDivisionError7);
        }
        let i: U256 = get_reward_integral()
            .checked_add(di)
            .unwrap_or_revert_with(Error::RewardWrapperAdditionError7);
        let balance_of: U256 = self.balance_of(Address::from(addr));
        let reward_integral_for: U256 = RewardIntegralFor::instance().get(&addr);
        let claimable_rewards: U256 = ClaimableRewards::instance().get(&addr);
        claimable_rewards
            .checked_add(balance_of)
            .unwrap_or_revert_with(Error::RewardWrapperAdditionError8)
            .checked_mul(
                i.checked_sub(reward_integral_for)
                    .unwrap_or_revert_with(Error::RewardWrapperSubtractionError7),
            )
            .unwrap_or_revert_with(Error::RewardWrapperMultiplyError8)
            .checked_div(U256::from(TEN_E_NINE))
            .unwrap_or_revert_with(Error::RewardWrapperDivisionError8)
    }
    #[inline(always)]
    fn claim_tokens(&self, addr: Option<Key>) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::RewardWrapperIsLocked1));
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
        let () = runtime::call_versioned_contract(
            get_rewarded_token().into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => addr,
                "amount" => ClaimableRewards::instance().get(&addr)
            },
        );
        ClaimableCrv::instance().set(&addr, 0.into());
        ClaimableRewards::instance().set(&addr, 0.into());
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
            runtime::revert(ApiError::from(Error::RewardWrapperIsLocked2));
        }
        set_lock(true);
        let addr: Key = match addr {
            Some(val) => val,
            None => self.get_caller(),
        };
        if get_is_killed() {
            runtime::revert(ApiError::from(Error::RewardWrapperIsKilled1));
        }
        if addr != self.get_caller()
            && !(ApprovedToDeposit::instance().get(&self.get_caller(), &addr))
        {
            runtime::revert(ApiError::from(Error::RewardWrapperNotApproved));
        }
        self._checkpoint(addr);
        if value != 0.into() {
            let balance: U256 = self
                .balance_of(Address::from(addr))
                .checked_add(value)
                .unwrap_or_revert_with(Error::RewardWrapperAdditionError9);
            let supply: U256 = self
                .total_supply()
                .checked_add(value)
                .unwrap_or_revert_with(Error::RewardWrapperAdditionError10);
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

                },
            );
        }
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::Deposit {
                provider: addr,
                value,
            },
        );
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::Transfer {
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
            runtime::revert(ApiError::from(Error::RewardWrapperIsLocked3));
        }
        set_lock(true);
        self._checkpoint(self.get_caller());
        if value != 0.into() {
            let balance: U256 = self
                .balance_of(Address::from(self.get_caller()))
                .checked_sub(value)
                .unwrap_or_revert_with(Error::RewardWrapperSubtractionError8);
            let supply: U256 = self
                .total_supply()
                .checked_sub(value)
                .unwrap_or_revert_with(Error::RewardWrapperSubtractionError9);
            self.set_balance(Address::from(self.get_caller()), balance);
            self.set_total_supply(supply);
            let () = runtime::call_versioned_contract(
                get_gauge().into_hash().unwrap_or_revert().into(),
                None,
                "withdraw",
                runtime_args! {
                    "value" => value,
                    "claim_rewards" => true
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
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::Withdraw {
                provider: self.get_caller(),
                value,
            },
        );
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::Transfer {
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
    fn allowance(&self, owner: Key, spender: Key) -> U256 {
        // Allowances::instance().get(&owner, &spender)
        CURVEERC20::allowance(self, Address::from(owner), Address::from(spender))
    }
    #[inline(always)]
    fn _transfer(&self, owner: Key, recipient: Key, amount: U256) {
        if get_is_killed() {
            runtime::revert(ApiError::from(Error::RewardWrapperIsKilled2));
        }
        self._checkpoint(owner);
        self._checkpoint(recipient);
        if amount != 0.into() {
            let balance_owner: U256 = self
                .balance_of(Address::from(owner))
                .checked_sub(amount)
                .unwrap_or_revert_with(Error::RewardWrapperSubtractionError10);
            self.set_balance(Address::from(owner), balance_owner);
            let balance_recipient: U256 = self
                .balance_of(Address::from(recipient))
                .checked_add(amount)
                .unwrap_or_revert_with(Error::RewardWrapperSubtractionError11);

            self.set_balance(Address::from(recipient), balance_recipient);
        }
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::Transfer {
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
    fn transfer(&mut self, recipient: Address, amount: U256) -> Result<(), Erc20Error> {
        self._transfer(self.get_caller(), Key::from(recipient), amount);
        Ok(())
    }
    //     /// @dev Transfer tokens from one address to another.
    //     /// @param _from address The address which you want to send tokens from
    //     /// @param _to address The address which you want to transfer to
    //     /// @param _value uint256 the amount of tokens to be transferred
    #[inline(always)]
    fn transfer_from(
        &mut self,
        owner: Address,
        recipient: Address,
        amount: U256,
    ) -> Result<(), Erc20Error> {
        let allowance: U256 = CURVEERC20::allowance(self, owner, Address::from(self.get_caller()));
        if allowance != U256::MAX {
            self.set_allowance(
                owner,
                Address::from(self.get_caller()),
                allowance
                    .checked_sub(amount)
                    .unwrap_or_revert_with(Error::RewardWrapperSubtractionError12),
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
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::Approval {
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
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::Approval {
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
            runtime::revert(ApiError::from(Error::RewardWrapperAdminOnly1));
        }
        set_is_killed(!get_is_killed());
    }

    /// @notice Transfer ownership of GaugeController to `addr`
    /// @param addr Address to have ownership transferred to
    #[inline(always)]
    fn commit_transfer_ownership(&self, addr: Key) {
        if self.get_caller() != get_admin() {
            runtime::revert(ApiError::from(Error::RewardWrapperAdminOnly2));
        }
        set_future_admin(addr);
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::CommitOwnership { admin: addr },
        );
    }

    /// @notice Apply pending ownership transfer
    #[inline(always)]
    fn apply_transfer_ownership(&self) {
        if self.get_caller() != get_admin() {
            runtime::revert(ApiError::from(Error::RewardWrapperAdminOnly3));
        }
        let admin: Key = get_future_admin();
        if admin == zero_address() {
            runtime::revert(ApiError::from(Error::RewardWrapperAdminNotSet));
        }
        set_admin(admin);
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::ApplyOwnership { admin },
        );
    }

    fn emit(&self, liquidity_gauge_reward_wrapper_event: &LiquidityGaugeRewardWrapperEvent) {
        match liquidity_gauge_reward_wrapper_event {
            LiquidityGaugeRewardWrapperEvent::Deposit { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert(
                    "event_type",
                    liquidity_gauge_reward_wrapper_event.type_name(),
                );
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeRewardWrapperEvent::Withdraw { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert(
                    "event_type",
                    liquidity_gauge_reward_wrapper_event.type_name(),
                );
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeRewardWrapperEvent::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert(
                    "event_type",
                    liquidity_gauge_reward_wrapper_event.type_name(),
                );
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeRewardWrapperEvent::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert(
                    "event_type",
                    liquidity_gauge_reward_wrapper_event.type_name(),
                );
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeRewardWrapperEvent::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert(
                    "event_type",
                    liquidity_gauge_reward_wrapper_event.type_name(),
                );
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeRewardWrapperEvent::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert(
                    "event_type",
                    liquidity_gauge_reward_wrapper_event.type_name(),
                );
                event.insert("from", owner.to_string());
                event.insert("to", spender.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
        };
    }
}
