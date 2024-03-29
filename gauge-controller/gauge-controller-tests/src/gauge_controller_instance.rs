use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, CLTyped, ContractPackageHash, Key, RuntimeArgs, U128, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};
use std::time::SystemTime;

pub struct GAUGECONLTROLLERInstance(TestContract);
#[allow(clippy::too_many_arguments)]
impl GAUGECONLTROLLERInstance {
    pub fn instance(gauge_controller: TestContract) -> GAUGECONLTROLLERInstance {
        GAUGECONLTROLLERInstance(gauge_controller)
    }
    pub fn deploy_voting_escrow(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        token_addr: Key,
        name: String,
        symbol: String,
        version: String,
        block_time: u64,
    ) -> TestContract {
        TestContract::new(
            env,
            "voting-escrow.wasm",
            contract_name,
            sender,
            runtime_args! {
                "token_addr" => token_addr,
                "name" => name,
                "symbol" => symbol,
                "version" => version,
            },
            block_time,
        )
    }
    pub fn deploy_erc20(
        env: &TestEnv,
        sender: AccountHash,
        name: &str,
        symbol: &str,
        decimals: u8,
        supply: U256,
        block_time: u64,
    ) -> TestContract {
        TestContract::new(
            env,
            "curve-erc20.wasm",
            "proxy_test2",
            sender,
            runtime_args! {
                "initial_supply" => supply,
                "name" => name,
                "symbol" => symbol,
                "decimals" => decimals
            },
            block_time,
        )
    }
    pub fn deploy_liquidity_gauge(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        lp_addr: Key,
        minter: Key,
        admin: Key,
        block_time: u64,
    ) -> TestContract {
        TestContract::new(
            env,
            "liquidity-gauge-v3.wasm",
            contract_name,
            sender,
            runtime_args! {
                "lp_addr" => lp_addr,
                "minter" => minter,
                "admin" => admin,
            },
            block_time,
        )
    }
    pub fn deploy_erc20_crv(env: &TestEnv, sender: AccountHash, block_time: u64) -> TestContract {
        TestContract::new(
            env,
            "erc20-crv.wasm",
            "erc20-crv",
            sender,
            runtime_args! {
                "name" => "CRV",
                "symbol" => "ERC20CRV",
                "decimals" => 9_u8,
            },
            block_time,
        )
    }
    pub fn minter(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        token: Key,
        controller: Key,
        block_time: u64,
    ) -> TestContract {
        TestContract::new(
            env,
            "minter-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "controller" => controller,
                "token" => token,
            },
            block_time,
        )
    }
    pub fn new_deploy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        token: Key,
        voting_escrow: Key,
        block_time: u64,
    ) -> TestContract {
        TestContract::new(
            env,
            "gauge-controller-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "voting_escrow" => voting_escrow,
                "token" => token,
            },
            block_time,
        )
    }

    pub fn constructor(
        &self,
        sender: AccountHash,
        name: &str,
        token: Key,
        controller: Key,
        reward_count: U256,
        block_time: u64,
    ) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
                "controller" => controller,
                "name" => name,
                "token" => token,
                "reward_count" => reward_count
            },
            block_time,
        );
    }

    pub fn commit_transfer_ownership<T: Into<Key>>(
        &self,
        sender: AccountHash,
        addr: T,
        block_time: u64,
    ) {
        self.0.call_contract(
            sender,
            "commit_transfer_ownership",
            runtime_args! {
                "addr" => addr.into(),
            },
            block_time,
        );
    }
    pub fn apply_transfer_ownership(&self, sender: AccountHash, block_time: u64) {
        self.0.call_contract(
            sender,
            "apply_transfer_ownership",
            runtime_args! {},
            block_time,
        );
    }

    pub fn checkpoint(&self, sender: AccountHash, block_time: u64) {
        self.0
            .call_contract(sender, "checkpoint", runtime_args! {}, block_time);
    }
    pub fn checkpoint_gauge<T: Into<Key>>(&self, sender: AccountHash, addr: T, block_time: u64) {
        self.0.call_contract(
            sender,
            "checkpoint_gauge",
            runtime_args! {
                "addr" => addr.into(),
            },
            block_time,
        );
    }

    pub fn change_type_weight(
        &self,
        sender: AccountHash,
        type_id: (bool, U128),
        weight: U256,
        block_time: u64,
    ) {
        self.0.call_contract(
            sender,
            "change_type_weight",
            runtime_args! {
                "type_id" => type_id,
                "weight" => weight,
            },
            block_time,
        );
    }
    pub fn change_gauge_weight<T: Into<Key>>(
        &self,
        sender: AccountHash,
        addr: T,
        weight: U256,
        block_time: u64,
    ) {
        self.0.call_contract(
            sender,
            "change_gauge_weight",
            runtime_args! {
                "addr" => addr.into(),
                "weight" => weight,
            },
            block_time,
        );
    }
    pub fn add_type(
        &self,
        sender: AccountHash,
        _name: String,
        weight: Option<U256>,
        block_time: u64,
    ) {
        self.0.call_contract(
            sender,
            "add_type",
            runtime_args! {
                "name" => _name,
                "weight" => weight
            },
            block_time,
        );
    }
    pub fn add_gauge<T: Into<Key>>(
        &self,
        sender: AccountHash,
        addr: T,
        gauge_type: (bool, U128),
        weight: Option<U256>,
        block_time: u64,
    ) {
        self.0.call_contract(
            sender,
            "add_gauge",
            runtime_args! {
                "addr" => addr.into(),
                "gauge_type" => gauge_type,
                "weight"=>weight
            },
            block_time,
        );
    }
    pub fn vote_for_gauge_weights<T: Into<Key>>(
        &self,
        sender: AccountHash,
        _gauge_addr: T,
        _user_weight: U256,
        block_time: u64,
    ) {
        self.0.call_contract(
            sender,
            "vote_for_gauge_weights",
            runtime_args! {
                "gauge_addr" => _gauge_addr.into(),
                "user_weight" => _user_weight,
            },
            block_time,
        );
    }

    pub fn gauge_type_names<T: Into<Key>>(&self, owner: U128) -> String {
        self.0
            .query_dictionary("gauge_type_names", owner.to_string())
            .unwrap_or_default()
    }

    pub fn gauge_types_<T: Into<Key>>(&self, owner: T) -> U256 {
        self.0
            .query_dictionary("gauge_types_", key_to_str(&owner.into()))
            .unwrap_or_default()
    }
    pub fn gauges<T: Into<Key>>(&self, owner: U256) -> Key {
        self.0
            .query_dictionary("gauges", owner.to_string())
            .unwrap_or_revert()
    }
    pub fn points_total<T: Into<Key>>(&self, owner: U256) -> U256 {
        self.0
            .query_dictionary("points_total", owner.to_string())
            .unwrap_or_default()
    }
    pub fn time_sum<T: Into<Key>>(&self, type_id: U256) -> U256 {
        self.0
            .query_dictionary("time_sum", type_id.to_string())
            .unwrap_or_default()
    }
    pub fn time_type_weight<T: Into<Key>>(&self, type_id: U256) -> U256 {
        self.0
            .query_dictionary("time_type_weight", type_id.to_string())
            .unwrap_or_default()
    }
    pub fn time_weight<T: Into<Key>>(&self, owner: T) -> U256 {
        self.0
            .query_dictionary("time_weight", key_to_str(&owner.into()))
            .unwrap_or_default()
    }
    pub fn vote_user_power<T: Into<Key>>(&self, owner: T) -> U256 {
        self.0
            .query_dictionary("vote_user_power", key_to_str(&owner.into()))
            .unwrap_or_default()
    }

    pub fn change_sum(&self, owner: U128, spender: U256) -> U256 {
        self.0
            .query_dictionary(
                "change_sum",
                values_to_str(&U256::from(owner.as_u128()), &spender),
            )
            .unwrap_or_default()
    }
    pub fn changes_weight(&self, owner: Key, spender: U256) -> U256 {
        self.0
            .query_dictionary("changes_weight", key_and_value_to_str(&owner, &spender))
            .unwrap_or_default()
    }
    pub fn last_user_vote(&self, owner: Key, spender: Key) -> U256 {
        self.0
            .query_dictionary("last_user_vote", keys_to_str(&owner, &spender))
            .unwrap_or_default()
    }
    pub fn points_sum(&self, owner: U128, spender: U256) -> U256 {
        self.0
            .query_dictionary(
                "points_sum",
                values_to_str(&U256::from(owner.as_u128()), &spender),
            )
            .unwrap_or_default()
    }
    pub fn points_type_weight(&self, owner: U128, spender: U256) -> U256 {
        self.0
            .query_dictionary(
                "points_type_weight",
                values_to_str(&U256::from(owner.as_u128()), &spender),
            )
            .unwrap_or_default()
    }
    pub fn points_weight(&self, owner: Key, spender: U256) -> U256 {
        self.0
            .query_dictionary("points_weight", key_and_value_to_str(&owner, &spender))
            .unwrap_or_default()
    }
    pub fn vote_user_slopes(&self, owner: Key, spender: Key) -> U256 {
        self.0
            .query_dictionary("vote_user_slopes", keys_to_str(&owner, &spender))
            .unwrap_or_default()
    }

    pub fn n_gauge_types(&self) -> (bool, U128) {
        self.0.query_named_key(String::from("n_gauge_types"))
    }
    pub fn n_gauges(&self) -> (bool, U128) {
        self.0.query_named_key(String::from("n_gauges"))
    }
    pub fn token(&self) -> Key {
        self.0.query_named_key(String::from("token"))
    }
    pub fn admin(&self) -> Key {
        self.0.query_named_key(String::from("admin"))
    }
    pub fn future_admin(&self) -> Key {
        self.0.query_named_key(String::from("future_admin"))
    }
    pub fn voting_escrow(&self) -> Key {
        self.0.query_named_key(String::from("voting_escrow"))
    }
    pub fn contract_package_hash(&self) -> ContractPackageHash {
        self.0
            .query_named_key(String::from("self_contract_package_hash"))
    }
    pub fn contract_hash(&self) -> Key {
        self.0.query_named_key(String::from("self_contract_hash"))
    }
    pub fn time_total(&self) -> U256 {
        self.0.query_named_key(String::from("time_total"))
    }
    pub fn key_value<T: CLTyped + FromBytes>(&self, key: String) -> T {
        self.0.query_named_key(key)
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

pub fn values_to_str<T: CLTyped + ToBytes>(value_a: &T, value_b: &T) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(value_a.to_bytes().unwrap());
    hasher.update(value_b.to_bytes().unwrap());
    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
    hex::encode(ret)
}
