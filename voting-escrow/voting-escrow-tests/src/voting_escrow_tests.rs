use crate::voting_escrow_instance::{now, VOTINGESCROWInstance, MILLI_SECONDS_IN_DAY};
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U128, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;
use crv20::Address;
use voting_escrow_crate::data::{LockedBalance, Point, WEEK};
pub const TEN_E_NINE: u128 = 1000000000;
// CRV
fn deploy_erc20_crv(env: &TestEnv, sender: AccountHash, time_now: u64) -> TestContract {
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
        time_now,
    )
}
fn deploy() -> (
    TestEnv,
    AccountHash,
    VOTINGESCROWInstance,
    TestContract,
    u64,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let time_now: u64 = now();
    let erc20_crv = deploy_erc20_crv(&env, owner, time_now);
    let instance = VOTINGESCROWInstance::new_deploy(
        &env,
        "Vote-escrowed CRV",
        owner,
        Key::Hash(erc20_crv.package_hash()),
        "Vote-escrowed CRV".into(),
        "veCRV".into(),
        "veCRV_1.0.0".into(),
        time_now,
    );

    (env, owner, instance, erc20_crv, time_now)
}
#[test]
fn test_deploy() {
    let (_, owner, instance, erc20_crv, _) = deploy();
    let admin: Key = instance.key_value(ADMIN.to_string());
    assert_eq!(admin, Key::from(owner));
    let token_addr: Key = instance.key_value(TOKEN.to_string());
    assert_eq!(token_addr, Key::Hash(erc20_crv.package_hash()));
    let controller: Key = instance.key_value(CONTROLLER.to_string());
    assert_eq!(controller, Key::from(owner));
    let transfer_enable: bool = instance.key_value(TRANSFERS_ENABLED.to_string());
    assert!(transfer_enable);
    let decimals: U256 = instance.key_value(DECIMALS.to_string());
    assert_eq!(decimals, 9.into());
    let name: String = instance.key_value(NAME.to_string());
    assert_eq!(name, "Vote-escrowed CRV");
    let symbol: String = instance.key_value(SYMBOL.to_string());
    assert_eq!(symbol, "veCRV");
    let version: String = instance.key_value(VERSION.to_string());
    assert_eq!(version, "veCRV_1.0.0");
}
#[test]
fn test_commit_transfer_ownership() {
    let (env, owner, instance, _, time_now) = deploy();
    let addr: Key = Key::Account(env.next_user());
    instance.commit_transfer_ownership(owner, addr, time_now);
    let ret: Key = instance.key_value(FUTURE_ADMIN.to_string());
    assert_eq!(ret, addr, "Ownership not transferred");
}
#[test]
fn test_apply_transfer_ownership() {
    let (env, owner, instance, _, time_now) = deploy();
    let addr: Key = Key::Account(env.next_user());
    instance.commit_transfer_ownership(owner, addr, time_now);
    instance.apply_transfer_ownership(owner, time_now);
    let ret: Key = instance.key_value(ADMIN.to_string());
    assert_eq!(ret, addr, "Ownership transfer not applied");
}

#[test]
fn test_get_last_user_slope() {
    let (env, owner, instance, erc20_crv, time_now) = deploy();
    let amount: U256 = U256::from(2500 * TEN_E_NINE);
    let unlock_time = U256::from(time_now + MILLI_SECONDS_IN_DAY * 365 * 4);
    let spender: Address = Address::Contract(instance.package_hash().into());
    erc20_crv.call_contract(
        owner,
        "increase_allowance",
        runtime_args! {
            "spender" => spender,
            "amount" => amount
        },
        time_now,
    );
    instance.create_lock(owner, amount, unlock_time, time_now);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(GET_LAST_USER_SLOPE),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => Key::from(owner)
        },
        time_now,
    );
    let ret: (bool, U128) = env.query_account_named_key(owner, &[GET_LAST_USER_SLOPE.into()]);
    assert_eq!(ret, (false, 19.into()), "Invalid last user scope value");
}

