use crate::vesting_escrow_instance::VESTINGESCROWInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;

const NAME: &str = "VESTINGESCROW";

const TOKEN_NAME: &str = "ERC20";
const TOKEN_SYMBOL: &str = "ERC";
const DECIMALS: u8 = 9;
const INIT_TOTAL_SUPPLY: u64 = 0;
const MILLI_SECONDS_IN_DAY: u64 = 86_400_000;
pub const TEN_E_NINE: u128 = 1000000000;

fn deploy() -> (
    TestEnv,
    VESTINGESCROWInstance,
    AccountHash,
    AccountHash,
    TestContract,
    u64,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let time_now: u64 = VESTINGESCROWInstance::now();
    let _token: TestContract = VESTINGESCROWInstance::erc20(
        &env,
        owner,
        TOKEN_NAME,
        TOKEN_SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
    );
    let _start_time: U256 = U256::from(time_now);
    let _end_time: U256 = U256::from(time_now + MILLI_SECONDS_IN_DAY * 365);
    let _can_disable: bool = true;
    let user1 = env.next_user();
    let _fund_admins: Vec<String> = vec![
        user1.to_formatted_string(),
        env.next_user().to_formatted_string(),
        env.next_user().to_formatted_string(),
        env.next_user().to_formatted_string(),
    ];
    let token: TestContract = VESTINGESCROWInstance::new_deploy(
        &env,
        NAME,
        owner,
        Key::Hash(_token.package_hash()),
        _start_time,
        _end_time,
        _can_disable,
        _fund_admins,
    );
    (
        env,
        VESTINGESCROWInstance::instance(token),
        owner,
        user1,
        _token,
        time_now,
    )
}

#[test]
fn test_vesting_escrow_deploy() {
    let (env, vesting_escrow_instance, _owner, user1, token, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(vesting_escrow_instance.start_time(), time_now_u256);
    assert_eq!(
        vesting_escrow_instance.end_time(),
        U256::from(time_now + MILLI_SECONDS_IN_DAY * 365)
    );
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(_owner));
    assert!(vesting_escrow_instance.fund_admins(user1));
}

#[test]
fn test_vesting_escrow_disable_fund_admins() {
    let (env, vesting_escrow_instance, _owner, user1, token, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(vesting_escrow_instance.start_time(), time_now_u256);
    assert_eq!(
        vesting_escrow_instance.end_time(),
        U256::from(time_now + MILLI_SECONDS_IN_DAY * 365)
    );
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(_owner));
    assert!(vesting_escrow_instance.fund_admins(user1));
    vesting_escrow_instance.disable_fund_admins(_owner, time_now);
    assert!(!vesting_escrow_instance.fund_admins_enabled());
    assert!(vesting_escrow_instance.can_disable());
}

#[test]
fn test_vesting_escrow_disable_can_disable() {
    let (env, vesting_escrow_instance, _owner, user1, token, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(vesting_escrow_instance.start_time(), time_now_u256);
    assert_eq!(
        vesting_escrow_instance.end_time(),
        U256::from(time_now + MILLI_SECONDS_IN_DAY * 365)
    );
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(_owner));
    assert!(vesting_escrow_instance.fund_admins(user1));
    vesting_escrow_instance.disable_can_disable(_owner, time_now);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.can_disable());
}

#[test]
fn test_vesting_escrow_toggle_disable() {
    let (env, vesting_escrow_instance, _owner, _user1, token, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(vesting_escrow_instance.start_time(), time_now_u256);
    assert_eq!(
        vesting_escrow_instance.end_time(),
        U256::from(time_now + MILLI_SECONDS_IN_DAY * 365)
    );
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(_owner));
    vesting_escrow_instance.toggle_disable(_owner, time_now, _user);
    assert_eq!(vesting_escrow_instance.disabled_at(_owner), 0.into());
    assert_eq!(vesting_escrow_instance.disabled_at(_user), time_now_u256);
}

