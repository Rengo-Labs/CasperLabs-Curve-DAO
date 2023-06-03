use std::{collections::BTreeMap, time::SystemTime};

use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};

use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, CLTyped, Key, RuntimeArgs, U256,
};

use casperlabs_test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub const ALLOWANCES: &str = "allowances";
use crv20::Address;
use hex::encode;

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
pub struct LIQUIDITYGUAGEV4INSTANCEInstance(TestContract);

impl LIQUIDITYGUAGEV4INSTANCEInstance {
    pub fn instance(liquidity_gauge_v4: TestContract) -> LIQUIDITYGUAGEV4INSTANCEInstance {
        LIQUIDITYGUAGEV4INSTANCEInstance(liquidity_gauge_v4)
    }

    pub fn new_deploy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        lp_addr: Key,
        minter: Key,
        admin: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "liquidity-gauge-v4.wasm",
            contract_name,
            sender,
            runtime_args! {
                "lp_addr" => lp_addr,
                "minter"=>minter,
                "admin" => admin,
            },
            LIQUIDITYGUAGEV4INSTANCEInstance::now(),
        )
    }
    pub fn commit_transfer_ownership(&self, sender: AccountHash, addr: Key, time_now: u64) {
        self.0.call_contract(
            sender,
            "commit_transfer_ownership",
            runtime_args! {
                "addr" => addr,
            },
            time_now,
        );
    }
    pub fn accept_transfer_ownership(&self, sender: AccountHash, time_now: u64) {
        self.0.call_contract(
            sender,
            "accept_transfer_ownership",
            runtime_args! {},
            time_now,
        );
    }
    pub fn set_killed(&self, sender: AccountHash, is_killed: bool, time_now: u64) {
        self.0.call_contract(
            sender,
            "set_killed",
            runtime_args! {
                "is_killed"=>is_killed,
            },
            time_now,
        );
    }
    pub fn approve(&self, sender: AccountHash, spender: Address, amount: U256, time_now: u64) {
        self.0.call_contract(
            sender,
            "approve",
            runtime_args! {
                "spender" => spender,
                "amount" => amount

            },
            time_now,
        );
    }
    pub fn increase_allowance(
        &self,
        sender: AccountHash,
        spender: Address,
        amount: U256,
        time_now: u64,
    ) {
        self.0.call_contract(
            sender,
            "increase_allowance",
            runtime_args! {
                "spender" => spender,
                "amount" => amount,

            },
            time_now,
        );
    }
    pub fn decrease_allowance(
        &self,
        sender: AccountHash,
        spender: Address,
        amount: U256,
        time_now: u64,
    ) {
        self.0.call_contract(
            sender,
            "decrease_allowance",
            runtime_args! {
                "spender" => spender,
                "amount" => amount

            },
            time_now,
        );
    }
    pub fn decimals(&self, sender: AccountHash, time_now: u64) {
        self.0
            .call_contract(sender, "decimals", runtime_args! {}, time_now);
    }
    pub fn integrate_checkpoint(&self, sender: AccountHash, time_now: u64) {
        self.0
            .call_contract(sender, "integrate_checkpoint", runtime_args! {}, time_now);
    }
    pub fn deposit(
        &self,
        sender: AccountHash,
        value: U256,
        addr: Option<Key>,
        claim_rewards: Option<bool>,
        time_now: u64,
    ) {
        self.0.call_contract(
            sender,
            "deposit",
            runtime_args! {
                "value" => value,
                "addr" => addr,
                "claim_rewards" => claim_rewards,
            },
            time_now,
        );
    }
    pub fn withdraw(
        &self,
        sender: AccountHash,
        value: U256,
        claim_rewards: Option<bool>,
        time_now: u64,
    ) {
        self.0.call_contract(
            sender,
            "withdraw",
            runtime_args! {
                "value" => value,
                "claim_rewards" => claim_rewards,
            },
            time_now,
        );
    }
    pub fn transfer(&self, sender: AccountHash, recipient: Address, amount: U256, time_now: u64) {
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
        owner: Address,
        recipient: Address,
        amount: U256,
        time_now: u64,
    ) {
        self.0.call_contract(
            sender,
            "transfer_from",
            runtime_args! {
                "owner" => owner,
                "recipient" => recipient,
                "amount" => amount

            },
            time_now,
        );
    }
    pub fn set_rewards_receiver(&self, sender: AccountHash, receiver: Key, time_now: u64) {
        self.0.call_contract(
            sender,
            "set_rewards_receiver",
            runtime_args! {
                "receiver" => receiver

            },
            time_now,
        );
    }
    pub fn claim_rewards(
        &self,
        sender: AccountHash,
        addr: Option<Key>,
        receiver: Option<Key>,
        time_now: u64,
    ) {
        self.0.call_contract(
            sender,
            "claim_rewards",
            runtime_args! {
                "addr" => addr,
                "receiver" => receiver,
            },
            time_now,
        );
    }
    pub fn set_rewards(
        &self,
        sender: AccountHash,
        reward_contract: Key,
        sigs: String,
        reward_tokens: Vec<String>,
        time_now: u64,
    ) {
        self.0.call_contract(
            sender,
            "set_rewards",
            runtime_args! {
                "reward_contract" => reward_contract,
                "sigs" => sigs,
                "reward_tokens" => reward_tokens,

            },
            time_now,
        );
    }

    //var
    pub fn future_admin(&self) -> Key {
        self.0.query_named_key(String::from("future_admin"))
    }
    pub fn admin(&self) -> Key {
        self.0.query_named_key(String::from("admin"))
    }
    pub fn is_killed(&self) -> bool {
        self.0.query_named_key(String::from("is_killed"))
    }
    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        let ret: U256 = self.0.query(ALLOWANCES, addresses_to_str(owner, spender));
        ret
    }
    pub fn name(&self) -> String {
        self.0.query_named_key(String::from("name"))
    }
    pub fn symbol(&self) -> String {
        self.0.query_named_key(String::from("symbol"))
    }
    pub fn future_epoch_time(&self) -> U256 {
        self.0.query_named_key(String::from("future_epoch_time"))
    }
    pub fn inflation_rate(&self) -> U256 {
        self.0.query_named_key(String::from("inflation_rate"))
    }
    pub fn total_supply(&self) -> U256 {
        self.0.query_named_key(String::from("total_supply"))
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
    pub fn package_hash(&self) -> [u8; 32] {
        self.0.package_hash()
    }

    // Get stored key values
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