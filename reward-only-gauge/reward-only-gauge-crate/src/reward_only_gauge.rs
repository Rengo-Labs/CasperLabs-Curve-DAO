use crate::alloc::string::ToString;
use crate::data::{
    self, get_package_hash, ClaimData, ClaimDataStruct, RewardBalances, RewardData, RewardIntegral,
    RewardIntegralFor, RewardTokens, RewardsReceiver, CLAIM_FREQUENCY, MAX_REWARDS,
};
use alloc::collections::BTreeMap;
use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};

use casper_types::{
    runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs, U256,
};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::{errors::*, utils::*};
use crv20::{self, Address, CURVEERC20};
use curve_casper_erc20::Error as Erc20Error;
pub enum REWARDONLYGAUGEEvent {
    Withdraw {
        provider: Key,
        value: U256,
    },
    Deposit {
        provider: Key,
        value: U256,
    },
    CommitOwnership {
        admin: Key,
    },
    ApplyOwnership {
        admin: Key,
    },
    Approval {
        owner: Key,
        spender: Key,
        value: U256,
    },
    Transfer {
        from: Key,
        to: Key,
        value: U256,
    },
}

impl REWARDONLYGAUGEEvent {
    pub fn type_name(&self) -> String {
        match self {
            REWARDONLYGAUGEEvent::Withdraw {
                provider: _,
                value: _,
            } => "withdraw",
            REWARDONLYGAUGEEvent::Deposit {
                provider: _,
                value: _,
            } => "deposit",
            REWARDONLYGAUGEEvent::CommitOwnership { admin: _ } => "CommitOwnership",
            REWARDONLYGAUGEEvent::ApplyOwnership { admin: _ } => "ApplyOwnership",
            REWARDONLYGAUGEEvent::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "approve",
            REWARDONLYGAUGEEvent::Transfer {
                from: _,
                to: _,
                value: _,
            } => "transfer",
        }
        .to_string()
    }
}