#[test]
fn test_user_point_history_ts() {
    let (_, owner, instance, erc20_crv, time_now) = deploy();
    let amount_approve: U256 = U256::from(1000 * TEN_E_NINE);
    let amount: U256 = U256::from(1000 * TEN_E_NINE);
    let unlock_time = U256::from(time_now + MILLI_SECONDS_IN_DAY * 365 * 4);
    let spender: Address = Address::Contract(instance.package_hash().into());
    erc20_crv.call_contract(
        owner,
        "increase_allowance",
        runtime_args! {
            "spender" => spender,
            "amount" => amount_approve
        },
        time_now,
    );
    instance.create_lock(owner, amount, unlock_time, time_now);
    let ret: Point = instance.query_user_point_history(&Key::from(owner), &U256::from(1));
    assert_eq!(ret.ts, U256::from(time_now), "Invalid default value");
}
#[test]
fn test_locked_end() {
    let (_, owner, instance, erc20_crv, time_now) = deploy();
    let amount: U256 = U256::from(2500 * TEN_E_NINE);
    let unlock_time = U256::from(time_now + MILLI_SECONDS_IN_DAY * 365 * 4);
    let spender: Address = Address::Contract(instance.package_hash().into());
    erc20_crv.call_contract(
        owner,
        "increase_allowance",
        runtime_args! {
            "spender" => spender,
            "amount" => amount
        },
        time_now,
    );
    instance.create_lock(owner, amount, unlock_time, time_now);
    let ret: LockedBalance = instance.query_locked(&Key::from(owner));
    assert_eq!(ret.end / WEEK, unlock_time / WEEK, "Invalid default value");
}
#[test]
fn test_checkpoint() {
    let (_, owner, instance, _, time_now) = deploy();
    instance.checkpoint(owner, time_now);
}
#[test]
fn test_deposit_for() {
    let (env, owner, instance, erc20_crv, time_now) = deploy();
    let amount_approve: U256 = U256::from(400 * TEN_E_NINE);
    let amount: U256 = U256::from(200 * TEN_E_NINE);
    let unlock_time = U256::from(time_now + MILLI_SECONDS_IN_DAY * 365 * 4);
    let spender: Address = Address::Contract(instance.package_hash().into());
    erc20_crv.call_contract(
        owner,
        "increase_allowance",
        runtime_args! {
            "spender" => spender,
            "amount" => amount_approve
        },
        time_now,
    );
    instance.create_lock(owner, amount, unlock_time, time_now);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VE_BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => Key::from(owner),
            "t" => U256::from(time_now)
        },
        time_now,
    );
    let balance_after_lock: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    let addr = env.next_user();
    instance.deposit_for(addr, Key::from(owner), amount, time_now);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VE_BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => Key::from(owner),
            "t" => U256::from(time_now)
        },
        time_now,
    );
    let balance_after_deposit: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert!(balance_after_deposit > balance_after_lock);
}

#[test]
fn test_create_lock() {
    let (env, owner, instance, erc20_crv, time_now) = deploy();
    let amount_approve: U256 = U256::from(1000 * TEN_E_NINE);
    let amount: U256 = U256::from(1000 * TEN_E_NINE);
    let unlock_time = U256::from(time_now + MILLI_SECONDS_IN_DAY * 365 * 4);
    let spender: Address = Address::Contract(instance.package_hash().into());
    erc20_crv.call_contract(
        owner,
        "increase_allowance",
        runtime_args! {
            "spender" => spender,
            "amount" => amount_approve
        },
        time_now,
    );
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VE_BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => Key::from(owner),
            "t" => U256::from(time_now)
        },
        time_now,
    );
    let balance_before_lock: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(balance_before_lock, 0.into());
    instance.create_lock(owner, amount, unlock_time, time_now);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VE_BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => Key::from(owner),
            "t" => U256::from(time_now)
        },
        time_now,
    );
    let balance_after_lock: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert!(
        balance_after_lock / TEN_E_NINE <= 890.into()
            && balance_after_lock / TEN_E_NINE >= 860.into()
    );
}

#[test]
fn test_increase_amount() {
    let (env, owner, instance, erc20_crv, time_now) = deploy();
    let amount_approve: U256 = U256::from(400 * TEN_E_NINE);
    let amount: U256 = U256::from(200 * TEN_E_NINE);
    let unlock_time = U256::from(time_now + MILLI_SECONDS_IN_DAY * 365 * 4);
    let spender: Address = Address::Contract(instance.package_hash().into());
    erc20_crv.call_contract(
        owner,
        "increase_allowance",
        runtime_args! {
            "spender" => spender,
            "amount" => amount_approve
        },
        time_now,
    );
    instance.create_lock(owner, amount, unlock_time, time_now);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VE_BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => Key::from(owner),
            "t" => U256::from(time_now)
        },
        time_now,
    );
    let balance_after_lock: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    instance.increase_amount(owner, amount, time_now);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VE_BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => Key::from(owner),
            "t" => U256::from(time_now)
        },
        time_now,
    );
    let balance_after_increase_amount: U256 =
        env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert!(balance_after_increase_amount > balance_after_lock);
}