#[test]
fn test_vesting_escrow_toggle_disable_after_toggle_disable() {
    let (env, vesting_escrow_instance, _owner, _user1, token, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(vesting_escrow_instance.start_time(), time_now_u256);
    assert_eq!(
        vesting_escrow_instance.end_time(),
        U256::from(time_now + MILLI_SECONDS_IN_DAY * 365)
    );
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(_owner));
    vesting_escrow_instance.toggle_disable(_owner, time_now, _user);
    assert_eq!(vesting_escrow_instance.disabled_at(_owner), 0.into());
    assert_eq!(vesting_escrow_instance.disabled_at(_user), time_now_u256);
    vesting_escrow_instance.toggle_disable(_owner, time_now, _user);
    assert_eq!(vesting_escrow_instance.disabled_at(_owner), 0.into());
    assert_eq!(vesting_escrow_instance.disabled_at(_user), 0.into());
}

#[test]
#[should_panic]
fn test_vesting_escrow_toggle_disable_by_user() {
    let (env, vesting_escrow_instance, _owner, user1, token, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(vesting_escrow_instance.start_time(), time_now_u256);
    assert_eq!(
        vesting_escrow_instance.end_time(),
        U256::from(time_now + MILLI_SECONDS_IN_DAY * 365)
    );
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(_owner));
    assert!(vesting_escrow_instance.fund_admins(user1));
    vesting_escrow_instance.disable_can_disable(_owner, time_now);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(vesting_escrow_instance.can_disable());
    vesting_escrow_instance.toggle_disable(_user, time_now, _user);
}

#[test]
#[should_panic]
fn test_vesting_escrow_toggle_disable_when_disabled() {
    let (env, vesting_escrow_instance, _owner, user1, token, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(vesting_escrow_instance.start_time(), time_now_u256);
    assert_eq!(
        vesting_escrow_instance.end_time(),
        U256::from(time_now + MILLI_SECONDS_IN_DAY * 365)
    );
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(_owner));
    assert!(vesting_escrow_instance.fund_admins(user1));
    vesting_escrow_instance.disable_can_disable(_owner, time_now);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.can_disable());
    vesting_escrow_instance.toggle_disable(_owner, time_now, _user);
}

#[test]
fn test_vesting_escrow_add_tokens() {
    let (env, vesting_escrow_instance, owner, user1, token, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(vesting_escrow_instance.start_time(), time_now_u256);
    assert_eq!(
        vesting_escrow_instance.end_time(),
        U256::from(time_now + MILLI_SECONDS_IN_DAY * 365)
    );
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));
    let amount: U256 = U256::from(100 * TEN_E_NINE);
    let value: U256 = U256::from(1000 * TEN_E_NINE);
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        time_now,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(vesting_escrow_instance.package_hash()),
            "amount" => value + value
        },
        time_now,
    );
    vesting_escrow_instance.add_tokens(owner, time_now, amount);
    assert_eq!(vesting_escrow_instance.unallocated_supply(), amount);
    vesting_escrow_instance.add_tokens(owner, time_now, amount);
    assert_eq!(
        vesting_escrow_instance.unallocated_supply(),
        amount + amount
    );
}

#[test]
#[should_panic]
fn test_vesting_escrow_add_tokens_by_user() {
    let (env, vesting_escrow_instance, owner, user1, token, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(vesting_escrow_instance.start_time(), time_now_u256);
    assert_eq!(
        vesting_escrow_instance.end_time(),
        U256::from(time_now + MILLI_SECONDS_IN_DAY * 365)
    );
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));
    let amount: U256 = U256::from(100 * TEN_E_NINE);
    let value: U256 = U256::from(1000 * TEN_E_NINE);
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        time_now,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(vesting_escrow_instance.package_hash()),
            "amount" => value + value
        },
        time_now,
    );
    vesting_escrow_instance.add_tokens(user1, time_now, amount);
    assert_eq!(vesting_escrow_instance.unallocated_supply(), amount);
}

