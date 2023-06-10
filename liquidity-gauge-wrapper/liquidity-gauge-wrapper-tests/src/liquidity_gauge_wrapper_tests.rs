use crate::liquidity_gauge_wrapper_instance::LIQUIDITYGAUGEWRAPPERInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U128, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;
use crv20::Address;
//Const
pub const TEN_E_NINE: u128 = 1000000000;
const NAME: &str = "LiquidityGuageWrapper";
//ERC20
fn deploy_erc20(env: &TestEnv, owner: AccountHash, block_time: u64) -> TestContract {
    TestContract::new(
        env,
        "curve-erc20.wasm",
        "rewarded_token",
        owner,
        runtime_args! {
            "name" => "rewarded_token",
            "symbol" => "ERA",
            "decimals" => 9_u8,
            "initial_supply" => U256::from(TEN_E_NINE * 100000000000000000000)
        },
        block_time,
    )
}
// CRV
fn deploy_erc20_crv(env: &TestEnv, sender: AccountHash, block_time: u64) -> TestContract {
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
// Voting Escrow
fn deploy_voting_escrow(
    env: &TestEnv,
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
        "voting-escrow",
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
//gauge_controller
fn deploy_gauge_controller(
    env: &TestEnv,
    sender: AccountHash,
    token: Key,
    voting_escrow: Key,
    block_time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "gauge-controller-token.wasm",
        "gauge-controller",
        sender,
        runtime_args! {
            "token" => token,
            "voting_escrow" => voting_escrow,
        },
        block_time,
    )
}
//Minter
fn deploy_minter(
    env: &TestEnv,
    sender: AccountHash,
    controller: Key,
    token: Key,
    block_time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "minter-token.wasm",
        "minter",
        sender,
        runtime_args! {
            "controller" => controller,
            "token" => token,
        },
        block_time,
    )
}
// Liquidity Gauge V3
fn deploy_liquidity_gauge_v3(
    env: &TestEnv,
    sender: AccountHash,
    lp_addr: Key,
    minter: Key,
    admin: Key,
    block_time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "liquidity-gauge-v3.wasm",
        "liquidity-gauge-v3",
        sender,
        runtime_args! {
            "lp_addr" => lp_addr,
            "minter" => minter,
            "admin" => admin,
        },
        block_time,
    )
}
fn deploy() -> (TestEnv, AccountHash, TestContract, u64) {
    let block_time = LIQUIDITYGAUGEWRAPPERInstance::now();
    let env = TestEnv::new();
    let owner = env.next_user();
    let erc20 = deploy_erc20(&env, owner, block_time);
    let erc20_crv = deploy_erc20_crv(&env, owner, block_time);
    let voting_escrow = deploy_voting_escrow(
        &env,
        owner,
        Key::Hash(erc20.package_hash()),
        "Voting Escrow".into(),
        "VT".into(),
        "1".into(),
        block_time,
    );
    let gauge_controller = deploy_gauge_controller(
        &env,
        owner,
        Key::Hash(erc20.package_hash()),
        Key::Hash(voting_escrow.package_hash()),
        block_time,
    );
    let minter = deploy_minter(
        &env,
        owner,
        Key::Hash(gauge_controller.package_hash()),
        Key::Hash(erc20_crv.package_hash()),
        block_time,
    );
    let deploy_liquidity_gauge_v3 = deploy_liquidity_gauge_v3(
        &env,
        owner,
        Key::Hash(erc20.package_hash()),
        Key::Hash(minter.package_hash()),
        Key::Account(owner),
        block_time,
    );
    let liquidity_gauge_wrapper_instance = LIQUIDITYGAUGEWRAPPERInstance::new_deploy(
        &env,
        NAME,
        owner,
        "Gauge Wrapper".to_string(),
        "LGW".to_string(),
        Key::Hash(deploy_liquidity_gauge_v3.package_hash()),
        Key::Account(owner),
        block_time,
    );
    erc20_crv.call_contract(
        owner,
        "set_minter",
        runtime_args! {"minter"=>Key::Hash(minter.package_hash())},
        block_time,
    );
    // For Minting Purpose
    let to = Key::Hash(liquidity_gauge_wrapper_instance.package_hash());
    let amount: U256 = U256::from(TEN_E_NINE * 100000000000000000000);
    let amount_1: U256 = U256::from(TEN_E_NINE * 100);
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount},
        block_time,
    );
    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {"spender" => to , "amount" => amount},
        block_time,
    );
    erc20_crv.call_contract(
        owner,
        "set_minter",
        runtime_args! {"minter" => Key::Account(owner)},
        block_time,
    );
    erc20_crv.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount_1},
        block_time + 8640000000,
    );

    let _name: String = "type".to_string();
    gauge_controller.call_contract(
        owner,
        "add_type",
        runtime_args! {"name" => _name, "weight" => Some(U256::from(1)) },
        block_time,
    );
    let addr: Key = Key::Account(owner);
    let gauge_type: (bool, U128) = (false, 0.into());
    gauge_controller.call_contract(
        owner,
        "add_gauge",
        runtime_args! {
            "addr" => addr,
            "gauge_type" => gauge_type,
            "weight"=>Some(U256::from(10))
        },
        block_time,
    );
    let _name_1: String = "type1".to_string();
    gauge_controller.call_contract(
        owner,
        "add_type",
        runtime_args! {"name" => _name_1, "weight" => Some(U256::from(100)) },
        block_time,
    );
    let addr1: Key = Key::Hash(deploy_liquidity_gauge_v3.package_hash());
    let gauge_type_1: (bool, U128) = (false, 1.into());
    gauge_controller.call_contract(
        owner,
        "add_gauge",
        runtime_args! {
            "addr" => addr1,
            "gauge_type" => gauge_type_1,
            "weight"=>Some(U256::from(1000))
        },
        block_time,
    );
    (env, owner, liquidity_gauge_wrapper_instance, block_time)
}
mod checkpoint_deploy_and_claim_test_cases {
    use crate::liquidity_gauge_wrapper_tests::*;
    #[test]
    fn test_deploy() {
        let (_, _, _, _) = deploy();
    }
    #[test]
    fn test_user_checkpoint() {
        let (env, owner, instance, block_time) = deploy();
        let package_hash = Key::Hash(instance.package_hash());
        let addr: Key = Key::Account(owner);
        TestContract::new(
            &env,
            "liquidity-gauge-wrapper-session-code.wasm",
            SESSION_CODE_NAME,
            owner,
            runtime_args! {
                "entrypoint" => String::from(USER_CHECKPOINT),
                "package_hash" => package_hash,
                "addr" => addr,
            },
            block_time,
        );
    }
    #[test]
    fn test_claimable_tokens() {
        let (env, owner, instance, block_time) = deploy();
        let package_hash = Key::Hash(instance.package_hash());
        let addr: Key = Key::Account(owner);
        TestContract::new(
            &env,
            "liquidity-gauge-wrapper-session-code.wasm",
            SESSION_CODE_NAME,
            owner,
            runtime_args! {
                "entrypoint" => String::from(CLAIMABLE_TOKENS),
                "package_hash" => package_hash,
                "addr" => addr,
            },
            block_time,
        );
    }
    #[test]
    fn test_claim_tokens() {
        let (_, owner, instance, block_time) = deploy();
        let liquidity_gauge_wrapper_instance =
            LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
        liquidity_gauge_wrapper_instance.claim_tokens(owner, None, block_time);
    }
}
#[test]
fn test_set_approve_deposit() {
    let (env, owner, instance, block_time) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let addr: Key = Key::Account(owner);
    let liquidity_gauge_wrapper_instance =
        LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_wrapper_instance.set_approve_deposit(owner, addr, true, block_time);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(APPROVED_TO_DEPOSIT),
            "package_hash" => package_hash,
            "owner" => Key::Account(owner) ,
            "spender" => Key::Account(owner)
        },
        block_time,
    );
    let ret: bool = env.query_account_named_key(owner, &[APPROVED_TO_DEPOSIT.into()]);
    assert!(ret, "{} {}", true, "Invalid result");
}
#[test]
fn test_deposit() {
    let (_env, owner, instance, block_time) = deploy();
    let liquidity_gauge_wrapper_instance =
        LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_wrapper_instance.deposit(owner, U256::from(TEN_E_NINE * 10), None, block_time);
    assert_eq!(
        liquidity_gauge_wrapper_instance.balance_of(Address::Account(owner)),
        U256::from(TEN_E_NINE * 10),
        "Invalid result"
    );
}

