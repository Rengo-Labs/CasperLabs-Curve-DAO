use crate::data::{
    self, get_lp_token, get_package_hash, ClaimData, ClaimDataStruct, PeriodTimestamp, RewardData,
    RewardIntegral, RewardIntegralFor, RewardTokens, RewardsReceiver, CLAIM_FREQUENCY, MAX_REWARDS,
};
use crate::{alloc::string::ToString, event::*};
use alloc::vec::Vec;
use alloc::{collections::BTreeMap, string::String};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::bytesrepr::Bytes;
use casper_types::{
    runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs, U256,
};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::{errors::*, utils::*};
use crv20::{self, Address, CURVEERC20};
use curve_casper_erc20::Error as Erc20Error;

pub trait LIQUIDITYTGAUGEV3<Storage: ContractStorage>:
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
        ClaimData::init();
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
    #[inline(always)]
    fn reward_data(&mut self) -> RewardData {
        data::reward_data()
    }
    #[inline(always)]
    fn lp_token(&mut self) -> Key {
        data::get_lp_token()
    }
    #[inline(always)]
    fn admin(&mut self) -> Key {
        data::get_admin()
    }
    #[inline(always)]
    fn reward_integral(&mut self, reward_token: Key) -> U256 {
        RewardIntegral::instance().get(&reward_token)
    }
    #[inline(always)]
    fn reward_tokens(&mut self, index: U256) -> Key {
        RewardTokens::instance().get(&index)
    }

    #[inline(always)]
    fn future_admin(&mut self) -> Key {
        data::get_future_admin()
    }
    #[inline(always)]
    fn claim_data(&mut self, user: Key, claiming_address: Key) -> ClaimDataStruct {
        ClaimData::instance().get(&user, &claiming_address)
    }

    //function implementaion of liquidity gauge v3

    #[inline(always)]
    fn integrate_checkpoint(&self) -> U256 {
        PeriodTimestamp::instance().get(&U256::from(data::get_period()))
    }

    #[inline(always)]
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
        self.emit(&LiquidityGaugeV3Event::UpdateLiquidityLimit {
            user: addr,
            original_balance: l,
            original_supply: _supply,
            working_balance: lim,
            working_supply,
        });
    }

    #[inline(always)]
    fn _checkpoint_rewards(
        &mut self,
        _user: Key,
        _total_supply: U256,
        _claim: bool,
        _receiver: Key,
    ) {
        let mut reward_tokens: Vec<Key> = Vec::new();
        let mut reward_integrals: Vec<U256> = Vec::new();
        for i in 0..(MAX_REWARDS.as_usize()) {
            let token: Key = self.reward_tokens(i.into());
            if token == zero_address() {
                break;
            }
            reward_tokens.push(token);
            reward_integrals.push(self.reward_integral(token));
        }
        let mut reward_data: RewardData = self.reward_data();
        if _total_supply != 0.into()
            && reward_data.address != zero_address()
            && reward_data.time_stamp != 0.into()
            && U256::from(u64::from(runtime::get_blocktime()))
                > (reward_data.time_stamp + U256::from(CLAIM_FREQUENCY.as_u128()))
        {
            let mut reward_balances: Vec<U256> = Vec::new();
            for i in 0..(MAX_REWARDS.as_usize()) {
                let token: Key = self.reward_tokens(i.into());
                if token == zero_address() {
                    break;
                }
                reward_balances.push(runtime::call_versioned_contract(
                    token.into_hash().unwrap_or_revert().into(),
                    None,
                    "balance_of",
                    runtime_args! {
                        "address" => Key::from(data::get_package_hash())
                    },
                ));
            }
            let reward_contract = reward_data.address;
            let reward_contract_hash_add_array = match reward_contract {
                Key::Hash(package) => package,
                _ => runtime::revert(ApiError::UnexpectedKeyVariant),
            };
            let reward_contract_package_hash =
                ContractPackageHash::new(reward_contract_hash_add_array);
            let () = runtime::call_versioned_contract(
                reward_contract_package_hash,
                None,
                "claim_sig",
                runtime_args! {},
            );
            reward_data.address = reward_contract;
            reward_data.time_stamp = U256::from(u64::from(runtime::get_blocktime()));

            for i in 0..(MAX_REWARDS.as_usize()) {
                let token: Key = self.reward_tokens(i.into());
                if token == zero_address() {
                    break;
                }

                let token_balance: U256 = runtime::call_versioned_contract(
                    token.into_hash().unwrap_or_revert().into(),
                    None,
                    "balance_of",
                    runtime_args! {
                        "address" => Key::from(get_package_hash())
                    },
                );

                let d_i: U256 = U256::from(1000000000)
                    .checked_mul(
                        token_balance
                            .checked_sub(reward_balances[i])
                            .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError12),
                    )
                    .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError13)
                    .checked_div(_total_supply)
                    .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError14);
                if d_i > 0.into() {
                    reward_integrals[i] = reward_integrals[i]
                        .checked_add(d_i)
                        .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError15);
                    data::RewardIntegral::instance().set(&token, reward_integrals[i]);
                }
            }
        }

        if _user != zero_address() {
            let mut receiver: Key = _receiver;
            if _claim && receiver == zero_address() {
                receiver = RewardsReceiver::instance().get(&_user);
                if receiver == zero_address() {
                    receiver = _user;
                }
            }

            let user_balance: U256 = self.balance_of(Address::from(_user));
            for (i, item) in reward_integrals
                .iter()
                .enumerate()
                .take(MAX_REWARDS.as_usize())
            {
                let token: Key = self.reward_tokens(i.into());
                if token == zero_address() {
                    break;
                }
                let integral = *item;
                let integral_for = RewardIntegralFor::instance().get(&token, &_user);
                let mut new_claimable: U256 = 0.into();
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
                if total_claimable > 0.into() {
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
                    } else if new_claimable > 0.into() {
                        claim_data.claimed_amount = total_claimed;
                        claim_data.claimable_amount = total_claimable;
                        ClaimData::instance().set(&_user, &token, claim_data);
                    }
                }
            }
        }
    }
    #[inline(always)]
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
            let mut working_supply = data::get_working_supply();
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

            for _ in 0..500 {
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
    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
    fn reward_contract(&mut self) -> Key {
        let address = self.reward_data().address;
        if address == zero_address() {
            zero_address()
        } else {
            address
        }
    }

    #[inline(always)]
    fn last_claim(&mut self) -> U256 {
        self.reward_data().time_stamp
    }

    #[inline(always)]
    fn claimed_reward(&mut self, addr: Key, token: Key) -> U256 {
        self.claim_data(addr, token).claimed_amount
    }

    #[inline(always)]
    fn claimable_reward(&mut self, addr: Key, token: Key) -> U256 {
        self.claim_data(addr, token).claimable_amount
    }

    #[inline(always)]
    fn claimable_reward_write(&mut self, addr: Key, token: Key) -> U256 {
        let lock = data::get_lock();
        if lock {
            // Locked
            runtime::revert(Error::LiquidityGaugeLocked1);
        }
        data::set_lock(true);
        let reward_token = self.reward_tokens(0.into());
        if reward_token != zero_address() {
            let total_supply = self.total_supply();
            self._checkpoint_rewards(addr, total_supply, false, zero_address());
        }
        data::set_lock(false);
        self.claim_data(addr, token).claimable_amount
    }

    #[inline(always)]
    fn set_rewards_receiver(&mut self, receiver: Key) {
        RewardsReceiver::instance().set(&self.get_caller(), receiver)
    }

    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
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
            if is_rewards {
                let reward_data: RewardData = self.reward_data();
                if reward_data.time_stamp > 0.into() {
                    let reward_contract = reward_data.address;
                    let () = runtime::call_versioned_contract(
                        reward_contract.into_hash().unwrap_or_revert().into(),
                        None,
                        "deposit",
                        runtime_args! {
                            "value"=>value
                        },
                    );
                }
            }
        }
        self.emit(&LiquidityGaugeV3Event::Deposit {
            provider: _addr,
            value,
        });
        self.emit(&LiquidityGaugeV3Event::Transfer {
            from: zero_address(),
            to: _addr,
            value,
        });
        data::set_lock(false);
    }
    #[inline(always)]
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
            if is_rewards {
                let reward_data: RewardData = self.reward_data();
                if reward_data.time_stamp > 0.into() {
                    let reward_contract = reward_data.address;
                    let () = runtime::call_versioned_contract(
                        reward_contract.into_hash().unwrap_or_revert().into(),
                        None,
                        "withdraw",
                        runtime_args! {
                            "value"=>value
                        },
                    );
                }
            }
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
        self.emit(&LiquidityGaugeV3Event::Withdraw {
            provider: self.get_caller(),
            value,
        });
        self.emit(&LiquidityGaugeV3Event::Transfer {
            from: self.get_caller(),
            to: zero_address(),
            value,
        });
        data::set_lock(false);
    }
    #[inline(always)]
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
        self.emit(&LiquidityGaugeV3Event::Transfer { from, to, value });
    }

    #[inline(always)]
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
    #[inline(always)]
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

    #[inline(always)]
    fn approve(&self, spender: Address, amount: U256) -> Result<(), Erc20Error> {
        CURVEERC20::approve(self, spender, amount)
    }
    #[inline(always)]
    fn increase_allowance(&self, spender: Address, amount: U256) -> Result<(), Erc20Error> {
        let res = CURVEERC20::increase_allowance(self, spender, amount);
        self.emit(&LiquidityGaugeV3Event::Approval {
            owner: self.get_caller(),
            spender: Key::from(spender),
            value: amount,
        });
        res
    }
    #[inline(always)]
    fn decrease_allowance(&self, spender: Address, amount: U256) -> Result<(), Erc20Error> {
        let res = CURVEERC20::decrease_allowance(self, spender, amount);
        self.emit(&LiquidityGaugeV3Event::Approval {
            owner: self.get_caller(),
            spender: Key::from(spender),
            value: amount,
        });
        res
    }

    #[inline(always)]
    fn set_rewards(&mut self, reward_contract: Key, sigs: String, reward_tokens: Vec<String>) {
        let lock = data::get_lock();
        if lock {
            runtime::revert(Error::LiquidityGaugeLocked6);
        }
        data::set_lock(true);
        let _sigs: Bytes = Bytes::from(sigs.as_bytes());
        if self.get_caller() != self.admin() {
            runtime::revert(Error::LiquidityGaugeOnlyAdmin2);
        }
        let mut _reward_tokens: Vec<Key> = Vec::new();
        for reward_token in &reward_tokens {
            _reward_tokens.push(Key::from_formatted_str(reward_token).unwrap());
        }
        let _lp_token = self.lp_token();
        let current_reward_contract = self.reward_data().address;
        let total_supply = self.total_supply();
        if RewardTokens::instance().get(&0.into()) != zero_address() {
            self._checkpoint_rewards(zero_address(), total_supply, false, zero_address());
        }
        if current_reward_contract != zero_address() {
            if total_supply != 0.into() {
                let () = runtime::call_versioned_contract(
                    current_reward_contract
                        .into_hash()
                        .unwrap_or_revert()
                        .into(),
                    None,
                    "withdraw",
                    runtime_args! {
                        "value" => total_supply,
                        "addr" => None::<Key>
                    },
                );
            }
            let amount: U256 = 0.into();
            let () = runtime::call_versioned_contract(
                get_lp_token().into_hash().unwrap_or_revert().into(),
                None,
                "approve",
                runtime_args! {"spender" => Address::from(current_reward_contract),"amount" => amount},
            );
        }

        if reward_contract != zero_address() {
            let reward_token = self.reward_tokens(0.into());
            if reward_token == zero_address() {
                runtime::revert(Error::LiquidityGaugeTokenIsZeroAddress);
            }
            //is Contract Check is missing
            if total_supply == 0.into() {
                runtime::revert(Error::LiquidityGaugeZeroTotalSupply);
            }
            let () = runtime::call_versioned_contract(
                get_lp_token().into_hash().unwrap_or_revert().into(),
                None,
                "approve",
                runtime_args! {"spender" => Address::from(reward_contract),"amount" => U256::MAX},
            );
            let () = runtime::call_versioned_contract(
                reward_contract.into_hash().unwrap_or_revert().into(),
                None,
                "deposit",
                runtime_args! {
                    "value"=>total_supply
                },
            );
            let mut balance_of: U256 = runtime::call_versioned_contract(
                get_lp_token().into_hash().unwrap_or_revert().into(),
                None,
                "balance_of",
                runtime_args! {"address" => Address::from(get_package_hash())},
            );
            if balance_of != 0.into() {
                runtime::revert(Error::LiquidityGaugeFailedToDeposit);
            }
            let () = runtime::call_versioned_contract(
                reward_contract.into_hash().unwrap_or_revert().into(),
                None,
                "withdraw",
                runtime_args! {
                    "value" => total_supply,
                    "addr" => None::<Key>
                },
            );
            balance_of = runtime::call_versioned_contract(
                get_lp_token().into_hash().unwrap_or_revert().into(),
                None,
                "balance_of",
                runtime_args! {"address" => Address::from(get_package_hash())},
            );
            if balance_of != total_supply {
                runtime::revert(Error::LiquidityGaugeFailedToWithdraw);
            }
            let () = runtime::call_versioned_contract(
                reward_contract.into_hash().unwrap_or_revert().into(),
                None,
                "deposit",
                runtime_args! {
                    "value"=>total_supply
                },
            );
        }
        let mut reward_data = self.reward_data();
        reward_data.address = reward_contract;
        data::set_reward_sigs(_sigs);
        for (i, reward_token) in _reward_tokens
            .iter()
            .enumerate()
            .take(MAX_REWARDS.as_usize())
        {
            let current_token = self.reward_tokens(i.into());
            let new_token: Key = *reward_token;

            if current_token != zero_address() {
                if current_token != new_token {
                    runtime::revert(Error::LiquidityGaugeCannotModifyExistingRewardToken);
                }
            } else if new_token != zero_address() {
                RewardTokens::instance().set(&i.into(), new_token);
            } else {
                break;
            }
        }
        if reward_contract != zero_address() {
            // # do an initial checkpoint to verify that claims are working
            self._checkpoint_rewards(zero_address(), total_supply, false, zero_address())
        }
        data::set_lock(false);
    }

    #[inline(always)]
    fn set_killed(&mut self, is_killed: bool) {
        if self.get_caller() != self.admin() {
            runtime::revert(Error::LiquidityGaugeOnlyAdmin1);
        }
        data::set_is_killed(is_killed);
    }

    #[inline(always)]
    fn commit_transfer_ownership(&mut self, addr: Key) {
        if self.get_caller() != self.admin() {
            runtime::revert(Error::LiquidityGaugeOnlyAdmin3);
        }
        data::set_future_admin(addr);
        self.emit(&LiquidityGaugeV3Event::CommitOwnership { admin: addr });
    }

    #[inline(always)]
    fn accept_transfer_ownership(&mut self) {
        let _admin = self.future_admin();
        if self.get_caller() != _admin {
            runtime::revert(Error::LiquidityGaugeOnlyFutureAdmin);
        }
        data::set_admin(_admin);
        self.emit(&LiquidityGaugeV3Event::ApplyOwnership { admin: _admin });
    }

    fn emit(&self, liquidity_gauge_event: &LiquidityGaugeV3Event) {
        match liquidity_gauge_event {
            LiquidityGaugeV3Event::Deposit { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeV3Event::Withdraw { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeV3Event::Approval {
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
            LiquidityGaugeV3Event::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeV3Event::UpdateLiquidityLimit {
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
            LiquidityGaugeV3Event::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            LiquidityGaugeV3Event::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
        };
    }
}