#[test]
fn test_vesting_escrow_fund() {
    let (env, vesting_escrow_instance, owner, user1, token, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(vesting_escrow_instance.start_time(), time_now_u256);
    assert_eq!(
        vesting_escrow_instance.end_time(),
        U256::from(time_now + MILLI_SECONDS_IN_DAY * 365)
    );
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));

    let amount: U256 = U256::from(100 * TEN_E_NINE);
    let value: U256 = U256::from(1000 * TEN_E_NINE);
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        time_now,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(vesting_escrow_instance.package_hash()),
            "amount" => value + value
        },
        time_now,
    );
    vesting_escrow_instance.add_tokens(owner, time_now, amount);
    assert_eq!(vesting_escrow_instance.unallocated_supply(), amount);
    let user_1 = env.next_user();
    let user_2 = env.next_user();
    let user_3 = env.next_user();
    let user_4 = env.next_user();
    let recipients: Vec<String> = vec![
        user_1.to_formatted_string(),
        user_2.to_formatted_string(),
        user_3.to_formatted_string(),
        user_4.to_formatted_string(),
    ];
    let _amounts: Vec<U256> = vec![1.into(), 2.into(), 3.into(), 4.into()];
    vesting_escrow_instance.fund(owner, time_now, recipients, _amounts);
    assert_eq!(vesting_escrow_instance.initial_locked_supply(), 10.into());
    assert_eq!(
        vesting_escrow_instance.unallocated_supply(),
        99999999990_i64.into()
    );
    assert_eq!(vesting_escrow_instance.initial_locked(user_1), 1.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_2), 2.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_3), 3.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_4), 4.into());
}

#[test]
#[should_panic]
fn test_vesting_escrow_fund_by_user() {
    let (env, vesting_escrow_instance, owner, user1, token, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(vesting_escrow_instance.start_time(), time_now_u256);
    assert_eq!(
        vesting_escrow_instance.end_time(),
        U256::from(time_now + MILLI_SECONDS_IN_DAY * 365)
    );
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));

    let amount: U256 = U256::from(100 * TEN_E_NINE);
    let value: U256 = U256::from(1000 * TEN_E_NINE);
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        time_now,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(vesting_escrow_instance.package_hash()),
            "amount" => value + value
        },
        time_now,
    );
    vesting_escrow_instance.add_tokens(owner, time_now, amount);
    assert_eq!(vesting_escrow_instance.unallocated_supply(), amount);
    let user_1 = env.next_user();
    let user_2 = env.next_user();
    let user_3 = env.next_user();
    let user_4 = env.next_user();
    let recipients: Vec<String> = vec![
        user_1.to_formatted_string(),
        user_2.to_formatted_string(),
        user_3.to_formatted_string(),
        user_4.to_formatted_string(),
    ];
    let _amounts: Vec<U256> = vec![1.into(), 2.into(), 3.into(), 4.into()];
    vesting_escrow_instance.fund(_user, time_now, recipients, _amounts);
    assert_eq!(vesting_escrow_instance.initial_locked_supply(), 10.into());
    assert_eq!(vesting_escrow_instance.unallocated_supply(), 90.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_1), 1.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_2), 2.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_3), 3.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_4), 4.into());
}

#[test]
fn test_vesting_escrow_commit_transfer_ownership() {
    let (env, vesting_escrow_instance, owner, user1, token, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(vesting_escrow_instance.start_time(), time_now_u256);
    assert_eq!(
        vesting_escrow_instance.end_time(),
        U256::from(time_now + MILLI_SECONDS_IN_DAY * 365)
    );
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));
    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(COMMIT_TRANSFER_OWNERSHIP),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "addr"=>Key::from(_user)
        },
        time_now,
    );

    let ret: bool = env.query_account_named_key(owner, &[COMMIT_TRANSFER_OWNERSHIP.into()]);
    assert!(ret);
    assert_eq!(vesting_escrow_instance.future_admin(), Key::from(_user));
}

#[test]
fn test_vesting_escrow_apply_transfer_ownership() {
    let (env, vesting_escrow_instance, owner, user1, token, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(vesting_escrow_instance.start_time(), time_now_u256);
    assert_eq!(
        vesting_escrow_instance.end_time(),
        U256::from(time_now + MILLI_SECONDS_IN_DAY * 365)
    );
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));

    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(COMMIT_TRANSFER_OWNERSHIP),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "addr"=>Key::from(_user)
        },
        time_now,
    );

    let ret: bool = env.query_account_named_key(owner, &[COMMIT_TRANSFER_OWNERSHIP.into()]);
    assert!(ret);
    assert_eq!(vesting_escrow_instance.future_admin(), Key::from(_user));

    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(APPLY_TRANSFER_OWNERSHIP),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "addr"=>Key::from(_user)
        },
        time_now,
    );

    let ret: bool = env.query_account_named_key(owner, &[APPLY_TRANSFER_OWNERSHIP.into()]);
    assert!(ret);
    assert_eq!(vesting_escrow_instance.future_admin(), Key::from(_user));
    assert_eq!(vesting_escrow_instance.admin(), Key::from(_user));
}

