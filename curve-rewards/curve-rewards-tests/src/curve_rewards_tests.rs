use crate::curve_rewards_instance::CURVEREWARDSInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;
//Const
pub const TEN_E_NINE: u128 = 1000000000;
pub const WEEK: U256 = U256([604800000, 0, 0, 0]);
fn deploy_token(env: &TestEnv, owner: AccountHash, block_time: u64) -> TestContract {
    TestContract::new(
        env,
        "curve-erc20.wasm",
        "erc2020",
        owner,
        runtime_args! {
            "name" => "Token",
            "symbol" => "TK",
            "decimals" => 9_u8,
            "initial_supply" => U256::from(TEN_E_NINE * 1000000000000000000)
        },
        block_time,
    )
}
fn deploy_reward(env: &TestEnv, owner: AccountHash, block_time: u64) -> TestContract {
    TestContract::new(
        env,
        "curve-erc20.wasm",
        "erc",
        owner,
        runtime_args! {
            "name" => "Reward",
            "symbol" => "RD",
            "decimals" => 9_u8,
            "initial_supply" => U256::from(TEN_E_NINE * 1000000000000000000000)
        },
        block_time,
    )
}
fn deploy() -> (TestEnv, AccountHash, TestContract, u64) {
    let block_time = CURVEREWARDSInstance::now();
    let env = TestEnv::new();
    let owner = env.next_user();
    let token = deploy_token(&env, owner, block_time);
    let reward = deploy_reward(&env, owner, block_time);
    let curve_rewards_instance = CURVEREWARDSInstance::new_deploy(
        &env,
        "CURVEREWARDS",
        owner,
        Key::Hash(token.package_hash()),
        Key::Hash(reward.package_hash()),
        block_time,
    );
    let curve_rewards_package_hash = Key::Hash(curve_rewards_instance.package_hash());
    // For Minting Purpose
    let to: Key = curve_rewards_package_hash;
    let amount: U256 = U256::from(TEN_E_NINE * 100000000000000000000);
    token.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount},
        block_time,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {"spender" => to , "amount" => amount},
        block_time,
    );
    reward.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount},
        block_time,
    );
    reward.call_contract(
        owner,
        "approve",
        runtime_args! {"spender" => to , "amount" => amount},
        block_time,
    );
    (env, owner, curve_rewards_instance, block_time)
}

#[test]
fn test_deploy() {
    let (_, _, _, _) = deploy();
}
#[test]
fn last_time_reward_applicable() {
    let (env, owner, instance, block_time) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(LAST_TIME_REWARD_APPLICABLE),
            "package_hash" => package_hash,
        },
        block_time,
    );
    let ret: U256 = env.query_account_named_key(owner, &[LAST_TIME_REWARD_APPLICABLE.into()]);
    assert_eq!(ret, 0.into(), "invalid result");
}
#[test]
fn reward_per_token() {
    let (env, owner, instance, block_time) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let curve_rewards_instance = CURVEREWARDSInstance::contract_instance(instance);
    let amount: U256 = U256::from(TEN_E_NINE * 30);
    curve_rewards_instance.stake(owner, amount, block_time);
    curve_rewards_instance.notify_reward_amount(owner, U256::from(TEN_E_NINE * 15), block_time);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(REWARD_PER_TOKEN),
            "package_hash" => package_hash,
        },
        block_time + WEEK.as_u64(),
    );
    let ret: U256 = env.query_account_named_key(owner, &[REWARD_PER_TOKEN.into()]);
    assert!(ret >= 200000000.into(), "invalid result");
}
#[test]
fn earned() {
    let (env, owner, instance, block_time) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let curve_rewards_instance = CURVEREWARDSInstance::contract_instance(instance);
    let amount: U256 = U256::from(TEN_E_NINE * 10000000000000);
    curve_rewards_instance.stake(owner, amount, block_time);
    curve_rewards_instance.notify_reward_amount(
        owner,
        U256::from(TEN_E_NINE * 1000000000000),
        block_time,
    );
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(EARNED),
            "package_hash" => package_hash,
            "account" => Key::Account(owner)
        },
        block_time + WEEK.as_u64(),
    );
    let ret: U256 = env.query_account_named_key(owner, &[EARNED.into()]);
    let v: u128 = 2400000000000000_u128;
    assert!(ret > v.into(), "invalid result");
}
#[test]
fn stake() {
    let (env, owner, instance, block_time) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let curve_rewards_instance = CURVEREWARDSInstance::contract_instance(instance);
    let amount: U256 = U256::from(TEN_E_NINE * 50);
    curve_rewards_instance.stake(owner, amount, block_time);
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
        block_time,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, amount, "Invalid result");
}
#[test]
fn withdraw() {
    let (env, owner, instance, block_time) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let curve_rewards_instance = CURVEREWARDSInstance::contract_instance(instance);
    let amount: U256 = U256::from(TEN_E_NINE * 60);
    curve_rewards_instance.stake(owner, amount, block_time);
    let withdraw_amount: U256 = U256::from(TEN_E_NINE * 30);
    curve_rewards_instance.withdraw(owner, withdraw_amount, block_time);
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
        block_time,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, withdraw_amount, "Invalid result");
}
#[should_panic]
#[test]
fn withdraw_panic() {
    let (_, owner, instance, block_time) = deploy();
    let curve_rewards_instance = CURVEREWARDSInstance::contract_instance(instance);
    let withdraw_amount: U256 = U256::from(TEN_E_NINE * 20);
    curve_rewards_instance.withdraw(owner, withdraw_amount, block_time);
}
#[test]
fn get_reward() {
    let (env, owner, instance, block_time) = deploy();

    let package_hash = Key::Hash(instance.package_hash());
    let curve_rewards_instance = CURVEREWARDSInstance::contract_instance(instance);
    let amount: U256 = U256::from(TEN_E_NINE * 20);
    curve_rewards_instance.stake(owner, amount, block_time);
    curve_rewards_instance.get_reward(owner, block_time);
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
        block_time,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, amount, "Invalid result");
}
#[test]
fn exit() {
    let (env, owner, instance, block_time) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let curve_rewards_instance = CURVEREWARDSInstance::contract_instance(instance);
    let amount: U256 = U256::from(TEN_E_NINE * 30);
    curve_rewards_instance.stake(owner, amount, block_time);
    curve_rewards_instance.exit(owner, block_time);
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
        block_time,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 0.into(), "Invalid result");
}
#[should_panic]
#[test]
fn exit_panic() {
    let (env, owner, instance, block_time) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let curve_rewards_instance = CURVEREWARDSInstance::contract_instance(instance);
    curve_rewards_instance.exit(owner, block_time);
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
        block_time,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 0.into(), "Invalid result");
}
#[test]
fn notify_reward_amount() {
    let (env, owner, instance, block_time) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let curve_rewards_instance = CURVEREWARDSInstance::contract_instance(instance);
    let amount: U256 = U256::from(TEN_E_NINE * 30);
    curve_rewards_instance.stake(owner, amount, block_time);
    curve_rewards_instance.notify_reward_amount(owner, U256::from(TEN_E_NINE * 20), block_time);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(LAST_TIME_REWARD_APPLICABLE),
            "package_hash" => package_hash,
        },
        block_time,
    );
    let ret: U256 = env.query_account_named_key(owner, &[LAST_TIME_REWARD_APPLICABLE.into()]);
    assert!(ret >= U256::from(block_time), "invalid result");
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
        block_time,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, amount, "Invalid result");
}
