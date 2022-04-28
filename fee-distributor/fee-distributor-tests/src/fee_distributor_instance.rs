use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};

pub struct FEEDISTRIBUTORInstance(TestContract);

impl FEEDISTRIBUTORInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        voting_escrow: Key,
        start_time: U256,
        token: Key,
        admin: Key,
        emergency_return: Key,
    ) -> FEEDISTRIBUTORInstance {
        FEEDISTRIBUTORInstance(TestContract::new(
            env,
            "fee-distributor.wasm",
            contract_name,
            sender,
            runtime_args! {
                "voting_escrow" => voting_escrow,
                "start_time" => start_time,
                "token" => token,
                "admin" => admin,
                "emergency_return" => emergency_return,
            },
            0,
        ))
    }

    pub fn commit_transfer_ownership(&self, owner: AccountHash, addr: Key) {
        self.0.call_contract(
            owner,
            "commit_transfer_ownership",
            runtime_args! {
                "addr" => addr
            },
            0,
        );
    }

    pub fn apply_transfer_ownership(&self, owner: AccountHash) {
        self.0
            .call_contract(owner, "apply_transfer_ownership", runtime_args! {}, 0);
    }

    pub fn commit_smart_wallet_checker(&self, owner: AccountHash, addr: Key) {
        self.0.call_contract(
            owner,
            "commit_smart_wallet_checker",
            runtime_args! {
                "addr" => addr
            },
            0,
        );
    }

    pub fn apply_smart_wallet_checker(&self, owner: AccountHash) {
        self.0
            .call_contract(owner, "apply_smart_wallet_checker", runtime_args! {}, 0);
    }

    pub fn get_last_user_slope_js_client(&self, owner: AccountHash, addr: Key) {
        self.0.call_contract(
            owner,
            "get_last_user_slope_js_client",
            runtime_args! {
                "addr" => addr
            },
            0,
        );
    }

    pub fn user_point_history_ts_js_client(&self, owner: AccountHash, addr: Key, idx: U256) {
        self.0.call_contract(
            owner,
            "user_point_history_ts_js_client",
            runtime_args! {
                "addr" => addr,
                "idx" => idx,
            },
            0,
        );
    }

    pub fn locked_end_js_client(&self, owner: AccountHash, addr: Key) {
        self.0.call_contract(
            owner,
            "locked_end_js_client",
            runtime_args! {
                "addr" => addr,
            },
            0,
        );
    }

    pub fn checkpoint(&self, owner: AccountHash) {
        self.0
            .call_contract(owner, "checkpoint", runtime_args! {}, 0);
    }

    pub fn deposit_for(&self, owner: AccountHash, addr: Key, value: U256) {
        self.0.call_contract(
            owner,
            "deposit_for",
            runtime_args! {
                "addr" => addr,
                "value" => value
            },
            0,
        );
    }

    pub fn create_lock(&self, owner: AccountHash, value: U256, unlock_time: U256) {
        self.0.call_contract(
            owner,
            "create_lock",
            runtime_args! {
                "value" => value,
                "unlock_time" =>  unlock_time
            },
            0,
        );
    }

    pub fn increase_amount(&self, owner: AccountHash, value: U256) {
        self.0.call_contract(
            owner,
            "increase_amount",
            runtime_args! {
                "value" => value
            },
            0,
        );
    }

    pub fn increase_unlock_time(&self, owner: AccountHash, unlock_time: U256) {
        self.0.call_contract(
            owner,
            "increase_unlock_time",
            runtime_args! {
                "unlock_time" => unlock_time
            },
            0,
        );
    }

    pub fn withdraw(&self, owner: AccountHash, time: u64) {
        self.0
            .call_contract(owner, "withdraw", runtime_args! {}, time);
    }

    pub fn balance_of_js_client(&self, owner: AccountHash, addr: Key, t: U256) {
        self.0.call_contract(
            owner,
            "balance_of_js_client",
            runtime_args! {
                "addr" => addr,
                "t" => t
            },
            0,
        );
    }

    pub fn balance_of_at_js_client(&self, owner: AccountHash, addr: Key, block: U256) {
        self.0.call_contract(
            owner,
            "balance_of_at_js_client",
            runtime_args! {
                "addr" => addr,
                "block" => block
            },
            0,
        );
    }

    pub fn total_supply_js_client(&self, owner: AccountHash, t: U256) {
        self.0.call_contract(
            owner,
            "total_supply_js_client",
            runtime_args! {
                "t" => t
            },
            0,
        );
    }

    pub fn total_supply_at_js_client(&self, owner: AccountHash, block: U256) {
        self.0.call_contract(
            owner,
            "total_supply_at_js_client",
            runtime_args! {
                "block" => block
            },
            0,
        );
    }

    pub fn change_controller(&self, owner: AccountHash, new_controller: Key) {
        self.0.call_contract(
            owner,
            "change_controller",
            runtime_args! {
                "new_controller" => new_controller
            },
            0,
        );
    }

    pub fn package_hash(&self) -> [u8; 32] {
        self.0.package_hash()
    }

    // Get stored key values
    pub fn key_value<T: CLTyped + FromBytes>(&self, key: String) -> T {
        self.0.query_named_key(key)
    }
}