#[test]
fn test_increase_unlock_time() {
    let (env, owner, instance, erc20_crv, time_now) = deploy();
    let amount_approve: U256 = U256::from(1000 * TEN_E_NINE);
    let amount: U256 = U256::from(1000 * TEN_E_NINE);
    let unlock_time = U256::from(time_now + MILLI_SECONDS_IN_DAY * 365);
    let spender: Address = Address::Contract(instance.package_hash().into());
    erc20_crv.call_contract(
        owner,
        "increase_allowance",
        runtime_args! {
            "spender" => spender,
            "amount" => amount_approve
        },
        time_now,
    );
    instance.create_lock(owner, amount, unlock_time, time_now);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VE_BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => Key::from(owner),
            "t" => U256::from(time_now)
        },
        time_now,
    );
    let balance_after_lock: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    // assert_eq!(balance_after_lock,0.into());
    let unlock_time_increase = U256::from(time_now + MILLI_SECONDS_IN_DAY * 365 * 4);
    instance.increase_unlock_time(owner, unlock_time_increase, time_now);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VE_BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => Key::from(owner),
            "t" => U256::from(time_now)
        },
        time_now,
    );
    let balance_after_increase_unlock_time: U256 =
        env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert!(balance_after_increase_unlock_time > balance_after_lock);
}
#[test]
fn test_withdraw() {
    let (env, owner, instance, erc20_crv, time_now) = deploy();
    let amount_approve: U256 = U256::from(1000 * TEN_E_NINE);
    let amount: U256 = U256::from(1000 * TEN_E_NINE);
    let unlock_time = U256::from(time_now + MILLI_SECONDS_IN_DAY * 365);
    let spender: Address = Address::Contract(instance.package_hash().into());
    erc20_crv.call_contract(
        owner,
        "increase_allowance",
        runtime_args! {
            "spender" => spender,
            "amount" => amount_approve
        },
        time_now,
    );
    instance.create_lock(owner, amount, unlock_time, time_now);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VE_BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => Key::from(owner),
            "t" => U256::from(time_now)
        },
        time_now,
    );
    let _balance_after_lock: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    // assert_eq!(balance_after_lock,0.into());
    let after_unlock_time = time_now + MILLI_SECONDS_IN_DAY * 365;
    instance.withdraw(owner, after_unlock_time);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VE_BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => Key::from(owner),
            "t" => U256::from(after_unlock_time)
        },
        after_unlock_time,
    );
    let balance_after_withdraw: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(balance_after_withdraw, 0.into());
}
#[test]
fn test_balance_of() {
    let (env, owner, instance, erc20_crv, time_now) = deploy();
    let amount_approve: U256 = U256::from(1000 * TEN_E_NINE);
    let amount: U256 = U256::from(1000 * TEN_E_NINE);
    let unlock_time = U256::from(time_now + MILLI_SECONDS_IN_DAY * 365 * 4);
    let spender: Address = Address::Contract(instance.package_hash().into());
    erc20_crv.call_contract(
        owner,
        "increase_allowance",
        runtime_args! {
            "spender" => spender,
            "amount" => amount_approve
        },
        time_now,
    );
    instance.create_lock(owner, amount, unlock_time, time_now);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VE_BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => Key::from(owner),
            "t" => U256::from(time_now)
        },
        time_now,
    );
    let balance_after_lock: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert!(
        balance_after_lock / TEN_E_NINE <= 890.into()
            && balance_after_lock / TEN_E_NINE >= 860.into()
    );
}

#[test]
fn test_balance_of_at() {
    let (env, owner, instance, erc20_crv, time_now) = deploy();

    let amount: U256 = U256::from(2500 * TEN_E_NINE);
    let unlock_time = U256::from(time_now + MILLI_SECONDS_IN_DAY * 365 * 4);
    let spender: Address = Address::Contract(instance.package_hash().into());
    erc20_crv.call_contract(
        owner,
        "increase_allowance",
        runtime_args! {
            "spender" => spender,
            "amount" => amount
        },
        time_now,
    );
    let time_call_balance_at: u64 = time_now + MILLI_SECONDS_IN_DAY;
    let current_block: U256 = U256::from(time_now / 45000);
    instance.create_lock(owner, amount, unlock_time, time_now);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF_AT),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => Key::from(owner),
            "block" => current_block
        },
        time_call_balance_at,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF_AT.into()]);
    assert!(
        ret / TEN_E_NINE >= 2385.into() && ret / TEN_E_NINE <= 2395.into(),
        "Invalid default value balance of at"
    );
}

