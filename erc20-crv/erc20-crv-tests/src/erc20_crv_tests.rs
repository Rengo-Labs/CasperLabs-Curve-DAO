use crate::erc20_crv_instance::ERC20CRVInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::{keys::*, utils::key_to_str};
use crv20::Address;
use erc20_crv::data::*;

pub const TEN_E_NINE: u128 = 1000000000;
const MILLI_SECONDS_IN_DAY: u64 = 86_400_000;

fn deploy() -> (TestEnv, AccountHash, ERC20CRVInstance, u64) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let time_now: u64 = ERC20CRVInstance::now();
    let instance = ERC20CRVInstance::new_deploy(
        &env,
        "ERC20CRV",
        owner,
        "ERC20CRV".to_string(),
        "erc20_crv".to_string(),
        9_u8,
        time_now,
    );
    (env, owner, instance, time_now)
}

#[test]
fn test_deploy() {
    let (env, owner, contract, time_now) = deploy();
    assert_eq!(contract.get_init_supply(), 1303030303000000000_i64.into());
    assert!(contract
        .contract()
        .query_dictionary::<bool>("admin_whitelist", key_to_str(&Key::Account(owner)))
        .unwrap());
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(contract.package_hash()),
            "address"=>Address::from(owner)
        },
        time_now,
    );

    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 1303030303000000000_i64.into());
    let start_epoch_time: U256 = U256::from(time_now) + INFLATION_DELAY - RATE_REDUCTION_TIME;
    assert_eq!(contract.get_start_epoch_time(), start_epoch_time);
}

#[test]
fn burn() {
    let (env, owner, contract, time_now) = deploy();
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(TOTAL_SUPPLY),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        time_now,
    );
    let mut ret: U256 = env.query_account_named_key(owner, &[TOTAL_SUPPLY.into()]);
    assert_eq!(ret, 1303030303000000000_i64.into());
    contract.burn(owner, 1303030303000000000_i64.into());
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(TOTAL_SUPPLY),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        time_now,
    );
    ret = env.query_account_named_key(owner, &[TOTAL_SUPPLY.into()]);
    assert_eq!(ret, 0.into());
}
#[test]
fn set_and_remove_admin() {
    let (env, owner, contract, _) = deploy();
    let admin: Key = Key::Account(env.next_user());
    contract.set_admin(owner, admin);
    assert!(contract
        .contract()
        .query_dictionary::<bool>("admin_whitelist", key_to_str(&admin))
        .unwrap());
    contract.remove_admin(owner, admin);
    assert!(!contract
        .contract()
        .query_dictionary::<bool>("admin_whitelist", key_to_str(&admin))
        .unwrap());
}
#[test]
fn test_set_minter() {
    let (env, owner, contract, _) = deploy();
    let minter: Key = Key::Account(env.next_user());
    contract.set_minter(owner, minter);
    assert_eq!(contract.get_minter(), minter);
}
#[test]
fn test_update_mining_parameters() {
    let (_, owner, contract, time_now) = deploy();
    contract.update_mining_parameters(owner, time_now + MILLI_SECONDS_IN_DAY);
    assert_eq!(contract.get_rate(), 8714335.into());
}
#[test]
fn test_start_epoch_time_write() {
    let (env, owner, contract, time_now) = deploy();
    TestContract::new(
        &env,
        "erc20-crv-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(START_EPOCH_TIME_WRITE),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        time_now,
    );
    let epcoh_time: U256 = U256::from(time_now + MILLI_SECONDS_IN_DAY - 31536000000);
    let ret: U256 = env.query_account_named_key(owner, &[START_EPOCH_TIME_WRITE.into()]);
    assert_eq!(ret / 60000, epcoh_time / 60000);
}

