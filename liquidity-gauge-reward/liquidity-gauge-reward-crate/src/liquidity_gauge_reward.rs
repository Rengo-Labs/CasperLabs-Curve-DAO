use crate::{data::*, event::LiquidityGaugeRewardEvent};
use alloc::{collections::BTreeMap, string::ToString};
use casper_contract::{
    contract_api::{
        runtime::{self, get_blocktime},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs, U128, U256,
};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::{errors::*, utils::*};
use crv20::{self, get_package_hash, Address, CURVEERC20};

#[allow(clippy::too_many_arguments)]
pub trait LIQUIDITYGAUGEREWARD<Storage: ContractStorage>:
    ContractContext<Storage> + CURVEERC20<Storage>
{
    /// @notice Contract constructor
    /// @param lp_addr Liquidity Pool contract address
    /// @param _minter Minter contract address
    /// @param _reward_contract Synthetix reward contract address
    /// @param _rewarded_token Received synthetix token contract address
    /// @param _admin Admin who can kill the gauge
    fn init(
        &self,
        lp_addr: Key,
        minter: Key,
        reward_contract: Key,
        rewarded_token: Key,
        admin: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        if lp_addr == zero_address() {
            runtime::revert(ApiError::from(Error::LiquidityGaugeRewardZeroAddress1));
        }
        if minter == zero_address() {
            runtime::revert(ApiError::from(Error::LiquidityGaugeRewardZeroAddress2));
        }
        if reward_contract == zero_address() {
            runtime::revert(ApiError::from(Error::LiquidityGaugeRewardZeroAddress3));
        }

        CURVEERC20::init(self, contract_hash, package_hash);

        ApprovedToDeposit::init();
        WorkingBalances::init();
        PeriodTimestamp::init();
        IntegrateInvSupply::init();
        IntegrateInvSupplyOf::init();
        IntegrateCheckpointOf::init();
        IntegrateFraction::init();
        RewardIntegralFor::init();
        RewardsFor::init();
        ClaimedRewardsFor::init();

        set_lp_token(lp_addr);
        set_minter(minter);
        let crv_addr: Key = runtime::call_versioned_contract(
            minter.into_hash().unwrap_or_revert().into(),
            None,
            "token",
            runtime_args! {},
        );
        set_crv_token(crv_addr);
        let controller_addr: Key = runtime::call_versioned_contract(
            minter.into_hash().unwrap_or_revert().into(),
            None,
            "controller",
            runtime_args! {},
        );
        set_controller(controller_addr);
        set_voting_escrow(runtime::call_versioned_contract(
            controller_addr.into_hash().unwrap_or_revert().into(),
            None,
            "voting_escrow",
            runtime_args! {},
        ));
        PeriodTimestamp::instance().set(&0.into(), U256::from(u64::from(get_blocktime())));
        set_inflation_rate(runtime::call_versioned_contract(
            crv_addr.into_hash().unwrap_or_revert().into(),
            None,
            "rate",
            runtime_args! {},
        ));
        set_future_epoch_time(runtime::call_versioned_contract(
            crv_addr.into_hash().unwrap_or_revert().into(),
            None,
            "future_epoch_time_write",
            runtime_args! {},
        ));
        set_reward_contract(reward_contract);
        let () = runtime::call_versioned_contract(
            lp_addr.into_hash().unwrap_or_revert().into(),
            None,
            "approve",
            runtime_args! {
                "spender" => reward_contract,
                "amount" => U256::MAX
            },
        );
        set_rewarded_token(rewarded_token);
        set_admin(admin);
        set_is_claiming_rewards(true);
    }

    /// @notice Calculate limits which depend on the amount of CRV token per-user. Effectively it calculates working balances to apply amplification of CRV production by CRV
    /// @param addr User address
    /// @param l User's amount of liquidity (LP tokens)
    /// @param L Total amount of liquidity (LP tokens)
    #[allow(non_snake_case)]
    fn _update_liquidity_limit(&self, addr: Key, l: U256, L: U256) {
        // To be called after totalSupply is updated
        let voting_escrow: Key = get_voting_escrow();
        let voting_balance: U256 = runtime::call_versioned_contract(
            voting_escrow.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "addr" => addr,
                "t" => None::<U256>
            },
        );
        let voting_total: U256 = runtime::call_versioned_contract(
            voting_escrow.into_hash().unwrap_or_revert().into(),
            None,
            "total_supply",
            runtime_args! {
                "t" => None::<U256>
            },
        );
        let mut lim: U256 = l
            .checked_mul(TOKENLESS_PRODUCTION)
            .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError3)
            .checked_div(100.into())
            .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError4);
        if (voting_total > 0.into())
            && (U256::from(u64::from(get_blocktime()))
                > PeriodTimestamp::instance()
                    .get(&0.into())
                    .checked_add(BOOST_WARMUP)
                    .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError5))
        {
            lim = lim
                .checked_add(
                    L.checked_mul(voting_balance)
                        .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError6)
                        .checked_div(voting_total)
                        .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError7)
                        .checked_mul(
                            U256::from(100)
                                .checked_sub(TOKENLESS_PRODUCTION)
                                .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError8),
                        )
                        .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError9)
                        .checked_div(100.into())
                        .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError10),
                )
                .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError11);
        }
        lim = U256::min(l, lim);
        let old_bal: U256 = WorkingBalances::instance().get(&addr);
        WorkingBalances::instance().set(&addr, lim);
        let working_supply: U256 = get_working_supply()
            .checked_add(lim)
            .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError12)
            .checked_sub(old_bal)
            .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError13);
        set_working_supply(working_supply);
        LIQUIDITYGAUGEREWARD::emit(
            self,
            &LiquidityGaugeRewardEvent::UpdateLiquidityLimit {
                user: addr,
                original_balance: l,
                original_supply: L,
                working_balance: lim,
                working_supply,
            },
        );
    }

    #[allow(non_snake_case)]
    fn _checkpoint_rewards(&self, addr: Key, claim_rewards: bool) {
        // Update reward integrals (no gauge weights involved: easy)
        let rewarded_token: Key = get_rewarded_token();
        let mut d_reward: U256 = 0.into();
        if claim_rewards {
            d_reward = runtime::call_versioned_contract(
                rewarded_token.into_hash().unwrap_or_revert().into(),
                None,
                "balance_of",
                runtime_args! {
                    "address" => Key::from(get_package_hash())
                },
            );
            let () = runtime::call_versioned_contract(
                get_reward_contract().into_hash().unwrap_or_revert().into(),
                None,
                "get_reward",
                runtime_args! {},
            );
            let _d_reward: U256 = runtime::call_versioned_contract(
                rewarded_token.into_hash().unwrap_or_revert().into(),
                None,
                "balance_of",
                runtime_args! {
                    "address" => Key::from(get_package_hash())
                },
            );
            d_reward = _d_reward
                .checked_sub(d_reward)
                .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError14);
        }

        let user_balance: U256 = self.balance_of(Address::from(addr));
        let total_balance: U256 = self.total_supply();
        let mut dI: U256 = 0.into();
        if total_balance > 0.into() {
            dI = U256::from(10)
                .pow(9.into())
                .checked_mul(d_reward)
                .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError15)
                .checked_div(total_balance)
                .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError16);
        }
        let I: U256 = get_reward_integral()
            .checked_add(dI)
            .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError17);
        set_reward_integral(I);
        RewardsFor::instance().set(
            &addr,
            RewardsFor::instance()
                .get(&addr)
                .checked_add(
                    user_balance
                        .checked_mul(
                            I.checked_sub(RewardIntegralFor::instance().get(&addr))
                                .unwrap_or_revert_with(
                                    Error::LiquidityGaugeRewardArithmaticError18,
                                ),
                        )
                        .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError19)
                        .checked_div(10.into())
                        .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError20)
                        .pow(9.into()),
                )
                .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError21),
        );
        RewardIntegralFor::instance().set(&addr, I);
    }

    /// @notice Checkpoint for a user
    /// @param addr User address
    fn _checkpoint(&self, addr: Key, claim_rewards: bool) {
        let token: Key = get_crv_token();
        let controller: Key = get_controller();
        let mut period: U128 = get_period();
        let period_time: U256 = PeriodTimestamp::instance().get(&period.as_u128().into());
        let mut integrate_inv_supply: U256 =
            IntegrateInvSupply::instance().get(&period.as_u128().into());
        let mut rate: U256 = get_inflation_rate();
        let mut new_rate: U256 = rate;
        let prev_future_epoch: U256 = get_future_epoch_time();
        if prev_future_epoch >= period_time {
            set_future_epoch_time(runtime::call_versioned_contract(
                token.into_hash().unwrap_or_revert().into(),
                None,
                "future_epoch_time_write",
                runtime_args! {},
            ));
            new_rate = runtime::call_versioned_contract(
                token.into_hash().unwrap_or_revert().into(),
                None,
                "rate",
                runtime_args! {},
            );
            set_inflation_rate(new_rate);
        }
        let () = runtime::call_versioned_contract(
            controller.into_hash().unwrap_or_revert().into(),
            None,
            "checkpoint_gauge",
            runtime_args! {
                "addr" => Key::from(get_package_hash())
            },
        );
        let working_balance: U256 = WorkingBalances::instance().get(&addr);
        let working_supply: U256 = get_working_supply();
        if get_is_killed() {
            rate = 0.into(); // Stop distributing inflation as soon as killed
        }
        // Update integral of 1/supply
        if U256::from(u64::from(get_blocktime())) > period_time {
            let mut prev_week_time: U256 = period_time;
            let mut week_time: U256 = U256::min(
                (period_time
                    .checked_add(WEEK)
                    .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError22))
                .checked_div(WEEK)
                .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError23)
                .checked_mul(WEEK)
                .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError24),
                U256::from(u64::from(get_blocktime())),
            );
            for _ in 0..500 {
                let dt: U256 = week_time
                    .checked_sub(prev_week_time)
                    .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError25);
                let w: U256 = runtime::call_versioned_contract(
                    controller.into_hash().unwrap_or_revert().into(),
                    None,
                    "gauge_relative_weight",
                    runtime_args! {
                        "addr" => Key::from(get_package_hash()),
                        "time" => prev_week_time.checked_div(WEEK).unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError26).checked_mul(WEEK).unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError27)
                    },
                );
                if working_supply > 0.into() {
                    if prev_future_epoch >= prev_week_time && prev_future_epoch < week_time {
                        // If we went across one or multiple epochs, apply the rate
                        // of the first epoch until it ends, and then the rate of
                        // the last epoch.
                        // If more than one epoch is crossed - the gauge gets less,
                        // but that'd meen it wasn't called for more than 1 year
                        integrate_inv_supply = integrate_inv_supply
                            .checked_add(
                                rate.checked_mul(w)
                                    .unwrap_or_revert_with(
                                        Error::LiquidityGaugeRewardArithmaticError28,
                                    )
                                    .checked_mul(
                                        prev_future_epoch
                                            .checked_sub(prev_week_time)
                                            .unwrap_or_revert_with(
                                                Error::LiquidityGaugeRewardArithmaticError29,
                                            ),
                                    )
                                    .unwrap_or_revert_with(
                                        Error::LiquidityGaugeRewardArithmaticError30,
                                    )
                                    .checked_div(working_supply)
                                    .unwrap_or_revert_with(
                                        Error::LiquidityGaugeRewardArithmaticError31,
                                    ),
                            )
                            .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError32);
                        rate = new_rate;
                        integrate_inv_supply = integrate_inv_supply
                            .checked_add(
                                rate.checked_mul(w)
                                    .unwrap_or_revert_with(
                                        Error::LiquidityGaugeRewardArithmaticError33,
                                    )
                                    .checked_mul(
                                        week_time
                                            .checked_sub(prev_future_epoch)
                                            .unwrap_or_revert_with(
                                                Error::LiquidityGaugeRewardArithmaticError34,
                                            ),
                                    )
                                    .unwrap_or_revert_with(
                                        Error::LiquidityGaugeRewardArithmaticError35,
                                    )
                                    .checked_div(working_supply)
                                    .unwrap_or_revert_with(
                                        Error::LiquidityGaugeRewardArithmaticError36,
                                    ),
                            )
                            .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError37);
                    } else {
                        integrate_inv_supply = integrate_inv_supply
                            .checked_add(
                                rate.checked_mul(w)
                                    .unwrap_or_revert_with(
                                        Error::LiquidityGaugeRewardArithmaticError38,
                                    )
                                    .checked_mul(dt)
                                    .unwrap_or_revert_with(
                                        Error::LiquidityGaugeRewardArithmaticError39,
                                    )
                                    .checked_div(working_supply)
                                    .unwrap_or_revert_with(
                                        Error::LiquidityGaugeRewardArithmaticError40,
                                    ),
                            )
                            .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError41);
                    }
                    // On precisions of the calculation
                    //  rate ~= 10e18
                    //  last_weight > 0.01 * 1e9 = 1e16 (if pool weight is 1%)
                    //  _working_supply ~= TVL * 1e9 ~= 1e26 ($100M for example)
                    //  The largest loss is at dt = 1
                    //  Loss is 1e-9 - acceptable
                }
                if week_time == U256::from(u64::from(get_blocktime())) {
                    break;
                }
                prev_week_time = week_time;
                week_time = U256::min(
                    week_time
                        .checked_add(WEEK)
                        .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError42),
                    U256::from(u64::from(get_blocktime())),
                )
            }
        }
        period = period
            .checked_add(1.into())
            .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError43);
        set_period(period);
        PeriodTimestamp::instance().set(
            &period.as_u128().into(),
            U256::from(u64::from(get_blocktime())),
        );
        IntegrateInvSupply::instance().set(&period.as_u128().into(), integrate_inv_supply);
        // Update user-specific integrals
        IntegrateFraction::instance().set(
            &addr,
            IntegrateFraction::instance()
                .get(&addr)
                .checked_add(
                    working_balance
                        .checked_mul(
                            integrate_inv_supply
                                .checked_sub(IntegrateInvSupplyOf::instance().get(&addr))
                                .unwrap_or_revert_with(
                                    Error::LiquidityGaugeRewardArithmaticError44,
                                ),
                        )
                        .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError45)
                        .checked_div(10.into())
                        .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError46)
                        .pow(9.into()),
                )
                .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError47),
        );
        IntegrateInvSupplyOf::instance().set(&addr, integrate_inv_supply);
        IntegrateCheckpointOf::instance().set(&addr, U256::from(u64::from(get_blocktime())));
        self._checkpoint_rewards(addr, claim_rewards);
    }

    /// @notice Record a checkpoint for `addr`
    /// @param addr User address
    /// @return bool success
    fn user_checkpoint(&self, addr: Key) -> bool {
        if !((self.get_caller() == addr) || (self.get_caller() == get_minter())) {
            runtime::revert(ApiError::from(Error::LiquidityGaugeRewardUnauthorized));
        }
        self._checkpoint(addr, get_is_claiming_rewards());
        self._update_liquidity_limit(
            addr,
            self.balance_of(Address::from(addr)),
            self.total_supply(),
        );
        true
    }

    /// @notice Get the number of claimable tokens per user
    /// @return uint256 number of claimable tokens per user
    fn claimable_tokens(&self, addr: Key) -> U256 {
        self._checkpoint(addr, true);
        IntegrateFraction::instance()
            .get(&addr)
            .checked_sub(runtime::call_versioned_contract(
                get_minter().into_hash().unwrap_or_revert().into(),
                None,
                "minted",
                runtime_args! {
                    "owner" => addr,
                    "spender" => Key::from(get_package_hash())
                },
            ))
            .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError48)
    }

    /// @notice Get the number of claimable reward tokens for a user
    /// @param addr Account to get reward amount for
    /// @return uint256 Claimable reward token amount
    #[allow(non_snake_case)]
    fn claimable_reward(&self, addr: Key) -> U256 {
        let d_reward: U256 = runtime::call_versioned_contract(
            get_reward_contract().into_hash().unwrap_or_revert().into(),
            None,
            "earned",
            runtime_args! {
                "account" => Key::from(get_package_hash())
            },
        );
        let user_balance: U256 = self.balance_of(Address::from(addr));
        let total_balance: U256 = self.total_supply();
        let mut dI: U256 = 0.into();
        if total_balance > 0.into() {
            dI = U256::from(10)
                .pow(9.into())
                .checked_mul(d_reward)
                .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError49)
                .checked_div(total_balance)
                .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError50);
        }
        let I: U256 = get_reward_integral()
            .checked_add(dI)
            .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError51);
        RewardsFor::instance()
            .get(&addr)
            .checked_add(
                user_balance
                    .checked_mul(
                        I.checked_sub(RewardIntegralFor::instance().get(&addr))
                            .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError52),
                    )
                    .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError53),
            )
            .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError54)
            .checked_div(10.into())
            .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError55)
            .pow(9.into())
    }

    /// @notice Kick `addr` for abusing their boost
    /// @dev Only if either they had another voting event, or their voting escrow lock expired
    /// @param addr Address to kick
    fn kick(&self, addr: Key) {
        let t_last: U256 = IntegrateCheckpointOf::instance().get(&addr);
        let ret: U256 = runtime::call_versioned_contract(
            get_voting_escrow().into_hash().unwrap_or_revert().into(),
            None,
            "user_point_epoch",
            runtime_args! {
                "user" => addr
            },
        );
        let t_ve: U256 = runtime::call_versioned_contract(
            get_voting_escrow().into_hash().unwrap_or_revert().into(),
            None,
            "user_point_history_ts",
            runtime_args! {
                "addr" => addr,
                "idx" => ret
            },
        );
        let balance: U256 = self.balance_of(Address::from(addr));
        let ret: U256 = runtime::call_versioned_contract(
            get_voting_escrow().into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "addr" => addr,
                "t" => None::<U256>
            },
        );
        if !(ret == 0.into() || t_ve > t_last) {
            runtime::revert(ApiError::from(Error::LiquidityGaugeRewardKickNotAllowed1));
        }
        if WorkingBalances::instance().get(&addr)
            <= balance
                .checked_mul(TOKENLESS_PRODUCTION)
                .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError56)
                .checked_div(100.into())
                .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError57)
        {
            runtime::revert(ApiError::from(Error::LiquidityGaugeRewardKickNotNeeded2));
        }
        self._checkpoint(addr, get_is_claiming_rewards());
        self._update_liquidity_limit(
            addr,
            self.balance_of(Address::from(addr)),
            self.total_supply(),
        );
    }

    /// @notice Set whether `addr` can deposit tokens for `self.get_caller()`
    /// @param addr Address to set approval on
    /// @param can_deposit bool - can this account deposit for `self.get_caller()`?
    fn set_approve_deposit(&self, addr: Key, can_deposit: bool) {
        ApprovedToDeposit::instance().set(&addr, &self.get_caller(), can_deposit);
    }

    /// @notice Deposit `_value` LP tokens
    /// @param _value Number of tokens to deposit
    /// @param addr Address to deposit for
    fn deposit(&self, value: U256, addr: Option<Key>) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::LiquidityGaugeRewardIsLocked1));
        }
        set_lock(true);
        let addr: Key = match addr {
            Some(val) => val,
            None => self.get_caller(),
        };

        if addr != self.get_caller()
            && !(ApprovedToDeposit::instance().get(&self.get_caller(), &addr))
        {
            runtime::revert(ApiError::from(Error::LiquidityGaugeRewardNotApproved));
        }
        self._checkpoint(addr, true);
        if value != 0.into() {
            let balance: U256 = self
                .balance_of(Address::from(addr))
                .checked_add(value)
                .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError58);
            let supply: U256 = self
                .total_supply()
                .checked_add(value)
                .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError59);
            self.set_balance(Address::from(addr), balance);
            self.set_total_supply(supply);
            self._update_liquidity_limit(addr, balance, supply);
            let () = runtime::call_versioned_contract(
                get_lp_token().into_hash().unwrap_or_revert().into(),
                None,
                "transfer_from",
                runtime_args! {
                    "owner" => Address::from(self.get_caller()),
                    "recipient" => Address::Contract(get_package_hash()),
                    "amount" => value
                },
            );

            let () = runtime::call_versioned_contract(
                get_reward_contract().into_hash().unwrap_or_revert().into(),
                None,
                "stake",
                runtime_args! {
                    "amount" => value
                },
            );
        }
        LIQUIDITYGAUGEREWARD::emit(
            self,
            &LiquidityGaugeRewardEvent::Deposit {
                provider: addr,
                value,
            },
        );
        set_lock(false);
    }

    /// @notice Withdraw `_value` LP tokens
    /// @param _value Number of tokens to withdraw
    fn withdraw(&self, value: U256, claim_rewards: bool) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::LiquidityGaugeRewardIsLocked2));
        }
        set_lock(true);
        self._checkpoint(self.get_caller(), claim_rewards);
        let balance: U256 = self
            .balance_of(Address::from(self.get_caller()))
            .checked_sub(value)
            .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError60);
        let supply: U256 = self
            .total_supply()
            .checked_sub(value)
            .unwrap_or_revert_with(Error::LiquidityGaugeRewardArithmaticError61);
        self.set_balance(Address::from(self.get_caller()), balance);
        self.set_total_supply(supply);
        self._update_liquidity_limit(self.get_caller(), balance, supply);
        if value > 0.into() {
            let () = runtime::call_versioned_contract(
                get_reward_contract().into_hash().unwrap_or_revert().into(),
                None,
                "withdraw",
                runtime_args! {
                    "amount" => value
                },
            );
            let () = runtime::call_versioned_contract(
                get_lp_token().into_hash().unwrap_or_revert().into(),
                None,
                "transfer",
                runtime_args! {
                    "recipient" => Address::from(self.get_caller()),
                    "amount" => value
                },
            );
        }
        LIQUIDITYGAUGEREWARD::emit(
            self,
            &LiquidityGaugeRewardEvent::Withdraw {
                provider: self.get_caller(),
                value,
            },
        );
        set_lock(false);
    }

    fn claim_rewards(&self, addr: Option<Key>) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::LiquidityGaugeRewardIsLocked3));
        }
        set_lock(true);
        let addr: Key = match addr {
            Some(val) => val,
            None => self.get_caller(),
        };

        self._checkpoint_rewards(addr, true);
        let rewards_for: U256 = RewardsFor::instance().get(&addr);
        let () = runtime::call_versioned_contract(
            get_rewarded_token().into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => addr,
                "amount" => rewards_for.checked_sub(ClaimedRewardsFor::instance().get(&addr)).unwrap_or_revert()
            },
        );
        ClaimedRewardsFor::instance().set(&addr, rewards_for);
        set_lock(false);
    }

    fn integrate_checkpoint(&self) -> U256 {
        PeriodTimestamp::instance().get(&get_period().as_u128().into())
    }

    fn kill_me(&self) {
        if self.get_caller() != get_admin() {
            runtime::revert(ApiError::from(Error::LiquidityGaugeRewardAdminOnly1));
        }
        set_is_killed(!get_is_killed());
    }

    /// @notice Transfer ownership of GaugeController to `addr`
    /// @param addr Address to have ownership transferred to
    fn commit_transfer_ownership(&self, addr: Key) {
        if self.get_caller() != get_admin() {
            runtime::revert(ApiError::from(Error::LiquidityGaugeRewardAdminOnly2));
        }
        set_future_admin(addr);
        LIQUIDITYGAUGEREWARD::emit(
            self,
            &LiquidityGaugeRewardEvent::CommitOwnership { admin: addr },
        );
    }

    /// @notice Apply pending ownership transfer
    fn apply_transfer_ownership(&self) {
        if self.get_caller() != get_admin() {
            runtime::revert(ApiError::from(Error::LiquidityGaugeRewardAdminOnly3));
        }
        let admin: Key = get_future_admin();
        if admin == zero_address() {
            runtime::revert(ApiError::from(Error::LiquidityGaugeRewardAdminNotSet));
        }
        set_admin(admin);
        LIQUIDITYGAUGEREWARD::emit(self, &LiquidityGaugeRewardEvent::ApplyOwnership { admin });
    }

    /// @notice Switch claiming rewards on/off. This is to prevent a malicious rewards contract from preventing CRV claiming
    fn toggle_external_rewards_claim(&self, val: bool) {
        if self.get_caller() != get_admin() {
            runtime::revert(ApiError::from(Error::LiquidityGaugeRewardAdminOnly4));
        }
        set_is_claiming_rewards(val);
    }

    fn emit(&self, liquidity_gauge_reward_event: &LiquidityGaugeRewardEvent) {
        match liquidity_gauge_reward_event {
            LiquidityGaugeRewardEvent::Deposit { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_reward_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeRewardEvent::Withdraw { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_reward_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeRewardEvent::UpdateLiquidityLimit {
                user,
                original_balance,
                original_supply,
                working_balance,
                working_supply,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_reward_event.type_name());
                event.insert("user", user.to_string());
                event.insert("original_balance", original_balance.to_string());
                event.insert("original_supply", original_supply.to_string());
                event.insert("working_balance", working_balance.to_string());
                event.insert("working_supply", working_supply.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeRewardEvent::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_reward_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeRewardEvent::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_reward_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
        };
    }
}