#[test]
fn test_total_supply() {
    let (env, owner, instance, erc20_crv, time_now) = deploy();
    let user = env.next_user();
    let amount_approve: U256 = U256::from(1000 * TEN_E_NINE);
    let amount: U256 = U256::from(1000 * TEN_E_NINE);
    let unlock_time = U256::from(time_now + MILLI_SECONDS_IN_DAY * 365 * 4);
    let spender: Address = Address::Contract(instance.package_hash().into());
    erc20_crv.call_contract(
        owner,
        "increase_allowance",
        runtime_args! {
            "spender" => spender,
            "amount" => amount_approve
        },
        time_now,
    );
    erc20_crv.call_contract(
        owner,
        "transfer",
        runtime_args! {
            "recipient" => Key::from(user),
            "amount" => amount
        },
        time_now,
    );
    erc20_crv.call_contract(
        user,
        "increase_allowance",
        runtime_args! {
            "spender" => Key::Hash(instance.package_hash()),
            "amount" => amount_approve
        },
        time_now,
    );

    instance.create_lock(owner, amount, unlock_time, time_now);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VE_TOTAL_SUPPLY),
            "package_hash" => Key::Hash(instance.package_hash()),
            "t" => U256::from(time_now),
        },
        time_now,
    );
    let ret: U256 = env.query_account_named_key(owner, &[TOTAL_SUPPLY.into()]);
    assert!(ret / TEN_E_NINE <= 890.into() && ret / TEN_E_NINE >= 860.into());
    instance.create_lock(user, amount, unlock_time, time_now);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VE_TOTAL_SUPPLY),
            "package_hash" => Key::Hash(instance.package_hash()),
            "t" => U256::from(time_now),
        },
        time_now,
    );
    let ret: U256 = env.query_account_named_key(owner, &[TOTAL_SUPPLY.into()]);
    assert!(
        ret / TEN_E_NINE <= 1780.into() && ret / TEN_E_NINE >= 1740.into(),
        "Invalid default total supply"
    );
    //Total supply will be 0 after lock time expired
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VE_TOTAL_SUPPLY),
            "package_hash" => Key::Hash(instance.package_hash()),
            "t" => unlock_time,
        },
        time_now,
    );
    let ret: U256 = env.query_account_named_key(owner, &[TOTAL_SUPPLY.into()]);
    assert_eq!(ret / TEN_E_NINE, 0.into(), "Invalid default total supply");
}
#[test]
fn test_total_supply_at() {
    let (env, owner, instance, erc20_crv, time_now) = deploy();
    let user = env.next_user();
    let amount_approve: U256 = U256::from(1000 * TEN_E_NINE);
    let amount: U256 = U256::from(1000 * TEN_E_NINE);
    let unlock_time = U256::from(time_now + MILLI_SECONDS_IN_DAY * 365 * 4);
    let spender: Address = Address::Contract(instance.package_hash().into());
    erc20_crv.call_contract(
        owner,
        "increase_allowance",
        runtime_args! {
            "spender" => spender,
            "amount" => amount_approve
        },
        time_now,
    );
    erc20_crv.call_contract(
        owner,
        "transfer",
        runtime_args! {
            "recipient" => Key::from(user),
            "amount" => amount
        },
        time_now,
    );
    erc20_crv.call_contract(
        user,
        "increase_allowance",
        runtime_args! {
            "spender" => Key::Hash(instance.package_hash()),
            "amount" => amount_approve
        },
        time_now,
    );

    instance.create_lock(owner, amount, unlock_time, time_now);
    let current_block: U256 = U256::from(time_now / 45000);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(TOTAL_SUPPLY_AT),
            "package_hash" => Key::Hash(instance.package_hash()),
            "block" => current_block,
        },
        time_now,
    );
    let ret: U256 = env.query_account_named_key(owner, &[TOTAL_SUPPLY_AT.into()]);
    assert!(ret / TEN_E_NINE <= 890.into() && ret / TEN_E_NINE >= 860.into());
    instance.create_lock(user, amount, unlock_time, time_now);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(TOTAL_SUPPLY_AT),
            "package_hash" => Key::Hash(instance.package_hash()),
            "block" => current_block,
        },
        time_now,
    );
    let ret: U256 = env.query_account_named_key(owner, &[TOTAL_SUPPLY_AT.into()]);
    assert!(
        ret / TEN_E_NINE <= 1780.into() && ret / TEN_E_NINE >= 1740.into(),
        "Invalid default total supply at"
    );
}

#[test]
fn test_change_controller() {
    let (env, owner, instance, _, time_now) = deploy();
    let new_controller: Key = Key::Account(env.next_user());
    instance.change_controller(owner, new_controller, time_now);
    let ret: Key = instance.key_value(CONTROLLER.to_string());
    assert_eq!(ret, new_controller, "Controller not changed");
}
