use std::time::SystemTime;
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};

pub struct LIQUIDITYGAUGEWRAPPERInstance(TestContract);
#[allow(clippy::too_many_arguments)]
impl LIQUIDITYGAUGEWRAPPERInstance {
    pub fn contract_instance(contract: TestContract) -> LIQUIDITYGAUGEWRAPPERInstance {
        LIQUIDITYGAUGEWRAPPERInstance(contract)
    }
    pub fn new_deploy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        name: String,
        symbol: String,
        gauge: Key,
        admin: Key,
        block_time:u64
    ) -> TestContract {
        TestContract::new(
            env,
            "liquidity-gauge-wrapper.wasm",
            contract_name,
            sender,
            runtime_args! {
                "name" => name,
                "symbol" => symbol,
                "gauge" => gauge,
                "admin" => admin,
            },
            block_time
        )
    }
    pub fn claim_tokens(&self, owner: AccountHash, addr: Option<Key>,block_time:u64) {
        self.0.call_contract(
            owner,
            "claim_tokens",
            runtime_args! {
                "addr" => addr
            },
            block_time
        );
    }
    pub fn set_approve_deposit(&self, owner: AccountHash, addr: Key, can_deposit: bool,block_time:u64) {
        self.0.call_contract(
            owner,
            "set_approve_deposit",
            runtime_args! {
                "addr" => addr,
                "can_deposit" => can_deposit
            },
            block_time
        );
    }
    pub fn deposit(&self, owner: AccountHash, value: U256, addr: Option<Key>,block_time:u64) {
        self.0.call_contract(
            owner,
            "deposit",
            runtime_args! {
                "value" => value,
                "addr" => addr,
            },
            block_time
        );
    }
    pub fn withdraw(&self, owner: AccountHash, value: U256, addr: Key,block_time:u64) {
        self.0.call_contract(
            owner,
            "withdraw",
            runtime_args! {
                "value" => value,
                "addr" => addr,
            },
            block_time
        );
    }
    pub fn transfer(&self, owner: AccountHash, recipient: Key, amount: U256,block_time:u64) {
        self.0.call_contract(
            owner,
            "transfer",
            runtime_args! {
                "recipient" => recipient,
                "amount" => amount,
            },
            block_time
        );
    }
    pub fn transfer_from(&self, owner: AccountHash, key: Key, recipient: Key, amount: U256,block_time:u64) {
        self.0.call_contract(
            owner,
            "transfer_from",
            runtime_args! {
                "owner" => key,
                "recipient" => recipient,
                "amount" => amount,
            },
            block_time
        );
    }
    pub fn approve(&self, owner: AccountHash, spender: Key, amount: U256,block_time:u64) {
        self.0.call_contract(
            owner,
            "approve",
            runtime_args! {
                "spender" => spender,
                "amount" => amount,
            },
            block_time
        );
    }
    pub fn increase_allowance(&self, owner: AccountHash, spender: Key, amount: U256,block_time:u64) {
        self.0.call_contract(
            owner,
            "increase_allowance",
            runtime_args! {
                "spender" => spender,
                "amount" => amount,
            },
            block_time
        );
    }
    pub fn decrease_allowance(&self, owner: AccountHash, spender: Key, amount: U256,block_time:u64) {
        self.0.call_contract(
            owner,
            "decrease_allowance",
            runtime_args! {
                "spender" => spender,
                "amount" => amount,
            },
            block_time
        );
    }
    pub fn kill_me(&self, owner: AccountHash,block_time:u64) {
        self.0.call_contract(owner, "kill_me", runtime_args! {}, block_time);
    }
    pub fn commit_transfer_ownership(&self, owner: AccountHash, addr: Key,block_time:u64) {
        self.0.call_contract(
            owner,
            "commit_transfer_ownership",
            runtime_args! {
                "addr" => addr
            },
            block_time
        );
    }

    pub fn apply_transfer_ownership(&self, owner: AccountHash,block_time:u64) {
        self.0
            .call_contract(owner, "apply_transfer_ownership", runtime_args! {}, block_time);
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