#[test]
fn test_vesting_escrow_vested_supply() {
    let (env, vesting_escrow_instance, owner, user1, token, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(vesting_escrow_instance.start_time(), time_now_u256);
    assert_eq!(
        vesting_escrow_instance.end_time(),
        U256::from(time_now + MILLI_SECONDS_IN_DAY * 365)
    );
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));
    let amount: U256 = U256::from(100 * TEN_E_NINE);
    let value: U256 = U256::from(1000 * TEN_E_NINE);
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        time_now,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(vesting_escrow_instance.package_hash()),
            "amount" => value + value
        },
        time_now,
    );
    vesting_escrow_instance.add_tokens(owner, time_now, amount);
    assert_eq!(vesting_escrow_instance.unallocated_supply(), amount);
    let user_1 = env.next_user();
    let user_2 = env.next_user();
    let user_3 = env.next_user();
    let user_4 = env.next_user();
    let recipients: Vec<String> = vec![
        user_1.to_formatted_string(),
        user_2.to_formatted_string(),
        user_3.to_formatted_string(),
        user_4.to_formatted_string(),
    ];
    let _amounts: Vec<U256> = vec![
        U256::from(TEN_E_NINE),
        U256::from(2 * TEN_E_NINE),
        U256::from(3 * TEN_E_NINE),
        U256::from(4 * TEN_E_NINE),
    ];
    vesting_escrow_instance.fund(owner, time_now, recipients, _amounts);
    assert_eq!(
        vesting_escrow_instance.initial_locked_supply(),
        U256::from(10 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.unallocated_supply(),
        90000000000_i64.into()
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_1),
        U256::from(TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_2),
        U256::from(2 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_3),
        U256::from(3 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_4),
        U256::from(4 * TEN_E_NINE)
    );

    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VESTED_SUPPLY),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
        },
        time_now + MILLI_SECONDS_IN_DAY * 100,
    );

    let ret: U256 = env.query_account_named_key(owner, &[VESTED_SUPPLY.into()]);
    assert_eq!(ret, 2739726027_i64.into());
}

#[test]
fn test_vesting_escrow_locked_supply() {
    let (env, vesting_escrow_instance, owner, user1, token, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(vesting_escrow_instance.start_time(), time_now_u256);
    assert_eq!(
        vesting_escrow_instance.end_time(),
        U256::from(time_now + MILLI_SECONDS_IN_DAY * 365)
    );
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));

    let amount: U256 = U256::from(100 * TEN_E_NINE);
    let value: U256 = U256::from(1000 * TEN_E_NINE);
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        time_now,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(vesting_escrow_instance.package_hash()),
            "amount" => value + value
        },
        time_now,
    );
    vesting_escrow_instance.add_tokens(owner, time_now, amount);
    assert_eq!(vesting_escrow_instance.unallocated_supply(), amount);
    let user_1 = env.next_user();
    let user_2 = env.next_user();
    let user_3 = env.next_user();
    let user_4 = env.next_user();
    let recipients: Vec<String> = vec![
        user_1.to_formatted_string(),
        user_2.to_formatted_string(),
        user_3.to_formatted_string(),
        user_4.to_formatted_string(),
    ];
    let _amounts: Vec<U256> = vec![
        U256::from(2 * TEN_E_NINE),
        U256::from(3 * TEN_E_NINE),
        U256::from(4 * TEN_E_NINE),
        U256::from(5 * TEN_E_NINE),
    ];
    vesting_escrow_instance.fund(owner, time_now, recipients, _amounts);
    assert_eq!(
        vesting_escrow_instance.initial_locked_supply(),
        U256::from(14 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.unallocated_supply(),
        U256::from(86 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_1),
        U256::from(2 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_2),
        U256::from(3 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_3),
        U256::from(4 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_4),
        U256::from(5 * TEN_E_NINE)
    );

    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(LOCKED_SUPPLY),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
        },
        time_now + MILLI_SECONDS_IN_DAY * 10,
    );

    let ret: U256 = env.query_account_named_key(owner, &[LOCKED_SUPPLY.into()]);
    assert_eq!(ret, 13616438357_i64.into());
}

