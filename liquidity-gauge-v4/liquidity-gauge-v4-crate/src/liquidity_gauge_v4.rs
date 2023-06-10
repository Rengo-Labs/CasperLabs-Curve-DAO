use crate::data::{
    self, get_package_hash, ClaimData, ClaimDataStruct, PeriodTimestamp, RewardData, RewardDataStruct,
    RewardIntegral, RewardIntegralFor, RewardTokens, RewardsReceiver, MAX_REWARDS,
};
use crate::{alloc::string::ToString, event::*};
use alloc::{collections::BTreeMap, string::String};
use alloc::format;
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

pub trait LIQUIDITYTGAUGEV4<Storage: ContractStorage>:
    ContractContext<Storage> + CURVEERC20<Storage>
{
    fn init(
        &mut self,
        lp_token: Key,
        minter: Key,
        admin: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        data::IntegrateCheckpointOf::init();
        data::IntegrateFraction::init();
        data::IntegrateInvSupply::init();
        data::IntegrateInvSupplyOf::init();
        data::PeriodTimestamp::init();
        data::WorkingBalances::init();
        data::RewardTokens::init();
        data::RewardIntegralFor::init();
        ClaimData::init();
        RewardData::init();
        RewardsReceiver::init();
        data::set_package_hash(package_hash);
        data::set_contract_hash(contract_hash);
        CURVEERC20::init(self, data::get_contract_hash(), data::get_package_hash());

        let _lp_token_hash_add_array = match lp_token {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let _lp_token_package_hash = ContractPackageHash::new(_lp_token_hash_add_array);
        let symbol: String = runtime::call_versioned_contract(
            _lp_token_package_hash,
            None,
            "symbol",
            runtime_args! {},
        );
        let mut name: String = "Curve.fi ".to_string();
        let post_name: &str = "Gauge Deposit";
        name.push_str(symbol.as_str());
        name.push_str(post_name);
        self.set_name(name);
        self.set_symbol(symbol + "-gauge");

        let crv_token: Key = runtime::call_versioned_contract(
            minter.into_hash().unwrap_or_revert().into(),
            None,
            "token",
            runtime_args! {},
        );

        let controller_addr: Key = runtime::call_versioned_contract(
            minter.into_hash().unwrap_or_revert().into(),
            None,
            "controller",
            runtime_args! {},
        );
        data::set_lp_token(lp_token);
        data::set_minter(minter);
        data::set_admin(admin);
        data::set_crv_token(crv_token);
        data::set_controller(controller_addr);
        data::set_voting_escrow(runtime::call_versioned_contract(
            controller_addr.into_hash().unwrap_or_revert().into(),
            None,
            "voting_escrow",
            runtime_args! {},
        ));
        let block_timestamp: u64 = runtime::get_blocktime().into();
        data::PeriodTimestamp::instance().set(&U256::from(0), block_timestamp.into());
        data::set_inflation_rate(runtime::call_versioned_contract(
            crv_token.into_hash().unwrap_or_revert().into(),
            None,
            "rate",
            runtime_args! {},
        ));
        data::set_future_epoch_time(runtime::call_versioned_contract(
            crv_token.into_hash().unwrap_or_revert().into(),
            None,
            "future_epoch_time_write",
            runtime_args! {},
        ));

        data::set_lock(false);
    }
    fn reward_data(&mut self, reward_token: Key) -> RewardDataStruct {
        RewardData::instance().get(&reward_token)
    }
    fn lp_token(&mut self) -> Key {
        data::get_lp_token()
    }
    fn reward_count(&mut self) -> U256 {
        data::get_reward_count()
    }
    fn admin(&mut self) -> Key {
        data::get_admin()
    }
    fn reward_integral(&mut self, reward_token: Key) -> U256 {
        RewardIntegral::instance().get(&reward_token)
    }
    fn reward_tokens(&mut self, index: U256) -> Key {
        RewardTokens::instance().get(&index)
    }

    fn future_admin(&mut self) -> Key {
        data::get_future_admin()
    }
    fn claim_data(&mut self, user: Key, claiming_address: Key) -> ClaimDataStruct {
        ClaimData::instance().get(&user, &claiming_address)
    }

    //function implementaion of liquidity gauge v4

    fn integrate_checkpoint(&self) -> U256 {
        PeriodTimestamp::instance().get(&U256::from(data::get_period()))
    }

    fn _update_liquidity_limit(&self, addr: Key, l: U256, _supply: U256) {
        let voting_escrow: Key = data::get_voting_escrow();
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
            .checked_mul(data::TOKENLESS_PRODUCTION)
            .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError2)
            .checked_div(100.into())
            .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError3);
        let _block_timestamp: u64 = runtime::get_blocktime().into();
        if voting_total > 0.into() {
            lim = lim
                .checked_add(
                    _supply
                        .checked_mul(voting_balance)
                        .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError4)
                        .checked_div(voting_total)
                        .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError5)
                        .checked_mul(
                            U256::from(100)
                                .checked_sub(data::TOKENLESS_PRODUCTION)
                                .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError6),
                        )
                        .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError7)
                        .checked_div(100.into())
                        .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError8),
                )
                .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError9);
        }
        lim = U256::min(l, lim);
        let old_bal: U256 = data::WorkingBalances::instance().get(&addr);
        data::WorkingBalances::instance().set(&addr, lim);
        let working_supply: U256 = data::get_working_supply()
            .checked_add(lim)
            .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError10)
            .checked_sub(old_bal)
            .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError11);
        data::set_working_supply(working_supply);
        self.emit(&LiquidityGaugeV4Event::UpdateLiquidityLimit {
            user: addr,
            original_balance: l,
            original_supply: _supply,
            working_balance: lim,
            working_supply,
        });
    }

    fn _checkpoint_rewards(
        &mut self,
        _user: Key,
        _total_supply: U256,
        _claim: bool,
        _receiver: Key,
    ) {
        let mut user_balance:U256 = U256::from(0);
        let mut receiver = _receiver;

        if _user != zero_address() {
            user_balance = self.balance_of(Address::from(_user));
            if _claim && receiver == zero_address() {
                // if receiver is not explicitly declared, check if a default receiver is set
                receiver = RewardsReceiver::instance().get(&_user);
                if receiver == zero_address() {
                    //# if receiver is not explicitly declared, check if a default receiver is set
                    receiver = _user;
                }
            }
        }

        let reward_count = self.reward_count();
        for i in 0..(MAX_REWARDS.as_usize()) {
            if U256::from(i) == reward_count {
                break;
            }
            let token: Key = self.reward_tokens(i.into());

            let mut reward_data: RewardDataStruct = self.reward_data(token);
            let mut integral: U256 = reward_data.integral;

            let block_timestamp: u64 = runtime::get_blocktime().into();
            let last_update: u64 = block_timestamp.min(reward_data.period_finish);
            let duration: u64 = last_update
                .checked_sub(reward_data.last_update)
                .unwrap_or_revert_with(Error::LiquidityGaugeV4CheckpointRewardsSubtractionOverFlow);
            if duration != 0 {
              reward_data.last_update = last_update;
              if _total_supply != U256::from(0) {
                integral = integral
                  .checked_add(
                    U256::from(duration)
                      .checked_mul(reward_data.rate)
                      .unwrap_or_revert_with(Error::LiquidityGaugeV4CheckpointRewardsMultiplicationOverFlow1)
                      .checked_mul(U256::from(1000000000))
                      .unwrap_or_revert_with(Error::LiquidityGaugeV4CheckpointRewardsMultiplicationOverFlow2)
                    / _total_supply
                  )
                  .unwrap_or_revert_with(Error::LiquidityGaugeV4CheckpointRewardsAdditionOverFlow);
                reward_data.integral = integral;
              }
              RewardData::instance().set(&token, reward_data);
            }

            if _user != zero_address() {
                let integral_for = RewardIntegralFor::instance().get(&token, &_user);
                let mut new_claimable: U256 = U256::from(0);
                if integral_for < integral {
                    RewardIntegralFor::instance().set(&token, &_user, integral);
                    new_claimable = user_balance
                        .checked_mul(
                            integral
                                .checked_sub(integral_for)
                                .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError47),
                        )
                        .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError16)
                        .checked_div(U256::from(1000000000))
                        .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError17);
                }
                let mut claim_data: ClaimDataStruct = self.claim_data(_user, token);
                let total_claimable: U256 = claim_data
                    .claimable_amount
                    .checked_add(new_claimable)
                    .unwrap_or_revert_with(Error::LiquidityGaugeUnderFlow6);
                if total_claimable > U256::from(0) {
                    let total_claimed = claim_data.claimed_amount;
                    if _claim {
                        let token_hash_add_array = match token {
                            Key::Hash(package) => package,
                            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
                        };
                        let token_package_hash = ContractPackageHash::new(token_hash_add_array);
                        let () = runtime::call_versioned_contract(
                            token_package_hash,
                            None,
                            "transfer",
                            runtime_args! {"to" => Address::from(receiver),"amount" => total_claimable},
                        );
                        // if len(response) != 0:
                        //     assert convert(response, bool)
                        claim_data.claimed_amount = total_claimed
                            .checked_add(total_claimable)
                            .unwrap_or_revert_with(Error::LiquidityGaugeOverFlow3);
                        ClaimData::instance().set(&_user, &token, claim_data);
                    } else if new_claimable > U256::from(0) {
                        claim_data.claimed_amount = total_claimed;
                        claim_data.claimable_amount = total_claimable;
                        ClaimData::instance().set(&_user, &token, claim_data);
                    }
                }
            }
        }
    }
    fn _checkpoint(&mut self, addr: Key) {
        let token: Key = data::get_crv_token();
        let controller: Key = data::get_controller();
        let mut period: i128 = data::get_period();
        let period_time: U256 = data::PeriodTimestamp::instance().get(&U256::from(period));
        let mut integrate_inv_supply: U256 =
            data::IntegrateInvSupply::instance().get(&U256::from(period));
        let mut rate: U256 = data::get_inflation_rate();
        let mut new_rate: U256 = rate;
        let prev_future_epoch: U256 = data::get_future_epoch_time();
        if prev_future_epoch >= period_time {
            data::set_future_epoch_time(runtime::call_versioned_contract(
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
            data::set_inflation_rate(new_rate);
        }
        if data::get_is_killed() {
            rate = 0.into();
        }
        let block_timestamp: u64 = runtime::get_blocktime().into();
        
        if U256::from(block_timestamp) > period_time {
            let working_supply = data::get_working_supply();
            let () = runtime::call_versioned_contract(
                controller.into_hash().unwrap_or_revert().into(),
                None,
                "checkpoint_gauge",
                runtime_args! {
                    "addr" => Key::from(data::get_package_hash())
                },
            );
            let mut prev_week_time = period_time;
            let mut week_time = U256::min(
                (period_time
                    .checked_add(data::WEEK)
                    .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError18))
                .checked_div(data::WEEK)
                .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError19)
                .checked_mul(data::WEEK)
                .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError20),
                U256::from(block_timestamp),
            );
            
            for i in 0..500 {
                let dt: U256 = week_time
                    .checked_sub(prev_week_time)
                    .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError21);
                let w: U256 = runtime::call_versioned_contract(
                    controller.into_hash().unwrap_or_revert().into(),
                    None,
                    "gauge_relative_weight",
                    runtime_args! {
                        "addr" => Key::from(data::get_package_hash()),
                        "time" => Some(prev_week_time.checked_div(data::WEEK).unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError22).checked_mul(data::WEEK).unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError23))
                    },
                );
                if working_supply > 0.into() {
                    if (prev_future_epoch >= prev_week_time) && (prev_future_epoch < week_time) {
                        integrate_inv_supply = integrate_inv_supply
                            .checked_add(
                                rate.checked_mul(w)
                                    .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError24)
                                    .checked_mul(
                                        prev_future_epoch
                                            .checked_sub(prev_week_time)
                                            .unwrap_or_revert_with(
                                                Error::LiquidityGaugeArithmeticError25,
                                            ),
                                    )
                                    .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError26)
                                    .checked_div(working_supply)
                                    .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError27),
                            )
                            .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError28);
                        rate = new_rate;
                        integrate_inv_supply = integrate_inv_supply
                            .checked_add(
                                rate.checked_mul(w)
                                    .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError29)
                                    .checked_mul(
                                        week_time
                                            .checked_sub(prev_future_epoch)
                                            .unwrap_or_revert_with(
                                                Error::LiquidityGaugeArithmeticError30,
                                            ),
                                    )
                                    .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError31)
                                    .checked_div(working_supply)
                                    .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError32),
                            )
                            .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError33);
                    } else {
                        integrate_inv_supply = integrate_inv_supply
                            .checked_add(
                                rate.checked_mul(w)
                                    .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError34)
                                    .checked_mul(dt)
                                    .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError35)
                                    .checked_div(working_supply)
                                    .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError36),
                            )
                            .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError37);
                    }
                }

                if week_time == block_timestamp.into() {
                    break;
                }
                prev_week_time = week_time;
                week_time = U256::min(
                    week_time
                        .checked_add(data::WEEK)
                        .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError38),
                    block_timestamp.into(),
                );
            }
        }
        period = period
            .checked_add(1.into())
            .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError39);
        data::set_period(period);
        data::PeriodTimestamp::instance().set(&U256::from(period), block_timestamp.into());
        data::IntegrateInvSupply::instance().set(&U256::from(period), integrate_inv_supply);
        let working_balance: U256 = data::WorkingBalances::instance().get(&addr);
        data::IntegrateFraction::instance().set(
            &addr,
            data::IntegrateFraction::instance()
                .get(&addr)
                .checked_add(working_balance)
                .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError40)
                .checked_mul(
                    integrate_inv_supply
                        .checked_sub(data::IntegrateInvSupplyOf::instance().get(&addr))
                        .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError41),
                )
                .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError42)
                .checked_div(U256::from(10).pow(9.into()))
                .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError43),
        );
        data::IntegrateInvSupplyOf::instance().set(&addr, integrate_inv_supply);
        data::IntegrateCheckpointOf::instance().set(&addr, block_timestamp.into());
    }
    fn user_checkpoint(&mut self, addr: Key) -> bool {
        if !(self.get_caller() == addr || self.get_caller() == data::get_minter()) {
            runtime::revert(Error::LiquidityGuageUnauthorized);
        }
        self._checkpoint(addr);
        self._update_liquidity_limit(
            addr,
            self.balance_of(Address::from(addr)),
            self.total_supply(),
        );
        true
    }

    fn claimable_tokens(&mut self, addr: Key) -> U256 {
        self._checkpoint(addr);
        data::IntegrateFraction::instance()
            .get(&addr)
            .checked_sub(runtime::call_versioned_contract(
                data::get_minter().into_hash().unwrap_or_revert().into(),
                None,
                "minted",
                runtime_args! {
                    "owner" => addr,
                    "spender" => Key::from(data::get_package_hash())
                },
            ))
            .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError44)
    }

    fn claimed_reward(&mut self, addr: Key, token: Key) -> U256 {
        self.claim_data(addr, token).claimed_amount
    }

    fn claimable_reward(&mut self, user: Key, reward_token: Key) -> U256 {
        let reward_data: RewardDataStruct = self.reward_data(reward_token);
        let mut integral: U256 = reward_data.integral;
        let total_supply = self.total_supply();
        if total_supply != U256::from(0) {
          let block_timestamp: u64 = runtime::get_blocktime().into();
          let last_update: u64 = block_timestamp.min(reward_data.period_finish);
          let duration : u64 = last_update
            .checked_sub(reward_data.last_update)
            .unwrap_or_revert_with(Error::LiquidityGaugeV4ClaimableRewardSubtractionOverFlow1);
          integral = integral
            .checked_add(
              U256::from(duration)
                .checked_mul(reward_data.rate)
                .unwrap_or_revert_with(Error::LiquidityGaugeV4ClaimableRewardMultiplicationOverFlow1)
                .checked_mul(U256::from(1000000000))
                .unwrap_or_revert_with(Error::LiquidityGaugeV4ClaimableRewardMultiplicationOverFlow2)
                / total_supply
            )
            .unwrap_or_revert_with(Error::LiquidityGaugeV4ClaimableRewardAdditionOverFlow1)
        }
        let integral_for: U256 = RewardIntegralFor::instance().get(&reward_token, &user);
        let new_claimable = self.balance_of(Address::from(user))
          .checked_mul(
            integral
              .checked_sub(integral_for)
              .unwrap_or_revert_with(Error::LiquidityGaugeV4ClaimableRewardSubtractionOverFlow2)
          )
          .unwrap_or_revert_with(Error::LiquidityGaugeV4ClaimableRewardMultiplicationOverFlow3)
          / 1000000000;
        self.claim_data(user, reward_token).claimable_amount
          .checked_add(new_claimable)
          .unwrap_or_revert_with(Error::LiquidityGaugeV4ClaimableRewardAdditionOverFlow2)
    }

    fn set_rewards_receiver(&mut self, receiver: Key) {
        RewardsReceiver::instance().set(&self.get_caller(), receiver)
    }

    fn claim_rewards(&mut self, _addr: Option<Key>, _receiver: Option<Key>) {
        let lock = data::get_lock();
        if lock {
            // Locked
            runtime::revert(Error::LiquidityGaugeLocked2);
        }
        data::set_lock(true);
        let addr: Key = if let Some(..) = _addr {
            _addr.unwrap()
        } else {
            self.get_caller()
        };
        let receiver: Key = if let Some(..) = _receiver {
            _receiver.unwrap()
        } else {
            zero_address()
        };
        if receiver != zero_address() && addr != self.get_caller() {
            runtime::revert(Error::LiquidityGaugeCannotRedirectWhenClaimingForAnotherUser);
        }
        let _total_supply = self.total_supply();
        self._checkpoint_rewards(addr, _total_supply, true, receiver);
        data::set_lock(false);
    }

    fn kick(&mut self, addr: Key) {
        let voting_escrow: Key = data::get_voting_escrow();
        let t_last: U256 = data::IntegrateCheckpointOf::instance().get(&addr);
        let ret: U256 = runtime::call_versioned_contract(
            voting_escrow.into_hash().unwrap_or_revert().into(),
            None,
            "user_point_epoch",
            runtime_args! {
                "user" => addr,
            },
        );
        let t_ve: U256 = runtime::call_versioned_contract(
            voting_escrow.into_hash().unwrap_or_revert().into(),
            None,
            "user_point_history_ts",
            runtime_args! {
                "addr" => addr,
                "epoch" => ret
            },
        );
        let balance: U256 = self.balance_of(Address::from(addr));
        let ret: U256 = runtime::call_versioned_contract(
            data::get_voting_escrow()
                .into_hash()
                .unwrap_or_revert()
                .into(),
            None,
            "balance_of",
            runtime_args! {
                "addr" => addr,
                "t"=>None::<U256>
            },
        );
        if !((ret == 0.into()) || (t_ve > t_last)) {
            runtime::revert(ApiError::User(Error::LiquidityGuageKickNotAllowed1 as u16));
        }
        if data::WorkingBalances::instance().get(&addr)
            <= balance
                .checked_mul(data::TOKENLESS_PRODUCTION)
                .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError45)
                .checked_div(100.into())
                .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError46)
        {
            runtime::revert(ApiError::User(Error::LiquidityGuageKickNotAllowed2 as u16));
        }
        self._checkpoint(addr);
        self._update_liquidity_limit(
            addr,
            self.balance_of(Address::from(addr)),
            self.total_supply(),
        );
    }

    fn deposit(&mut self, value: U256, addr: Option<Key>, claim_rewards: Option<bool>) {
        let _claim_rewards: bool = if let Some(..) = claim_rewards {
            claim_rewards.unwrap()
        } else {
            false
        };
        let _addr: Key = if let Some(..) = addr {
            addr.unwrap()
        } else {
            self.get_caller()
        };

        let lock = data::get_lock();
        if lock {
            //Locked
            runtime::revert(Error::LiquidityGaugeLocked1);
        }
        data::set_lock(true);
        self._checkpoint(_addr);
        if value != 0.into() {
            let is_rewards: bool = self.reward_tokens(0.into()) != zero_address();
            let mut total_supply = self.total_supply();
            if is_rewards {
                self._checkpoint_rewards(_addr, total_supply, _claim_rewards, zero_address());
            }
            total_supply = total_supply
                .checked_add(value)
                .unwrap_or_revert_with(Error::LiquidityGaugeOverFlow1);
            let balance = self.balance_of(Address::from(_addr));
            let new_balance = balance
                .checked_add(value)
                .unwrap_or_revert_with(Error::LiquidityGaugeOverFlow2);
            self.set_balance(Address::from(_addr), new_balance);
            self.set_total_supply(total_supply);
            self._update_liquidity_limit(_addr, new_balance, total_supply);

            let lp_token = self.lp_token();
            let token_hash_add_array = match lp_token {
                Key::Hash(package) => package,
                _ => runtime::revert(ApiError::UnexpectedKeyVariant),
            };
            let token_package_hash = ContractPackageHash::new(token_hash_add_array);
            let _ret: () = runtime::call_versioned_contract(
                token_package_hash,
                None,
                "transfer_from",
                runtime_args! {
                    "owner" => Address::from(self.get_caller()),
                    "recipient" => Address::from(data::get_package_hash()),
                    "amount" => value
                },
            );
        }
        self.emit(&LiquidityGaugeV4Event::Deposit {
            provider: _addr,
            value,
        });
        self.emit(&LiquidityGaugeV4Event::Transfer {
            from: zero_address(),
            to: _addr,
            value,
        });
        data::set_lock(false);
    }
    fn withdraw(&mut self, value: U256, claim_rewards: Option<bool>) {
        let lock = data::get_lock();
        if lock {
            runtime::revert(Error::LiquidityGaugeLocked3);
        }
        data::set_lock(true);
        let claim_rewards: bool = claim_rewards.is_some();
        self._checkpoint(self.get_caller());
        let mut _total_supply: U256 = 0.into();
        if value != 0.into() {
            let is_rewards: bool = data::RewardTokens::instance().get(&0.into()) != zero_address();
            _total_supply = self.total_supply();
            if is_rewards {
                self._checkpoint_rewards(
                    self.get_caller(),
                    _total_supply,
                    claim_rewards,
                    zero_address(),
                )
            }
            _total_supply = _total_supply
                .checked_sub(value)
                .unwrap_or_revert_with(Error::LiquidityGaugeUnderFlow5);
            let balance = self.balance_of(Address::from(self.get_caller()));
            let new_balance = balance
                .checked_sub(value)
                .unwrap_or_revert_with(Error::LiquidityGaugeUnderFlow6);
            self.set_balance(Address::from(self.get_caller()), new_balance);
            self.set_total_supply(_total_supply);
            self._update_liquidity_limit(self.get_caller(), new_balance, _total_supply);
           
            let lp_token = self.lp_token();
            let token_hash_add_array = match lp_token {
                Key::Hash(package) => package,
                _ => runtime::revert(ApiError::UnexpectedKeyVariant),
            };
            let token_package_hash = ContractPackageHash::new(token_hash_add_array);
            let _result: () = runtime::call_versioned_contract(
                token_package_hash,
                None,
                "transfer",
                runtime_args! {"recipient" => Address::from(self.get_caller()),"amount" => value},
            );
        }
        self.emit(&LiquidityGaugeV4Event::Withdraw {
            provider: self.get_caller(),
            value,
        });
        self.emit(&LiquidityGaugeV4Event::Transfer {
            from: self.get_caller(),
            to: zero_address(),
            value,
        });
        data::set_lock(false);
    }
    fn _transfer(&mut self, from: Key, to: Key, value: U256) {
        self._checkpoint(from);
        self._checkpoint(to);
        if value != 0.into() {
            let total_supply = self.total_supply();
            let is_rewards: bool = self.reward_tokens(0.into()) != zero_address();
            if is_rewards {
                self._checkpoint_rewards(from, total_supply, false, zero_address());
            }
            let _from_balance: U256 = self.balance_of(Address::from(from));
            let from_new_balance = _from_balance
                .checked_sub(value)
                .unwrap_or_revert_with(Error::LiquidityGaugeUnderFlow3);
            self.set_balance(Address::from(from), from_new_balance);
            self._update_liquidity_limit(from, from_new_balance, total_supply);
            if is_rewards {
                self._checkpoint_rewards(to, total_supply, false, zero_address());
            }
            let _to_balance: U256 = self.balance_of(Address::from(to));
            let to_new_balance = _to_balance
                .checked_add(value)
                .unwrap_or_revert_with(Error::LiquidityGaugeUnderFlow4);
            self.set_balance(Address::from(to), to_new_balance);
            self._update_liquidity_limit(to, to_new_balance, total_supply);
        }
        self.emit(&LiquidityGaugeV4Event::Transfer { from, to, value });
    }

    fn transfer(&mut self, recipient: Address, amount: U256) -> Result<(), u32> {
        let lock = data::get_lock();
        if lock {
            runtime::revert(Error::LiquidityGaugeLocked4);
        }
        data::set_lock(true);
        self._transfer(self.get_caller(), Key::from(recipient), amount);
        data::set_lock(false);
        Ok(())
    }
    fn transfer_from(
        &mut self,
        owner: Address,
        recipient: Address,
        amount: U256,
    ) -> Result<(), u32> {
        let lock = data::get_lock();
        if lock {
            //Locked
            runtime::revert(Error::LiquidityGaugeLocked5);
        }
        data::set_lock(true);
        //let allowances = Allowance::instance();
        let _allowance: U256 = self.allowance(owner, Address::from(self.get_caller()));
        if _allowance != U256::MAX {
            let _new_allowance: U256 = _allowance
                .checked_sub(amount)
                .unwrap_or_revert_with(Error::LiquidityGaugeUnderFlow2);
            self.set_allowance(owner, Address::from(self.get_caller()), _new_allowance);
        }
        self._transfer(Key::from(owner), Key::from(recipient), amount);
        data::set_lock(false);
        Ok(())
    }

    fn approve(&self, spender: Address, amount: U256) -> Result<(), Erc20Error> {
        CURVEERC20::approve(self, spender, amount)
    }
    fn increase_allowance(&self, spender: Address, amount: U256) -> Result<(), Erc20Error> {
        let res = CURVEERC20::increase_allowance(self, spender, amount);
        self.emit(&LiquidityGaugeV4Event::Approval {
            owner: self.get_caller(),
            spender: Key::from(spender),
            value: amount,
        });
        res
    }
    fn decrease_allowance(&self, spender: Address, amount: U256) -> Result<(), Erc20Error> {
        let res = CURVEERC20::decrease_allowance(self, spender, amount);
        self.emit(&LiquidityGaugeV4Event::Approval {
            owner: self.get_caller(),
            spender: Key::from(spender),
            value: amount,
        });
        res
    }

    fn add_reward(&mut self, _reward_token: Key, distributor: Key) {
        if self.get_caller() != self.admin() {
            runtime::revert(Error::LiquidityGaugeOnlyAdmin2);
        }
        let reward_count = self.reward_count();
        if reward_count >= MAX_REWARDS {
            runtime::revert(Error::LiquidityGaugeV4RewardCountExceedsMax);
        }
        let mut reward_data: RewardDataStruct = self.reward_data(_reward_token);
        if reward_data.distributor != zero_address() {
            runtime::revert(Error::LiquidityGaugeV4RewardDataExists1);
        }
        reward_data.distributor = distributor;
        RewardData::instance().set(&_reward_token, reward_data);

        RewardTokens::instance().set(&reward_count.into(), _reward_token);
        let new_reward_count = reward_count
            .checked_add(U256::from(1))
            .unwrap_or_revert_with(Error::LiquidityGaugeV4AddRewardAdditionOverFlow);
        data::set_reward_count(new_reward_count.into())
    }

    fn set_reward_distributor(&mut self, _reward_token: Key, distributor: Key) {
        let mut reward_data: RewardDataStruct = self.reward_data(_reward_token);
        let current_distributor: Key = reward_data.distributor;
        if self.get_caller() != self.admin() && self.get_caller() != current_distributor {
          runtime::revert(Error::LiquidityGaugeOnlyAdminOrDistributor);
        }
        if current_distributor == zero_address() {
            runtime::revert(Error::LiquidityGaugeV4CurrentDistributorExists1);
        }
        if distributor == zero_address() {
          runtime::revert(Error::LiquidityGaugeV4NewDistributorExists1);
        }

        if reward_data.distributor != zero_address() {
            runtime::revert(Error::LiquidityGaugeV4RewardDataExists1);
        }

        reward_data.distributor = distributor;
        RewardData::instance().set(&_reward_token, reward_data);
    }

    fn deposit_reward_token(&mut self, _reward_token: Key, amount: U256) {
        let lock = data::get_lock();
        if lock {
            runtime::revert(Error::LiquidityGaugeLocked6);
        }
        data::set_lock(true);

        let mut reward_data: RewardDataStruct = self.reward_data(_reward_token);
        let current_distributor: Key = reward_data.distributor;
        if self.get_caller() != current_distributor {
          runtime::revert(Error::LiquidityGaugeOnlyDistributor);
        }

        self._checkpoint_rewards(
          zero_address(),
          self.total_supply(),
          false,
          zero_address(),
        );
        let token_hash = match _reward_token {
          Key::Hash(package) => package,
          _ => runtime::revert(ApiError::UnexpectedContractRefVariant),
        };
        let token_package_hash = ContractPackageHash::new(token_hash);
        let _ret: () = runtime::call_versioned_contract(
          token_package_hash,
          None,
          "transfer_from",
          runtime_args! {
              "owner" => Address::from(self.get_caller()),
              "recipient" => Address::from(data::get_package_hash()),
              "amount" => amount
          },
        );

        let period_finish: u64 = reward_data.period_finish;
        let block_timestamp: u64 = runtime::get_blocktime().into();
        if block_timestamp >= period_finish {
          reward_data.rate = amount / data::WEEK;
        } else {
          let remaining: u64 = period_finish
            .checked_sub(block_timestamp)
            .unwrap_or_revert_with(Error::LiquidityGaugeV4DepositRewardTokensSubtractionOverFlow);
          let leftover: U256 = U256::from(remaining)
            .checked_mul(reward_data.rate)
            .unwrap_or_revert_with(Error::LiquidityGaugeV4DepositRewardTokensMultiplicationOverFlow);
          reward_data.rate = amount
            .checked_add(leftover)
            .unwrap_or_revert_with(Error::LiquidityGaugeV4DepositRewardTokensAdditionOverFlow1) 
            / data::WEEK
        }

        reward_data.last_update = block_timestamp;
        reward_data.period_finish = block_timestamp
          .checked_add(data::WEEK.as_u64())
          .unwrap_or_revert_with(Error::LiquidityGaugeV4DepositRewardTokensAdditionOverFlow2);

        RewardData::instance().set(&_reward_token, reward_data);
        data::set_lock(false);
    }

    fn set_killed(&mut self, is_killed: bool) {
        if self.get_caller() != self.admin() {
            runtime::revert(Error::LiquidityGaugeOnlyAdmin1);
        }
        data::set_is_killed(is_killed);
    }

    fn commit_transfer_ownership(&mut self, addr: Key) {
        if self.get_caller() != self.admin() {
            runtime::revert(Error::LiquidityGaugeOnlyAdmin3);
        }
        data::set_future_admin(addr);
        self.emit(&LiquidityGaugeV4Event::CommitOwnership { admin: addr });
    }

    fn accept_transfer_ownership(&mut self) {
        let _admin = self.future_admin();
        if self.get_caller() != _admin {
            runtime::revert(Error::LiquidityGaugeOnlyFutureAdmin);
        }
        data::set_admin(_admin);
        self.emit(&LiquidityGaugeV4Event::ApplyOwnership { admin: _admin });
    }

    fn emit(&self, liquidity_gauge_event: &LiquidityGaugeV4Event) {
        match liquidity_gauge_event {
            LiquidityGaugeV4Event::Deposit { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeV4Event::Withdraw { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeV4Event::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("spender", spender.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeV4Event::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeV4Event::UpdateLiquidityLimit {
                user,
                original_balance,
                original_supply,
                working_balance,
                working_supply,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("user", user.to_string());
                event.insert("original_balance", original_balance.to_string());
                event.insert("original_supply", original_supply.to_string());
                event.insert("working_balance", working_balance.to_string());
                event.insert("working_supply", working_supply.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeV4Event::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeV4Event::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
        };
    }
}