#[test]
fn test_withdraw() {
    let (_env, owner, instance, block_time) = deploy();
    let addr: Key = Key::Account(owner);
    let liquidity_gauge_wrapper_instance =
        LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_wrapper_instance.deposit(
        owner,
        U256::from(TEN_E_NINE * 1000),
        None,
        block_time,
    );
    liquidity_gauge_wrapper_instance.withdraw(owner, U256::from(TEN_E_NINE * 10), addr, block_time);
    let v: u128 = 990000000000_u128;
    assert_eq!(
        liquidity_gauge_wrapper_instance.balance_of(Address::Account(owner)),
        v.into(),
        "Invalid result"
    );
}
mod transfer_and_transfer_from_test_cases {
    use crate::liquidity_gauge_wrapper_tests::*;
    #[test]
    fn test_transfer() {
        let (env, owner, instance, block_time) = deploy();
        let recipient: Key = env.next_user().into();
        let liquidity_gauge_wrapper_instance =
            LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
        liquidity_gauge_wrapper_instance.deposit(
            owner,
            U256::from(TEN_E_NINE * 1000),
            None,
            block_time,
        );
        liquidity_gauge_wrapper_instance.transfer(
            owner,
            recipient,
            U256::from(TEN_E_NINE * 10),
            block_time,
        );
        let v: u128 = 990000000000_u128;
        assert_eq!(
            liquidity_gauge_wrapper_instance.balance_of(Address::Account(owner)),
            v.into(),
            "Invalid result"
        );
    }
    #[test]
    fn test_transfer_from() {
        let (env, owner, instance, block_time) = deploy();
        let recipient: Key = env.next_user().into();
        let liquidity_gauge_wrapper_instance =
            LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
        liquidity_gauge_wrapper_instance.deposit(
            owner,
            U256::from(TEN_E_NINE * 1000),
            None,
            block_time,
        );
        liquidity_gauge_wrapper_instance.approve(
            owner,
            Key::Account(owner),
            U256::from(TEN_E_NINE * 100),
            block_time,
        );
        liquidity_gauge_wrapper_instance.transfer_from(
            owner,
            Key::Account(owner),
            recipient,
            U256::from(TEN_E_NINE * 10),
            block_time,
        );
        let v: u128 = 90000000000_u128;
        assert_eq!(
            liquidity_gauge_wrapper_instance
                .allowance(Address::Account(owner), Address::Account(owner)),
            v.into(),
            "Invalid result"
        );
    }
}
mod allowance_and_approve_functions_test_cases_1 {
    use crate::liquidity_gauge_wrapper_tests::*;
    #[test]
    fn test_approve() {
        let (_env, owner, instance, block_time) = deploy();
        let liquidity_gauge_wrapper_instance =
            LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
        liquidity_gauge_wrapper_instance.approve(
            owner,
            Key::Account(owner),
            U256::from(TEN_E_NINE * 100),
            block_time,
        );
        let v: u128 = 100000000000_u128;
        assert_eq!(
            liquidity_gauge_wrapper_instance
                .allowance(Address::Account(owner), Address::Account(owner)),
            v.into(),
            "Invalid result"
        );
    }
    #[test]
    fn test_allowance() {
        let (_env, owner, instance, block_time) = deploy();
        let liquidity_gauge_wrapper_instance =
            LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
        liquidity_gauge_wrapper_instance.deposit(
            owner,
            U256::from(TEN_E_NINE * 1000),
            None,
            block_time,
        );
        liquidity_gauge_wrapper_instance.approve(
            owner,
            Key::Account(owner),
            U256::from(TEN_E_NINE * 100),
            block_time,
        );
        let v: u128 = 100000000000_u128;
        assert_eq!(
            liquidity_gauge_wrapper_instance
                .allowance(Address::Account(owner), Address::Account(owner)),
            v.into(),
            "Invalid result"
        );
    }
}
mod allowance_and_approve_functions_test_cases_2 {
    use crate::liquidity_gauge_wrapper_tests::*;
    #[test]
    fn test_increase_allowance() {
        let (env, owner, instance, block_time) = deploy();
        let liquidity_gauge_wrapper_instance =
            LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
        let user1 = env.next_user();
        liquidity_gauge_wrapper_instance.approve(
            owner,
            Key::Account(user1),
            U256::from(TEN_E_NINE * 100),
            block_time,
        );
        liquidity_gauge_wrapper_instance.increase_allowance(
            owner,
            Key::Account(user1),
            U256::from(TEN_E_NINE * 10),
            block_time,
        );
        let v: u128 = 110000000000_u128;
        assert_eq!(
            liquidity_gauge_wrapper_instance
                .allowance(Address::Account(owner), Address::Account(user1)),
            v.into(),
            "Invalid result"
        );
    }
    #[test]
    fn test_decrease_allowance() {
        let (env, owner, instance, block_time) = deploy();
        let user1 = env.next_user();
        let liquidity_gauge_wrapper_instance =
            LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
        liquidity_gauge_wrapper_instance.approve(
            owner,
            Key::Account(user1),
            U256::from(TEN_E_NINE * 100),
            block_time,
        );
        liquidity_gauge_wrapper_instance.decrease_allowance(
            owner,
            Key::Account(user1),
            U256::from(TEN_E_NINE * 10),
            block_time,
        );
        let v: u128 = 90000000000_u128;
        assert_eq!(
            liquidity_gauge_wrapper_instance
                .allowance(Address::Account(owner), Address::Account(user1)),
            v.into(),
            "Invalid result"
        );
    }
}
mod ownership_and_kill_functions_test_cases {
    use crate::liquidity_gauge_wrapper_tests::*;
    #[test]
    fn test_kill_me() {
        let (env, owner, instance, block_time) = deploy();
        let package_hash = Key::Hash(instance.package_hash());
        let liquidity_gauge_wrapper_instance =
            LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
        liquidity_gauge_wrapper_instance.kill_me(owner, block_time);
        TestContract::new(
            &env,
            TEST_SESSION_CODE_WASM,
            TEST_SESSION_CODE_NAME,
            owner,
            runtime_args! {
                "entrypoint" => String::from(IS_KILLED),
                "package_hash" => package_hash,
            },
            block_time,
        );
        let ret: bool = env.query_account_named_key(owner, &[IS_KILLED.into()]);
        assert!(ret, "{} {}", true, "Invalid result");
    }
    #[test]
    fn test_commit_transfer_ownership() {
        let (env, owner, instance, block_time) = deploy();
        let package_hash = Key::Hash(instance.package_hash());
        let liquidity_gauge_wrapper_instance =
            LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
        let addr: Key = Key::Account(owner);
        liquidity_gauge_wrapper_instance.commit_transfer_ownership(owner, addr, block_time);
        TestContract::new(
            &env,
            TEST_SESSION_CODE_WASM,
            TEST_SESSION_CODE_NAME,
            owner,
            runtime_args! {
                "entrypoint" => String::from(FUTURE_ADMIN),
                "package_hash" => package_hash,
            },
            block_time,
        );
        let ret: Key = env.query_account_named_key(owner, &[FUTURE_ADMIN.into()]);
        assert_eq!(ret, addr, "Invalid result");
    }