#[test]
fn test_vesting_escrow_vested_of() {
    let (env, vesting_escrow_instance, owner, user1, token, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(vesting_escrow_instance.start_time(), time_now_u256);
    assert_eq!(
        vesting_escrow_instance.end_time(),
        U256::from(time_now + MILLI_SECONDS_IN_DAY * 365)
    );
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));

    let amount: U256 = U256::from(100 * TEN_E_NINE);
    let value: U256 = U256::from(1000 * TEN_E_NINE);
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        0,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(vesting_escrow_instance.package_hash()),
            "amount" => value + value
        },
        0,
    );
    vesting_escrow_instance.add_tokens(owner, time_now, amount);
    assert_eq!(vesting_escrow_instance.unallocated_supply(), amount);
    let user_1 = env.next_user();
    let user_2 = env.next_user();
    let user_3 = env.next_user();
    let user_4 = env.next_user();
    let recipients: Vec<String> = vec![
        user_1.to_formatted_string(),
        user_2.to_formatted_string(),
        user_3.to_formatted_string(),
        user_4.to_formatted_string(),
    ];
    let _amounts: Vec<U256> = vec![
        U256::from(2 * TEN_E_NINE),
        U256::from(3 * TEN_E_NINE),
        U256::from(4 * TEN_E_NINE),
        U256::from(5 * TEN_E_NINE),
    ];
    vesting_escrow_instance.fund(owner, time_now, recipients, _amounts);
    assert_eq!(
        vesting_escrow_instance.initial_locked_supply(),
        U256::from(14 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.unallocated_supply(),
        U256::from(86 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_1),
        U256::from(2 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_2),
        U256::from(3 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_3),
        U256::from(4 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_4),
        U256::from(5 * TEN_E_NINE)
    );
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VESTED_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "recipient" => Key::Account(user_1),
        },
        time_now + MILLI_SECONDS_IN_DAY * 365,
    );

    let ret: U256 = env.query_account_named_key(owner, &[VESTED_OF.into()]);
    assert_eq!(ret, U256::from(2 * TEN_E_NINE));
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VESTED_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "recipient" => Key::Account(user_2),
        },
        time_now + MILLI_SECONDS_IN_DAY * 365,
    );

    let ret: U256 = env.query_account_named_key(owner, &[VESTED_OF.into()]);
    assert_eq!(ret, U256::from(3 * TEN_E_NINE));
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VESTED_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "recipient" => Key::Account(user_3),
        },
        time_now + MILLI_SECONDS_IN_DAY * 365,
    );

    let ret: U256 = env.query_account_named_key(owner, &[VESTED_OF.into()]);
    assert_eq!(ret, U256::from(4 * TEN_E_NINE));
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VESTED_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "recipient" => Key::Account(user_4),
        },
        time_now + MILLI_SECONDS_IN_DAY * 365,
    );

    let ret: U256 = env.query_account_named_key(owner, &[VESTED_OF.into()]);
    assert_eq!(ret, U256::from(5 * TEN_E_NINE));
}