pub trait REWARDONLYGAUGE<Storage: ContractStorage>:
    ContractContext<Storage> + CURVEERC20<Storage>
{
    /// @notice Contract constructor
    /// @param _admin Admin who can kill the gauge
    /// @param _lp_token Liquidity Pool contract address

    fn init(
        &mut self,
        _admin: Key,
        _lp_token: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        let _lp_token_hash_add_array = match _lp_token {
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
        let post_name: &str = " RewardGauge Deposit";
        name.push_str(symbol.as_str());
        name.push_str(post_name);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        CURVEERC20::init(self, data::get_hash(), data::get_package_hash());
        CURVEERC20::set_name(self, name);
        CURVEERC20::set_symbol(self, symbol + "-gauge");
        data::set_admin(_admin);
        data::set_lp_token(_lp_token);
        data::set_lock(0);
        RewardTokens::init();
        RewardBalances::init();
        RewardsReceiver::init();
        RewardIntegral::init();
        RewardIntegralFor::init();
        ClaimData::init();
    }

    #[inline(always)]
    fn reward_balances(&mut self, owner: Key) -> U256 {
        RewardBalances::instance().get(&owner)
    }
    #[inline(always)]
    fn rewards_receiver(&mut self, claimant: Key) -> Key {
        RewardsReceiver::instance().get(&claimant)
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
    fn transfer(&mut self, _to: Address, _value: U256) -> Result<(), Erc20Error> {
        let lock = data::get_lock();
        if lock != 0 {
            //Reward Only Gauge: Locked
            runtime::revert(Error::RewardOnlyGaugeLocked1);
        }
        data::set_lock(1);
        self._transfer(self.get_caller(), Key::from(_to), _value);
        data::set_lock(0);
        Ok(())
    }

    #[inline(always)]
    fn approve(&self, spender: Address, amount: U256) -> Result<(), Erc20Error> {
        CURVEERC20::approve(self, spender, amount)
    }

    #[inline(always)]
    fn reward_integral_for(&mut self, reward_token: Key, claiming_address: Key) -> U256 {
        RewardIntegralFor::instance().get(&reward_token, &claiming_address)
    }

    #[inline(always)]
    fn claim_data(&mut self, user: Key, claiming_address: Key) -> ClaimDataStruct {
        ClaimData::instance().get(&user, &claiming_address)
    }
    #[inline(always)]
    fn increase_allowance(&mut self, spender: Address, amount: U256) -> Result<(), Erc20Error> {
        let res = CURVEERC20::increase_allowance(self, spender, amount);
        self.emit(&REWARDONLYGAUGEEvent::Approval {
            owner: self.get_caller(),
            spender: Key::from(spender),
            value: CURVEERC20::allowance(self, Address::from(self.get_caller()), spender),
        });
        res
    }
    #[inline(always)]
    fn decrease_allowance(&mut self, spender: Address, amount: U256) -> Result<(), Erc20Error> {
        let res = CURVEERC20::decrease_allowance(self, spender, amount);
        self.emit(&REWARDONLYGAUGEEvent::Approval {
            owner: self.get_caller(),
            spender: Key::from(spender),
            value: CURVEERC20::allowance(self, Address::from(self.get_caller()), spender),
        });
        res
    }

    #[inline(always)]
    fn transfer_from(&mut self, _from: Address, _to: Address, _value: U256) -> Result<(), u32> {
        let lock = data::get_lock();
        if lock != 0 {
            //Reward Only Gauge: Locked
            runtime::revert(Error::RewardOnlyGaugeLocked1);
        }
        data::set_lock(1);
        let _allowance: U256 = CURVEERC20::allowance(self, _from, Address::from(self.get_caller()));
        if _allowance != U256::MAX {
            let new_allowance: U256 = _allowance
                .checked_sub(_value)
                .unwrap_or_revert_with(Error::RewardOnlyGaugeUnderFlow1);
            CURVEERC20::set_allowance(self, _from, Address::from(self.get_caller()), new_allowance);
        }
        self._transfer(Key::from(_from), Key::from(_to), _value);
        data::set_lock(0);
        Ok(())
    }

    #[inline(always)]
    fn _transfer(&mut self, _from: Key, _to: Key, _value: U256) {
        let reward_data: RewardData = self.reward_data();
        let _reward_contract = reward_data.address;
        if _value != 0.into() {
            let total_supply = CURVEERC20::total_supply(self);
            self._checkpoint_rewards(_from, total_supply, false, account_zero_address());
            let _from_balance: U256 = CURVEERC20::balance_of(self, Address::from(_from));
            let from_new_balance = _from_balance
                .checked_sub(_value)
                .unwrap_or_revert_with(Error::RewardOnlyGaugeUnderFlow2);
            CURVEERC20::set_balance(self, Address::from(_from), from_new_balance);
            self._checkpoint_rewards(_to, total_supply, false, account_zero_address());
            let _to_balance: U256 = CURVEERC20::balance_of(self, Address::from(_to));
            let to_new_balance = _to_balance
                .checked_add(_value)
                .unwrap_or_revert_with(Error::RewardOnlyGaugeUnderFlow3);
            CURVEERC20::set_balance(self, Address::from(_to), to_new_balance);
        }
        self.emit(&REWARDONLYGAUGEEvent::Transfer {
            from: _from,
            to: _to,
            value: _value,
        });
    }

    #[inline(always)]
    fn reward_data(&mut self) -> RewardData {
        data::reward_data()
    }
    #[inline(always)]
    fn claim_sig(&mut self) -> String {
        data::claim_sig()
    }

    #[inline(always)]
    fn commit_transfer_ownership(&mut self, addr: Key) {
        if self.get_caller() != self.admin() {
            //Reward Only Gauge Only Admin
            runtime::revert(Error::RewardOnlyGaugeOnlyAdmin1);
        }
        data::set_future_admin(addr);
        self.emit(&REWARDONLYGAUGEEvent::CommitOwnership { admin: addr });
    }

    #[inline(always)]
    fn accept_transfer_ownership(&mut self) {
        let _admin = self.future_admin();
        if self.get_caller() != _admin {
            //Reward Only Gauge Only Future Admin
            runtime::revert(Error::RewardOnlyGaugeOnlyFutureAdmin);
        }
        data::set_admin(_admin);
        self.emit(&REWARDONLYGAUGEEvent::ApplyOwnership { admin: _admin });
    }

    #[inline(always)]
    fn reward_contract(&mut self) -> Key {
        self.reward_data().address
    }
    #[inline(always)]
    fn last_claim(&mut self) -> U256 {
        self.reward_data().time_stamp
    }
    #[inline(always)]
    fn claimed_reward(&mut self, _addr: Key, _token: Key) -> U256 {
        self.claim_data(_addr, _token).claimed_amount
    }
    #[inline(always)]
    fn claimable_reward(&mut self, _addr: Key, _token: Key) -> U256 {
        self.claim_data(_addr, _token).claimable_amount
    }

    // lock
    #[inline(always)]
    fn claimable_reward_write(&mut self, _addr: Key, _token: Key) -> U256 {
        let lock = data::get_lock();
        if lock != 0 {
            //Reward Only Gauge: Locked
            runtime::revert(Error::RewardOnlyGaugeLocked1);
        }
        data::set_lock(1);
        let reward_token = self.reward_tokens(0.into());
        if reward_token != zero_address() && reward_token != account_zero_address() {
            let total_supply = self.total_supply();
            self._checkpoint_rewards(_addr, total_supply, false, account_zero_address());
        }
        data::set_lock(0);
        self.claim_data(_addr, _token).claimable_amount
    }

    // lock
    #[inline(always)]
    fn claim_rewards(&mut self, _addr: Option<Key>, _receiver: Option<Key>) {
        let lock = data::get_lock();
        if lock != 0 {
            //Reward Only Gauge: Locked
            runtime::revert(Error::RewardOnlyGaugeLocked1);
        }
        data::set_lock(1);
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
        if (receiver != zero_address() || receiver != account_zero_address())
            && addr != self.get_caller()
        {
            // Reward Only Gauge Cannot Redirect When Claiming For Another User
            runtime::revert(Error::RewardOnlyGaugeCannotRedirectWhenClaimingForAnotherUser);
        }
        let total_supply = CURVEERC20::total_supply(self);
        self._checkpoint_rewards(addr, total_supply, true, receiver);
        data::set_lock(0);
    }

    //lock
    #[inline(always)]
    fn withdraw(&mut self, _value: U256, _claim_rewards: Option<bool>) {
        let lock = data::get_lock();
        if lock != 0 {
            //Reward Only Gauge: Locked
            runtime::revert(Error::RewardOnlyGaugeLocked1);
        }
        data::set_lock(1);
        let claim_rewards: bool = !matches!(_claim_rewards, Some(..));
        if _value == 0.into() {
            // Reward Only Gauge Value Is Zero
            runtime::revert(Error::RewardOnlyGaugeValueIsZero1);
        }
        let reward_data: RewardData = self.reward_data();
        let _reward_contract: Key = reward_data.address;
        let total_supply = self.total_supply();
        self._checkpoint_rewards(
            self.get_caller(),
            total_supply,
            claim_rewards,
            account_zero_address(),
        );
        CURVEERC20::burn(self, Address::from(self.get_caller()), _value).unwrap_or_revert();
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
            runtime_args! {"recipient" => Address::from(self.get_caller()),"amount" => _value},
        );
        self.emit(&REWARDONLYGAUGEEvent::Withdraw {
            provider: self.get_caller(),
            value: _value,
        });
        self.emit(&REWARDONLYGAUGEEvent::Transfer {
            from: self.get_caller(),
            to: zero_address(),
            value: _value,
        });
        data::set_lock(0);
    }
    #[inline(always)]
    fn deposit(&mut self, _value: U256, _addr: Option<Key>, _claim_rewards: Option<bool>) {
        let lock = data::get_lock();
        if lock != 0 {
            //Reward Only Gauge: Locked
            runtime::revert(Error::RewardOnlyGaugeLocked1);
        }
        data::set_lock(1);
        let claim_rewards: bool = if let Some(..) = _claim_rewards {
            _claim_rewards.unwrap()
        } else {
            false
        };
        let addr: Key = if let Some(..) = _addr {
            _addr.unwrap()
        } else {
            self.get_caller()
        };
        if _value == 0.into() {
            // Reward Only Gauge Value Is Zero
            runtime::revert(Error::RewardOnlyGaugeValueIsZero2);
        }
        let reward_data: RewardData = self.reward_data();
        let _reward_contract: Key = reward_data.address;
        let total_supply = self.total_supply();
        self._checkpoint_rewards(addr, total_supply, claim_rewards, account_zero_address());
        CURVEERC20::mint(self, Address::from(self.get_caller()), _value).unwrap_or_revert();
        let lp_token = self.lp_token();
        let token_hash_add_array = match lp_token {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token_package_hash = ContractPackageHash::new(token_hash_add_array);
        let _result: () = runtime::call_versioned_contract(
            token_package_hash,
            None,
            "transfer_from",
            runtime_args! {"owner" => Address::from(self.get_caller()),"recipient" =>  Address::from(Key::from(data::get_package_hash())),"amount" => _value},
        );
        self.emit(&REWARDONLYGAUGEEvent::Deposit {
            provider: self.get_caller(),
            value: _value,
        });
        self.emit(&REWARDONLYGAUGEEvent::Transfer {
            from: self.get_caller(),
            to: zero_address(),
            value: _value,
        });
        data::set_lock(0);
    }
    #[inline(always)]
    fn set_rewards(
        &mut self,
        _reward_contract: Key,
        _claim_sig: String,
        _reward_tokens: Vec<String>,
    ) {
        let lock = data::get_lock();
        if lock != 0 {
            //Reward Only Gauge: Locked
            runtime::revert(Error::RewardOnlyGaugeLocked1);
        }
        data::set_lock(1);
        if self.get_caller() != self.admin() {
            runtime::revert(Error::RewardOnlyGaugeOnlyAdmin2);
        }
        let mut reward_tokens: Vec<Key> = Vec::new();
        for item in &_reward_tokens {
            reward_tokens.push(Key::from_formatted_str(item).unwrap());
        }
        let _lp_token = self.lp_token();
        let _current_reward_contract = self.reward_data().address;
        let total_supply = self.total_supply();
        self._checkpoint_rewards(
            account_zero_address(),
            total_supply,
            false,
            account_zero_address(),
        );
        if _reward_contract != zero_address() && _reward_contract != account_zero_address() {
            let reward_token = reward_tokens[0];
            if reward_token == zero_address() {
                //Reward Only Gauge Reward Token Is Zero Address
                runtime::revert(Error::RewardOnlyGaugeRewardTokenIsZeroAddress);
            }
            //is Contract Check is missing
        }
        let mut reward_data = self.reward_data();
        reward_data.address = _reward_contract;
        data::set_claim_sig(_claim_sig);
        for (i, item) in reward_tokens
            .iter()
            .enumerate()
            .take(MAX_REWARDS.as_usize())
        {
            let current_token = self.reward_tokens(i.into());
            let new_token: Key = *item;

            if current_token != zero_address() && current_token != account_zero_address() {
                if current_token != new_token {
                    //Reward Only Gauge Cannot Modify Existing Reward Token
                    runtime::revert(Error::RewardOnlyGaugeCannotModifyExistingRewardToken);
                }
            } else if new_token != zero_address() && new_token != account_zero_address() {
                RewardTokens::instance().set(&i.into(), new_token);
            } else {
                break;
            }
        }
        if _reward_contract != zero_address() && _reward_contract != account_zero_address() {
            // # do an initial checkpoint to verify that claims are working
            self._checkpoint_rewards(
                account_zero_address(),
                total_supply,
                false,
                account_zero_address(),
            )
        }
        data::set_lock(0);
    }

    #[inline(always)]
    fn set_rewards_receiver(&mut self, _receiver: Key) {
        RewardsReceiver::instance().set(&self.get_caller(), _receiver)
    }

    #[inline(always)]
    fn lp_token(&mut self) -> Key {
        data::lp_token()
    }
    #[inline(always)]
    fn admin(&mut self) -> Key {
        data::admin()
    }

    #[inline(always)]
    fn future_admin(&mut self) -> Key {
        data::future_admin()
    }

    /// @notice Claim pending rewards and checkpoint rewards for a user

    #[inline(always)]
    fn _checkpoint_rewards(
        &mut self,
        _user: Key,
        _total_supply: U256,
        _claim: bool,
        _receiver: Key,
    ) {
        let mut reward_data: RewardData = self.reward_data();
        if _total_supply != 0.into()
            && reward_data.address != zero_address()
            && reward_data.address != account_zero_address()
            && reward_data.time_stamp != 0.into()
            && U256::from(u64::from(runtime::get_blocktime()))
                > (reward_data.time_stamp + U256::from(CLAIM_FREQUENCY.as_u128()))
        {
            let reward_contract = reward_data.address;

            // raw_call(reward_contract, self.claim_sig)

            let reward_contract_hash_add_array = match reward_contract {
                Key::Hash(package) => package,
                _ => runtime::revert(ApiError::UnexpectedKeyVariant),
            };
            let reward_contract_package_hash =
                ContractPackageHash::new(reward_contract_hash_add_array);
            let () = runtime::call_versioned_contract(
                reward_contract_package_hash,
                None,
                "get_reward",
                runtime_args! {},
            );
            reward_data.address = reward_contract;
            reward_data.time_stamp = U256::from(u64::from(runtime::get_blocktime()))
        }
        let mut receiver = _receiver;
        if _claim && receiver == account_zero_address() && receiver == zero_address() {
            // if receiver is not explicitly declared, check for default receiver
            receiver = self.rewards_receiver(_user);
            if receiver == account_zero_address() || receiver == zero_address() {
                receiver = _user;
            }
        }
        let user_balance = CURVEERC20::balance_of(self, Address::from(_user));
        for i in 0..(MAX_REWARDS.as_usize()) {
            let token = self.reward_tokens(i.into());
            if token == zero_address() || token == account_zero_address() {
                break;
            }
            let mut d_i: U256 = 0.into();
            if _total_supply != 0.into() {
                let token_hash_add_array = match token {
                    Key::Hash(package) => package,
                    _ => runtime::revert(ApiError::UnexpectedKeyVariant),
                };
                let token_package_hash = ContractPackageHash::new(token_hash_add_array);
                let token_balance: U256 = runtime::call_versioned_contract(
                    token_package_hash,
                    None,
                    "balance_of",
                    runtime_args! {"address" => Address::from(data::get_package_hash())},
                );
                d_i = U256::from(1000000000)
                    * (token_balance
                        .checked_sub(self.reward_balances(token))
                        .unwrap_or_revert_with(Error::RewardOnlyGaugeUnderFlow4)
                        / _total_supply);
                RewardBalances::instance().set(&token, token_balance);
                if (_user == zero_address() || _user == account_zero_address()) && d_i != 0.into() {
                    let reward_integral = self.reward_integral(token);
                    RewardIntegral::instance().set(
                        &token,
                        reward_integral
                            .checked_add(d_i)
                            .unwrap_or_revert_with(Error::RewardOnlyGaugeOverFlow1),
                    )
                }
            }
            let integral = self
                .reward_integral(token)
                .checked_add(d_i)
                .unwrap_or_revert_with(Error::RewardOnlyGaugeOverFlow2);
            if d_i != 0.into() {
                RewardIntegral::instance().set(&token, integral);
            }
            let integral_for: U256 = self.reward_integral_for(token, _user);
            let mut new_claimable: U256 = 0.into();
            if integral_for < integral {
                RewardIntegralFor::instance().set(&token, &_user, integral);
                new_claimable = user_balance
                    * (integral
                        .checked_sub(integral_for)
                        .unwrap_or_revert_with(Error::RewardOnlyGaugeUnderFlow5))
                    / U256::from(1000000000);
            }
            let mut claim_data: ClaimDataStruct = self.claim_data(_user, token);
            let total_claimable: U256 = claim_data
                .claimable_amount
                .checked_add(new_claimable)
                .unwrap_or_revert_with(Error::RewardOnlyGaugeOverFlow3);
            if total_claimable > 0.into() {
                let total_claimed = claim_data.claimed_amount;
                if _claim {
                    let token_hash_add_array = match token {
                        Key::Hash(package) => package,
                        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
                    };
                    let token_package_hash = ContractPackageHash::new(token_hash_add_array);
                    let _res: () = runtime::call_versioned_contract(
                        token_package_hash,
                        None,
                        "transfer",
                        runtime_args! {"to" => Address::from(receiver),"amount" => total_claimable},
                    );
                    let latest_total_claimable = self
                        .reward_balances(token)
                        .checked_sub(total_claimable)
                        .unwrap_or_revert_with(Error::RewardOnlyGaugeUnderFlow6);
                    RewardBalances::instance().set(&token, latest_total_claimable);
                    claim_data.claimed_amount = total_claimed
                        .checked_add(total_claimable)
                        .unwrap_or_revert_with(Error::RewardOnlyGaugeOverFlow4);
                    ClaimData::instance().set(&_user, &token, claim_data);
                } else if new_claimable > 0.into() {
                    claim_data.claimed_amount = total_claimed;
                    claim_data.claimable_amount = total_claimable;
                    ClaimData::instance().set(&_user, &token, claim_data);
                }
            }
        }
    }
    #[inline(always)]
    fn named_keys(&self) -> Result<BTreeMap<String, Key>, Erc20Error> {
        CURVEERC20::named_keys(self, "".to_string(), "".to_string(), 9, 0.into())
    }
    fn emit(&mut self, reward_only_gauge_event: &REWARDONLYGAUGEEvent) {
        match reward_only_gauge_event {
            REWARDONLYGAUGEEvent::Withdraw { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", reward_only_gauge_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            REWARDONLYGAUGEEvent::Deposit { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", reward_only_gauge_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            REWARDONLYGAUGEEvent::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", reward_only_gauge_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            REWARDONLYGAUGEEvent::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", reward_only_gauge_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            REWARDONLYGAUGEEvent::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", reward_only_gauge_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("spender", spender.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            REWARDONLYGAUGEEvent::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", reward_only_gauge_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
        };
    }

    fn get_package_hash(&mut self) -> ContractPackageHash {
        data::get_package_hash()
    }
}