#[test]
fn test_future_epoch_time_write() {
    let (env, owner, contract, time_now) = deploy();
    TestContract::new(
        &env,
        "erc20-crv-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(FUTURE_EPOCH_TIME_WRITE),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        time_now,
    );
    let futrue_epcoh_time: U256 = U256::from(time_now + MILLI_SECONDS_IN_DAY);
    let ret: U256 = env.query_account_named_key(owner, &[FUTURE_EPOCH_TIME_WRITE.into()]);
    assert_eq!(ret / 60000, futrue_epcoh_time / 60000);
}
#[test]
fn test_available_supply() {
    let (env, owner, contract, time_now) = deploy();
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(AVAILABLE_SUPPLY),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        time_now,
    );
    let available_supply = contract.get_start_epoch_supply()
        + (U256::from(time_now) - contract.get_start_epoch_time()) * contract.get_rate();
    let ret: U256 = env.query_account_named_key(owner, &[AVAILABLE_SUPPLY.into()]);
    assert_eq!(ret, available_supply);
}
#[test]
fn test_total_supply() {
    let (env, owner, contract, time_now) = deploy();
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(TOTAL_SUPPLY),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        time_now,
    );
    let ret: U256 = env.query_account_named_key(owner, &[TOTAL_SUPPLY.into()]);
    assert_eq!(ret, 1303030303000000000_i64.into());
}
#[test]
fn test_mintable_in_timeframe() {
    let (env, owner, contract, time_now) = deploy();
    contract.update_mining_parameters(owner, time_now + MILLI_SECONDS_IN_DAY);
    let start: U256 = U256::from(time_now);
    let end: U256 = start + MILLI_SECONDS_IN_DAY;
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(MINTABLE_IN_TIMEFRAME),
            "package_hash" => Key::Hash(contract.package_hash()),
            "start"=>start,
            "end"=>end
        },
        time_now,
    );

    let _ret: U256 = env.query_account_named_key(owner, &[MINTABLE_IN_TIMEFRAME.into()]);
    assert_eq!(_ret, 752918544000000_i128.into());
}
#[test]
fn test_mint() {
    let (env, owner, contract, time_now) = deploy();
    let to = env.next_user();
    let amount: U256 = U256::from(10 * TEN_E_NINE);
    let minter: Key = Key::Account(owner);
    contract.set_minter(owner, minter);
    contract.mint(
        owner,
        Address::Account(to),
        amount,
        time_now + MILLI_SECONDS_IN_DAY + 2000,
    );
    //checking balance
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        to,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(contract.package_hash()),
            "address"=>Address::Account(to)
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(to, &[BALANCE_OF.into()]);
    assert_eq!(ret, U256::from(10 * TEN_E_NINE));
}
#[test]
fn test_increase_allowance() {
    let (env, owner, contract, _) = deploy();
    let spender: Address = Address::from(env.next_user());
    let amount: U256 = U256::from(100 * TEN_E_NINE);
    contract.increase_allowance(owner, spender, amount);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(ALLOWANCE_CRV),
            "package_hash" => Key::Hash(contract.package_hash()),
            "owner"=>Address::from(owner),
            "spender"=>spender
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[ALLOWANCE.into()]);
    assert_eq!(ret, U256::from(100 * TEN_E_NINE));
}
#[test]
fn test_decrease_allowance() {
    let (env, owner, contract, _) = deploy();
    let spender: Address = Address::from(env.next_user());
    let amount: U256 = U256::from(100 * TEN_E_NINE);
    contract.approve(owner, spender, amount);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(ALLOWANCE_CRV),
            "package_hash" => Key::Hash(contract.package_hash()),
            "owner"=>Address::from(owner),
            "spender"=>spender
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[ALLOWANCE.into()]);
    assert_eq!(ret, U256::from(100 * TEN_E_NINE));
    let decrease_amount: U256 = U256::from(80 * TEN_E_NINE);
    contract.decrease_allowance(owner, spender, decrease_amount);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(ALLOWANCE_CRV),
            "package_hash" => Key::Hash(contract.package_hash()),
            "owner"=>Address::from(owner),
            "spender"=>spender
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[ALLOWANCE.into()]);
    assert_eq!(ret, U256::from(20 * TEN_E_NINE));
}
#[test]
fn test_transfer() {
    let (env, owner, contract, time_now) = deploy();
    let recipient: Address = Address::from(env.next_user());
    let amount: U256 = U256::from(100 * TEN_E_NINE);
    contract.transfer(owner, recipient, amount);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(contract.package_hash()),
            "address"=>Key::from(owner)
        },
        time_now,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 1303030203000000000_i64.into());
}
#[test]
fn test_transfer_from() {
    let (env, owner, contract, time_now) = deploy();
    let spender: AccountHash = env.next_user();
    let recipient: Address = Address::from(env.next_user());
    let amount: U256 = U256::from(100 * TEN_E_NINE);
    contract.approve(owner, Address::Account(spender), amount);
    contract.transfer_from(spender, Address::from(owner), recipient, amount);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(contract.package_hash()),
            "address"=>Key::from(owner)
        },
        time_now,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 1303030203000000000_i64.into());
}
#[test]
fn test_allowance() {
    let (env, owner, contract, time_now) = deploy();
    let spender: AccountHash = env.next_user();
    let recipient: Address = Address::from(env.next_user());
    let amount: U256 = U256::from(100 * TEN_E_NINE);
    contract.approve(owner, Address::Account(spender), amount);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(ALLOWANCE_CRV),
            "package_hash" => Key::Hash(contract.package_hash()),
            "owner"=>Address::from(owner),
            "spender"=>Address::from(spender)
        },
        time_now,
    );
    let ret: U256 = env.query_account_named_key(owner, &[ALLOWANCE.into()]);
    assert_eq!(ret, U256::from(100 * TEN_E_NINE));
    let transfer_amount: U256 = U256::from(50 * TEN_E_NINE);
    contract.transfer_from(spender, Address::from(owner), recipient, transfer_amount);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(ALLOWANCE_CRV),
            "package_hash" => Key::Hash(contract.package_hash()),
            "owner"=>Key::from(owner),
            "spender"=>Key::from(spender)
        },
        time_now,
    );
    let ret: U256 = env.query_account_named_key(owner, &[ALLOWANCE.into()]);
    assert_eq!(ret, U256::from(50 * TEN_E_NINE));
}
