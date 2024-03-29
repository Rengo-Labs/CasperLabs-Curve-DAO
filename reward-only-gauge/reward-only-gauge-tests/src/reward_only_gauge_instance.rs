use std::{collections::BTreeMap, time::SystemTime};

use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use common::keys::*;
pub const ALLOWANCES: &str = "allowances";
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    account::AccountHash, bytesrepr::ToBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use casperlabs_test_env::{TestContract, TestEnv};
use crv20::Address;
use hex::encode;

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct ClaimDataStruct {
    pub claimable_amount: U256,
    pub claimed_amount: U256,
}

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes)]
pub struct RewardData {
    pub address: Key,
    pub time_stamp: U256,
}

pub fn address_to_str(owner: &Address) -> String {
    let preimage = owner.to_bytes().unwrap();
    base64::encode(&preimage)
}

pub fn addresses_to_str(owner: Address, spender: Address) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(owner.to_bytes().unwrap());
    hasher.update(spender.to_bytes().unwrap());

    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));

    encode(ret)
}

pub struct REWARDONLYGAUGEInstance(TestContract);
#[allow(clippy::too_many_arguments)]
impl REWARDONLYGAUGEInstance {
    pub fn instance(reward_only_gauge: TestContract) -> REWARDONLYGAUGEInstance {
        REWARDONLYGAUGEInstance(reward_only_gauge)
    }

    pub fn erc20_crv(
        env: &TestEnv,
        sender: AccountHash,
        name: &str,
        symbol: &str,
        decimals: u8,
    ) -> TestContract {
        TestContract::new(
            env,
            "erc20-crv.wasm",
            "proxy_test2",
            sender,
            runtime_args! {
                "name" => name,
                "symbol" => symbol,
                "decimals" => decimals,
            },
            REWARDONLYGAUGEInstance::now(),
        )
    }

