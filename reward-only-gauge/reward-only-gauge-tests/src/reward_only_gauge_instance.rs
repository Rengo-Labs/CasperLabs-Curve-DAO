use std::collections::BTreeMap;

use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    account::AccountHash,
    bytesrepr::{Bytes, ToBytes},
    runtime_args, CLTyped, ContractPackageHash, Key, RuntimeArgs, U256,
};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use casperlabs_test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes)]
pub struct ClaimDataStruct {
    pub claimable_amount: U256,
    pub claimed_amount: U256,
}

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes)]
pub struct RewardData {
    pub address: Key,
    pub time_stamp: U256,
}
pub struct REWARDONLYGAUGEInstance(TestContract);
//#[clippy::must_use]
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
        _supply: U256,
    ) -> TestContract {
        TestContract::new(
            env,
            "erc20_crv.wasm",
            "proxy_test2",
            sender,
            runtime_args! {
                "name" => name,
                "symbol" => symbol,
                "decimals" => decimals,
            },
            100000000,
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
            0,
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
            0,
        );
    }

    pub fn transfer<T: Into<Key>>(&self, sender: AccountHash, recipient: T, amount: U256) {
        self.0.call_contract(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient.into(),
                "amount" => amount
            },
            0,
        );
    }

    pub fn transfer_from(&self, sender: AccountHash, owner: Key, recipient: Key, amount: U256) {
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

    pub fn approve<T: Into<Key>>(&self, sender: AccountHash, spender: T, amount: U256) {
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

    pub fn increase_allowance<T: Into<Key>>(&self, sender: AccountHash, spender: T, amount: U256) {
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

    // pub fn allowance_fn(&self, sender: AccountHash, owner: Key, spender: Key) {
    //     self.0.call_contract(
    //         sender,
    //         "allowance",
    //         runtime_args! {
    //             "owner" => owner,
    //             "spender" => spender,
    //         },
    //         0,
    //     );
    // }

    pub fn decrease_allowance<T: Into<Key>>(&self, sender: AccountHash, spender: T, amount: U256) {
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

    // pub fn mint<T: Into<Key>>(&self, sender: AccountHash, to: T, amount: U256) {
    //     self.0.call_contract(
    //         sender,
    //         "mint",
    //         runtime_args! {
    //             "to" => to.into(),
    //             "amount" => amount
    //         },
    //         0,
    //     );
    // }
    pub fn accept_transfer_ownership(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "accept_transfer_ownership", runtime_args! {}, 0);
    }
    pub fn commit_transfer_ownership<T: Into<Key>>(&self, sender: AccountHash, addr: T) {
        self.0.call_contract(
            sender,
            "commit_transfer_ownership",
            runtime_args! {
                "addr" => addr.into(),
            },
            0,
        );
    }
    pub fn set_rewards_receiver<T: Into<Key>>(&self, sender: AccountHash, _receiver: T) {
        self.0.call_contract(
            sender,
            "set_rewards_receiver",
            runtime_args! {
                "_receiver" => _receiver.into(),
            },
            0,
        );
    }

    pub fn balance_of<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary("balances", key_to_str(&account.into()))
            .unwrap_or_default()
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
    pub fn reward_tokens<T: Into<Key>>(&self, account: T) -> Key {
        self.0
            .query_dictionary("reward_tokens", key_to_str(&account.into()))
            .unwrap()
    }

    pub fn allowance<T: Into<Key>>(&self, owner: T, spender: T) -> U256 {
        let owner: Key = owner.into();
        let spender: Key = spender.into();
        self.0
            .query_dictionary("allowances", keys_to_str(&owner, &spender))
            .unwrap_or_default()
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
    // pub fn allowance_package_hash<T: Into<Key>>(
    //     &self,
    //     owner: ContractPackageHash,
    //     spender: T,
    // ) -> U256 {
    //     let owner: Key = owner.into();
    //     let spender: Key = spender.into();
    //     self.0
    //         .query_dictionary("allowances", keys_to_str(&owner, &spender))
    //         .unwrap_or_default()
    // }

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
    pub fn claim_sig(&self) -> Bytes {
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

    // // Result methods
    // pub fn transfer_result(&self) -> Result<(), u32> {
    //     self.0.query_named_key("transfer_result".to_string())
    // }

    // pub fn package_hash_result(&self) -> ContractPackageHash {
    //     self.0.query_named_key("package_hash".to_string())
    // }

    // pub fn transfer_from_result(&self) -> Result<(), u32> {
    //     self.0.query_named_key("transfer_from_result".to_string())
    // }
    // pub fn allowance_res(&self) -> U256 {
    //     self.0.query_named_key("allowance".to_string())
    // }

    // pub fn increase_allowance_res(&self) -> Result<(), u32> {
    //     self.0
    //         .query_named_key("increase_allowance_result".to_string())
    // }
    // pub fn decrease_allowance_res(&self) -> Result<(), u32> {
    //     self.0
    //         .query_named_key("decrease_allowance_result".to_string())
    // }

    // pub fn meta(&self) -> Meta {
    //     self.0.query_named_key(String::from("meta"))
    // }
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
