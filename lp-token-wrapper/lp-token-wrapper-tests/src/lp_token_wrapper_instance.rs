use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};
use std::time::SystemTime;

pub struct LPTOKENWRAPPERInstance(TestContract);
#[allow(clippy::too_many_arguments)]
impl LPTOKENWRAPPERInstance {
    pub fn contract_instance(contract: TestContract) -> LPTOKENWRAPPERInstance {
        LPTOKENWRAPPERInstance(contract)
    }
    pub fn new_deploy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        uni: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "lp-token-wrapper.wasm",
            contract_name,
            sender,
            runtime_args! {
                "uni" => uni,
            },
            LPTOKENWRAPPERInstance::now(),
        )
    }
    pub fn total_supply(&self, sender: AccountHash) {
        self.0.call_contract(
            sender,
            "total_supply",
            runtime_args! {},
            LPTOKENWRAPPERInstance::now(),
        );
    }
    pub fn balance_of(&self, sender: AccountHash, account: Key) {
        self.0.call_contract(
            sender,
            "balance_of",
            runtime_args! {
                "address" => account
            },
            LPTOKENWRAPPERInstance::now(),
        );
    }
    pub fn stake(&self, sender: AccountHash, amount: U256) {
        self.0.call_contract(
            sender,
            "stake",
            runtime_args! {
                "amount" => amount
            },
            LPTOKENWRAPPERInstance::now(),
        );
    }
    pub fn withdraw(&self, sender: AccountHash, amount: U256) {
        self.0.call_contract(
            sender,
            "withdraw",
            runtime_args! {
                "amount" => amount
            },
            LPTOKENWRAPPERInstance::now(),
        );
    }
    // Result methods
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.query_named_key("self_package_hash".to_string())
    }

    pub fn now() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}