    pub fn curve_rewards(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        token: Key,
        reward: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "curve-rewards.wasm",
            contract_name,
            sender,
            runtime_args! {
                "token" => token,
                "reward" => reward
            },
            REWARDONLYGAUGEInstance::now(),
        )
    }

    pub fn new_deploy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        admin: Key,
        lp_token: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "reward-only-gauge-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "lp_token" => lp_token,
                "admin" => admin,
            },
            REWARDONLYGAUGEInstance::now(),
        )
    }

    pub fn constructor(&self, sender: AccountHash, name: &str, admin: Key, lp_token: Key) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
                "admin" => admin,
                "name" => name,
                "lp_token" => lp_token,
            },
            REWARDONLYGAUGEInstance::now(),
        );
    }

    pub fn transfer(&self, sender: AccountHash, time_now: u64, recipient: Address, amount: U256) {
        self.0.call_contract(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient,
                "amount" => amount
            },
            time_now,
        );
    }

    pub fn transfer_from(
        &self,
        sender: AccountHash,
        _time_now: u64,
        owner: Address,
        recipient: Address,
        amount: U256,
    ) {
        self.0.call_contract(
            sender,
            "transfer_from",
            runtime_args! {
                "owner" => owner,
                "recipient" => recipient,
                "amount" => amount
            },
            0,
        );
    }

    pub fn approve<T: Into<Key>>(
        &self,
        sender: AccountHash,
        _time_now: u64,
        spender: T,
        amount: U256,
    ) {
        self.0.call_contract(
            sender,
            "approve",
            runtime_args! {
                "spender" => spender.into(),
                "amount" => amount
            },
            0,
        );
    }

    pub fn increase_allowance<T: Into<Key>>(
        &self,
        sender: AccountHash,
        _time_now: u64,
        spender: T,
        amount: U256,
    ) {
        self.0.call_contract(
            sender,
            "increase_allowance",
            runtime_args! {
                "spender" => spender.into(),
                "amount" => amount
            },
            0,
        );
    }

    pub fn decrease_allowance<T: Into<Key>>(
        &self,
        sender: AccountHash,
        _time_now: u64,
        spender: T,
        amount: U256,
    ) {
        self.0.call_contract(
            sender,
            "decrease_allowance",
            runtime_args! {
                "spender" => spender.into(),
                "amount" => amount
            },
            0,
        );
    }

    pub fn accept_transfer_ownership(&self, sender: AccountHash, _time_now: u64) {
        self.0
            .call_contract(sender, "accept_transfer_ownership", runtime_args! {}, 0);
    }
    pub fn commit_transfer_ownership<T: Into<Key>>(
        &self,
        sender: AccountHash,
        time_now: u64,
        addr: T,
    ) {
        self.0.call_contract(
            sender,
            "commit_transfer_ownership",
            runtime_args! {
                "addr" => addr.into(),
            },
            time_now,
        );
    }
    pub fn set_rewards_receiver<T: Into<Key>>(
        &self,
        sender: AccountHash,
        time_now: u64,
        _receiver: T,
    ) {
        self.0.call_contract(
            sender,
            "set_rewards_receiver",
            runtime_args! {
                "receiver" => _receiver.into(),
            },
            time_now,
        );
    }
    pub fn set_rewards<T: Into<Key>>(
        &self,
        sender: AccountHash,
        time_now: u64,
        _reward_contract: T,
        _claim_sig: String,
        _reward_tokens: Vec<String>,
    ) {
        self.0.call_contract(
            sender,
            "set_rewards",
            runtime_args! {
                "reward_contract" => _reward_contract.into(),
                "claim_sig" => _claim_sig,
                "reward_tokens" => _reward_tokens,
            },
            time_now,
        );
    }
    pub fn claim_rewards(
        &self,
        sender: AccountHash,
        time_now: u64,
        _addr: Option<Key>,
        _receiver: Option<Key>,
    ) {
        self.0.call_contract(
            sender,
            "claim_rewards",
            runtime_args! {
                "addr" => _addr,
                "receiver" => _receiver,
            },
            time_now,
        );
    }

    pub fn deposit(
        &self,
        sender: AccountHash,
        time_now: u64,
        _value: U256,
        _addr: Option<Key>,
        _claim_rewards: Option<bool>,
    ) {
        self.0.call_contract(
            sender,
            "deposit",
            runtime_args! {
                "value" => _value,
                "addr" => _addr,
                "claim_rewards" => _claim_rewards,
            },
            time_now,
        );
    }

    pub fn withdraw(
        &self,
        sender: AccountHash,
        time_now: u64,
        _value: U256,
        _claim_rewards: Option<bool>,
    ) {
        self.0.call_contract(
            sender,
            "withdraw",
            runtime_args! {
                "value" => _value,
                "claim_rewards" => _claim_rewards,
            },
            time_now,
        );
    }

    pub fn balance_of(&self, owner: Address) -> U256 {
        self.0.query(BALANCES, address_to_str(&owner))
    }

    pub fn reward_balances<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary("reward_balances", key_to_str(&account.into()))
            .unwrap_or_default()
    }
    pub fn rewards_receiver<T: Into<Key>>(&self, account: T) -> Key {
        self.0
            .query_dictionary("reward_receiver", key_to_str(&account.into()))
            .unwrap()
    }
    pub fn reward_integral<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary("reward_integral", key_to_str(&account.into()))
            .unwrap_or_default()
    }
    pub fn reward_tokens(&self, index: U256) -> Key {
        self.0
            .query_dictionary("reward_tokens", (&index).to_string())
            .unwrap()
    }
    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        let ret: U256 = self.0.query(ALLOWANCES, addresses_to_str(owner, spender));
        ret
    }

    pub fn reward_integral_for<T: Into<Key>>(&self, owner: T, spender: T) -> U256 {
        let owner: Key = owner.into();
        let spender: Key = spender.into();
        self.0
            .query_dictionary("reward_integral_for", keys_to_str(&owner, &spender))
            .unwrap_or_default()
    }
    pub fn claim_data<T: Into<Key>>(&self, owner: T, spender: T) -> ClaimDataStruct {
        let owner: Key = owner.into();
        let spender: Key = spender.into();
        self.0
            .query_dictionary("claim_data", keys_to_str(&owner, &spender))
            .unwrap_or_revert()
    }

    pub fn name(&self) -> String {
        self.0.query_named_key(String::from("name"))
    }

    pub fn symbol(&self) -> String {
        self.0.query_named_key(String::from("symbol"))
    }

    pub fn decimals(&self) -> u8 {
        self.0.query_named_key(String::from("decimals"))
    }

    pub fn total_supply(&self) -> U256 {
        self.0.query_named_key(String::from("total_supply"))
    }
    pub fn claim_sig(&self) -> String {
        self.0.query_named_key(String::from("claim_sig"))
    }

    pub fn contract_package_hash(&self) -> ContractPackageHash {
        self.0
            .query_named_key(String::from("self_contract_package_hash"))
    }
    pub fn contract_hash(&self) -> Key {
        self.0.query_named_key(String::from("self_contract_hash"))
    }
    pub fn reward_data(&self) -> RewardData {
        self.0.query_named_key(String::from("reward_data"))
    }
    pub fn admin(&self) -> Key {
        self.0.query_named_key(String::from("admin"))
    }
    pub fn future_admin(&self) -> Key {
        self.0.query_named_key(String::from("future_admin"))
    }
    pub fn lp_token(&self) -> Key {
        self.0.query_named_key(String::from("lp_token"))
    }
    pub fn now() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}

pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => panic!("Unexpected key type"),
    }
}

pub fn keys_to_str(key_a: &Key, key_b: &Key) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(key_a.to_bytes().unwrap());
    hasher.update(key_b.to_bytes().unwrap());
    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
    hex::encode(ret)
}

pub fn key_and_value_to_str<T: CLTyped + ToBytes>(key: &Key, value: &T) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(key.to_bytes().unwrap());
    hasher.update(value.to_bytes().unwrap());
    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
    hex::encode(ret)
}