    #[test]
    fn test_apply_transfer_ownership() {
        let (env, owner, instance, block_time) = deploy();
        let package_hash = Key::Hash(instance.package_hash());
        let addr: Key = Key::Account(owner);
        let liquidity_gauge_wrapper_instance =
            LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
        liquidity_gauge_wrapper_instance.commit_transfer_ownership(owner, addr, block_time);
        liquidity_gauge_wrapper_instance.apply_transfer_ownership(owner, block_time);
        TestContract::new(
            &env,
            TEST_SESSION_CODE_WASM,
            TEST_SESSION_CODE_NAME,
            owner,
            runtime_args! {
                "entrypoint" => String::from(ADMIN),
                "package_hash" => package_hash,
            },
            block_time,
        );
        let ret: Key = env.query_account_named_key(owner, &[ADMIN.into()]);
        assert_eq!(ret, addr, "Invalid result");
    }
}
mod panic_test_cases {
    use crate::liquidity_gauge_wrapper_tests::*;
    #[should_panic]
    #[test]
    fn test_panic_apply_transfer_ownership() {
        let (_, owner, instance, block_time) = deploy();
        let liquidity_gauge_wrapper_instance =
            LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
        liquidity_gauge_wrapper_instance.apply_transfer_ownership(owner, block_time);
    }
    #[should_panic]
    #[test]
    fn test_panic_decrease_allowance() {
        let (_, owner, instance, block_time) = deploy();
        let liquidity_gauge_wrapper_instance =
            LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
        liquidity_gauge_wrapper_instance.decrease_allowance(
            owner,
            Key::Account(owner),
            U256::from(TEN_E_NINE * 10),
            block_time,
        );
    }
    #[should_panic]
    #[test]
    fn test_panic_transfer_from() {
        let (env, owner, instance, block_time) = deploy();
        let recipient: Key = env.next_user().into();
        let liquidity_gauge_wrapper_instance =
            LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
        liquidity_gauge_wrapper_instance.transfer_from(
            owner,
            Key::Account(owner),
            recipient,
            100000000.into(),
            block_time,
        );
    }
    #[should_panic]
    #[test]
    fn test_panic_transfer() {
        let (env, owner, instance, block_time) = deploy();
        let recipient: Key = env.next_user().into();
        let liquidity_gauge_wrapper_instance =
            LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
        liquidity_gauge_wrapper_instance.transfer(
            owner,
            recipient,
            U256::from(TEN_E_NINE * 10),
            block_time,
        );
    }
    #[should_panic]
    #[test]
    fn test_panic_withdraw() {
        let (_, owner, instance, block_time) = deploy();
        let addr: Key = Key::Account(owner);
        let liquidity_gauge_wrapper_instance =
            LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
        liquidity_gauge_wrapper_instance.withdraw(
            owner,
            U256::from(TEN_E_NINE * 10),
            addr,
            block_time,
        );
    }
}