#[test]
fn test_vesting_escrow_balance_of() {
    let (env, vesting_escrow_instance, owner, user1, token, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(vesting_escrow_instance.start_time(), time_now_u256);
    assert_eq!(
        vesting_escrow_instance.end_time(),
        U256::from(time_now + MILLI_SECONDS_IN_DAY * 365)
    );
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));

    let amount: U256 = U256::from(100 * TEN_E_NINE);
    let value: U256 = U256::from(1000 * TEN_E_NINE);
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        time_now,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(vesting_escrow_instance.package_hash()),
            "amount" => value + value
        },
        time_now,
    );
    vesting_escrow_instance.add_tokens(owner, time_now, amount);
    assert_eq!(vesting_escrow_instance.unallocated_supply(), amount);
    let user_1 = env.next_user();
    let user_2 = env.next_user();
    let user_3 = env.next_user();
    let user_4 = env.next_user();
    let recipients: Vec<String> = vec![
        user_1.to_formatted_string(),
        user_2.to_formatted_string(),
        user_3.to_formatted_string(),
        user_4.to_formatted_string(),
    ];
    let _amounts: Vec<U256> = vec![
        U256::from(2 * TEN_E_NINE),
        U256::from(3 * TEN_E_NINE),
        U256::from(4 * TEN_E_NINE),
        U256::from(5 * TEN_E_NINE),
    ];
    vesting_escrow_instance.fund(owner, time_now, recipients, _amounts);
    assert_eq!(
        vesting_escrow_instance.initial_locked_supply(),
        U256::from(14 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.unallocated_supply(),
        U256::from(86 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_1),
        U256::from(2 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_2),
        U256::from(3 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_3),
        U256::from(4 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_4),
        U256::from(5 * TEN_E_NINE)
    );
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "address" => Key::Account(user_1),
        },
        time_now + MILLI_SECONDS_IN_DAY * 365,
    );

    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, U256::from(2 * TEN_E_NINE));
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "address" => Key::Account(user_2),
        },
        time_now + MILLI_SECONDS_IN_DAY * 365,
    );

    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, U256::from(3 * TEN_E_NINE));
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "address" => Key::Account(user_3),
        },
        time_now + MILLI_SECONDS_IN_DAY * 365,
    );

    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, U256::from(4 * TEN_E_NINE));
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "address" => Key::Account(user_4),
        },
        time_now + MILLI_SECONDS_IN_DAY * 365,
    );

    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, U256::from(5 * TEN_E_NINE));
}

#[test]
fn test_vesting_escrow_locked_of() {
    let (env, vesting_escrow_instance, owner, user1, token, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(vesting_escrow_instance.start_time(), time_now_u256);
    assert_eq!(
        vesting_escrow_instance.end_time(),
        U256::from(time_now + MILLI_SECONDS_IN_DAY * 365)
    );
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));

    let amount: U256 = U256::from(100 * TEN_E_NINE);
    let value: U256 = U256::from(1000 * TEN_E_NINE);
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        0,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(vesting_escrow_instance.package_hash()),
            "amount" => value + value
        },
        0,
    );
    vesting_escrow_instance.add_tokens(owner, time_now, amount);
    assert_eq!(vesting_escrow_instance.unallocated_supply(), amount);
    let user_1 = env.next_user();
    let user_2 = env.next_user();
    let user_3 = env.next_user();
    let user_4 = env.next_user();
    let recipients: Vec<String> = vec![
        user_1.to_formatted_string(),
        user_2.to_formatted_string(),
        user_3.to_formatted_string(),
        user_4.to_formatted_string(),
    ];
    let _amounts: Vec<U256> = vec![
        U256::from(2 * TEN_E_NINE),
        U256::from(3 * TEN_E_NINE),
        U256::from(4 * TEN_E_NINE),
        U256::from(5 * TEN_E_NINE),
    ];
    vesting_escrow_instance.fund(owner, time_now, recipients, _amounts);
    assert_eq!(
        vesting_escrow_instance.initial_locked_supply(),
        U256::from(14 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.unallocated_supply(),
        U256::from(86 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_1),
        U256::from(2 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_2),
        U256::from(3 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_3),
        U256::from(4 * TEN_E_NINE)
    );
    assert_eq!(
        vesting_escrow_instance.initial_locked(user_4),
        U256::from(5 * TEN_E_NINE)
    );
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(LOCKED_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "recipient" => Key::Account(user_1),
        },
        time_now,
    );

    let ret: U256 = env.query_account_named_key(owner, &[LOCKED_OF.into()]);
    assert_eq!(ret, U256::from(2 * TEN_E_NINE));
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(LOCKED_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "recipient" => Key::Account(user_2),
        },
        time_now,
    );

    let ret: U256 = env.query_account_named_key(owner, &[LOCKED_OF.into()]);
    assert_eq!(ret, U256::from(3 * TEN_E_NINE));
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(LOCKED_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "recipient" => Key::Account(user_3),
        },
        time_now,
    );

    let ret: U256 = env.query_account_named_key(owner, &[LOCKED_OF.into()]);
    assert_eq!(ret, U256::from(4 * TEN_E_NINE));
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(LOCKED_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "recipient" => Key::Account(user_4),
        },
        time_now,
    );

    let ret: U256 = env.query_account_named_key(owner, &[LOCKED_OF.into()]);
    assert_eq!(ret, U256::from(5 * TEN_E_NINE));
}
