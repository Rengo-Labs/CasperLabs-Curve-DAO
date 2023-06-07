use crate::lp_token_wrapper_instance::LPTOKENWRAPPERInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;
//Const
pub const TEN_E_NINE: u128 = 1000000000;
fn deploy_erc20(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "curve-erc20.wasm",
        "erc2020",
        owner,
        runtime_args! {
            "name" => "ERC",
            "symbol" => "ERC20",
            "decimals" => 9_u8,
            "initial_supply" => U256::from(TEN_E_NINE * 10000000000000)
        },
        LPTOKENWRAPPERInstance::now(),
    )
}
fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let erc20 = deploy_erc20(&env, owner);
    let lp_token_wrapper_instance = LPTOKENWRAPPERInstance::new_deploy(
        &env,
        "LPTOKENWRAPPER",
        owner,
        Key::Hash(erc20.package_hash()),
    );
    let lp_token_wrapper_package_hash = Key::Hash(lp_token_wrapper_instance.package_hash());
    // For Minting Purpose
    let to: Key = lp_token_wrapper_package_hash;
    let amount: U256 = U256::from(TEN_E_NINE * 1000000000000);
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount},
        LPTOKENWRAPPERInstance::now(),
    );
    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {"spender" => to , "amount" => amount},
        LPTOKENWRAPPERInstance::now(),
    );
    (env, owner, lp_token_wrapper_instance)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}
#[test]
fn total_supply() {
    let (env, owner, lp_token_wrapper) = deploy();
    let package_hash = Key::Hash(lp_token_wrapper.package_hash());
    let lp_token_wrapper_instance = LPTOKENWRAPPERInstance::contract_instance(lp_token_wrapper);
    let amount: U256 = U256::from(TEN_E_NINE * 2);
    lp_token_wrapper_instance.stake(owner, amount);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(TOTAL_SUPPLY),
            "package_hash" => package_hash,
        },
        LPTOKENWRAPPERInstance::now(),
    );
    let ret: U256 = env.query_account_named_key(owner, &[TOTAL_SUPPLY.into()]);
    assert_eq!(ret, amount, "Invalid result");
}
#[test]
fn balance_of() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let lp_token_wrapper_instance = LPTOKENWRAPPERInstance::contract_instance(instance);
    let amount: U256 = U256::from(TEN_E_NINE * 2);
    lp_token_wrapper_instance.stake(owner, amount);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => package_hash,
            "address" => Key::Account(owner)
        },
        LPTOKENWRAPPERInstance::now(),
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, amount, "Invalid result");
}
#[test]
fn stake() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let lp_token_wrapper_instance = LPTOKENWRAPPERInstance::contract_instance(instance);
    let amount: U256 = U256::from(TEN_E_NINE * 20);
    lp_token_wrapper_instance.stake(owner, amount);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => package_hash,
            "address" => Key::Account(owner)
        },
        LPTOKENWRAPPERInstance::now(),
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, amount, "Invalid result");
}
#[test]
fn withdraw() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let lp_token_wrapper_instance = LPTOKENWRAPPERInstance::contract_instance(instance);
    let amount: U256 = U256::from(TEN_E_NINE * 20);
    lp_token_wrapper_instance.stake(owner, amount);
    let withdraw_amount: U256 = U256::from(TEN_E_NINE * 10);
    lp_token_wrapper_instance.withdraw(owner, withdraw_amount);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => package_hash,
            "address" => Key::Account(owner)
        },
        LPTOKENWRAPPERInstance::now(),
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, withdraw_amount, "Invalid result");
}
#[should_panic]
#[test]
fn withdraw_panic() {
    let (_, owner, instance) = deploy();
    let lp_token_wrapper_instance = LPTOKENWRAPPERInstance::contract_instance(instance);
    let withdraw_amount: U256 = U256::from(TEN_E_NINE * 10);
    lp_token_wrapper_instance.withdraw(owner, withdraw_amount);
}
