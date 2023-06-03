use crate::liquidity_gauge_reward_instance::LIQUIDITYGAUGEREWARDInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;
use crv20::Address;

fn deploy_erc20(env: &TestEnv, sender: AccountHash, blocktime: u64) -> TestContract {
    TestContract::new(
        env,
        "curve-erc20.wasm",
        "erc20",
        sender,
        runtime_args! {
            "initial_supply" => U256::from(1000000000000_u128),
            "name" => "Token",
            "symbol" => "ERC20",
            "decimals" => 9_u8
        },
        blocktime,
    )
}

fn deploy_erc20_crv(env: &TestEnv, sender: AccountHash, blocktime: u64) -> TestContract {
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
        blocktime,
    )
}

fn deploy_curve_rewards(
    env: &TestEnv,
    sender: AccountHash,
    token: Key,
    reward: Key,
    blocktime: u64,
) -> TestContract {
    TestContract::new(
        env,
        "curve-rewards.wasm",
        "curve-rewards",
        sender,
        runtime_args! {
            "token" => token,
            "reward" => reward
        },
        blocktime,
    )
}

fn deploy_voting_escrow(
    env: &TestEnv,
    sender: AccountHash,
    token_addr: Key,
    name: String,
    symbol: String,
    version: String,
    blocktime: u64,
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
        blocktime,
    )
}

fn deploy_gauge_controller(
    env: &TestEnv,
    sender: AccountHash,
    token: Key,
    voting_escrow: Key,
    blocktime: u64,
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
        blocktime,
    )
}

fn deploy_minter(
    env: &TestEnv,
    sender: AccountHash,
    controller: Key,
    token: Key,
    blocktime: u64,
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
        blocktime,
    )
}

fn deploy() -> (
    TestEnv,
    AccountHash,
    LIQUIDITYGAUGEREWARDInstance,
    TestContract,
    u64,
) {
    let blocktime = LIQUIDITYGAUGEREWARDInstance::now();
    let env = TestEnv::new();
    let owner = env.next_user();
    let erc20 = deploy_erc20(&env, owner, blocktime);
    let token = deploy_erc20(&env, owner, blocktime);

    let erc20_crv = deploy_erc20_crv(&env, owner, blocktime);
    let voting_escrow = deploy_voting_escrow(
        &env,
        owner,
        Key::Hash(erc20.package_hash()),
        "Voting Escrow".into(),
        "VT".into(),
        "1".into(),
        blocktime,
    );
    let gauge_controller = deploy_gauge_controller(
        &env,
        owner,
        Key::Hash(erc20.package_hash()),
        Key::Hash(voting_escrow.package_hash()),
        blocktime,
    );
    let minter = deploy_minter(
        &env,
        owner,
        Key::Hash(gauge_controller.package_hash()),
        Key::Hash(erc20_crv.package_hash()),
        blocktime,
    );
    let curve_rewards = deploy_curve_rewards(
        &env,
        owner,
        Key::Hash(token.package_hash()),
        Key::Hash(token.package_hash()),
        blocktime,
    );
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Hash(curve_rewards.package_hash()),
            "amount" => U256::from(10000000000000_u128)
        },
        blocktime,
    );
    let instance = LIQUIDITYGAUGEREWARDInstance::new_deploy(
        &env,
        "Liquidity Gauge Reward",
        owner,
        Key::Hash(erc20.package_hash()),
        Key::Hash(minter.package_hash()),
        Key::Hash(curve_rewards.package_hash()),
        Key::Hash(erc20.package_hash()),
        Key::Account(owner),
        blocktime,
    );
    (env, owner, instance, erc20, blocktime)
}
mod deploy_checkpoints_and_claimable_tokens_and_reward_test_cases {
    use crate::liquidity_gauge_reward_tests::*;
    #[test]
    fn test_deploy() {
        let (_, _, _, _, _) = deploy();
    }

    #[test]
    fn test_user_checkpoint() {
        let (env, owner, instance, _, blocktime) = deploy();
        let package_hash = Key::Hash(instance.package_hash());
        let addr: Key = Key::Account(owner);
        TestContract::new(
            &env,
            "liquidity-gauge-reward-session-code.wasm",
            SESSION_CODE_NAME,
            owner,
            runtime_args! {
                "entrypoint" => String::from(USER_CHECKPOINT),
                "package_hash" => package_hash,
                "addr" => addr,
            },
            blocktime,
        );
        let ret: bool = env.query_account_named_key(owner, &[USER_CHECKPOINT.into()]);
        assert!(ret, "{} {}", true, "Invalid result");
    }
    //This function output depends on staking thats why no assert added
    // And staking is not included
    #[test]
    fn test_claimable_tokens() {
        let (env, owner, instance, _, blocktime) = deploy();
        let package_hash = Key::Hash(instance.package_hash());
        let addr: Key = Key::Account(owner);
        TestContract::new(
            &env,
            "liquidity-gauge-reward-session-code.wasm",
            SESSION_CODE_NAME,
            owner,
            runtime_args! {
                "entrypoint" => String::from(CLAIMABLE_TOKENS),
                "package_hash" => package_hash,
                "addr" => addr,
            },
            blocktime,
        );
    }
    //This function output depends on staking thats why no assert added
    // Staking not included
    #[test]
    fn test_claimable_reward() {
        let (env, owner, instance, _, blocktime) = deploy();
        let package_hash = Key::Hash(instance.package_hash());
        let addr: Key = Key::Account(owner);
        TestContract::new(
            &env,
            TEST_SESSION_CODE_WASM,
            TEST_SESSION_CODE_NAME,
            owner,
            runtime_args! {
                "entrypoint" => String::from(CLAIMABLE_REWARD),
                "package_hash" => package_hash,
                "addr" => addr,
            },
            blocktime,
        );
    }

    //This function output depends on staking thats why no assert added
    // Staking not included

    #[test]
    fn test_claim_rewards() {
        let (_env, owner, instance, _, blocktime) = deploy();
        instance.claim_rewards(owner, None, blocktime);
    }

    #[test]
    fn test_integrate_checkpoint() {
        let (env, owner, instance, _, blocktime) = deploy();
        let package_hash = Key::Hash(instance.package_hash());
        TestContract::new(
            &env,
            TEST_SESSION_CODE_WASM,
            TEST_SESSION_CODE_NAME,
            owner,
            runtime_args! {
                "entrypoint" => String::from(INTEGRATE_CHECKPOINT),
                "package_hash" => package_hash,
            },
            blocktime,
        );
        let ret: U256 = env.query_account_named_key(owner, &[INTEGRATE_CHECKPOINT.into()]);
        assert_eq!(ret, U256::from(blocktime), "Invalid result");
    }
}
mod deposit_withdraw_kill_me_and_approve_test_cases {
    use crate::liquidity_gauge_reward_tests::*;
    #[test]
    fn test_set_approve_deposit() {
        let (env, owner, instance, _, blocktime) = deploy();
        let addr: Key = Key::Account(owner);
        let package_hash = instance.package_hash();
        let can_deposit: bool = true;
        instance.set_approve_deposit(owner, addr, can_deposit, blocktime);
        TestContract::new(
            &env,
            TEST_SESSION_CODE_WASM,
            TEST_SESSION_CODE_NAME,
            owner,
            runtime_args! {
                "entrypoint" => String::from(APPROVED_TO_DEPOSIT),
                "package_hash" => Key::Hash(package_hash),
                "owner" => Key::Account(owner) ,
                "spender" => Key::Account(owner)
            },
            blocktime,
        );
        let ret: bool = env.query_account_named_key(owner, &[APPROVED_TO_DEPOSIT.into()]);
        assert!(ret, "{} {}", true, "Invalid result");
    }

    #[test]
    fn test_deposit() {
        let (env, owner, instance, erc20, blocktime) = deploy();
        let value: U256 = U256::from(10000000000_u128);
        erc20.call_contract(
            owner,
            "mint",
            runtime_args! {
                "to" => Key::Account(owner),
                "amount" => value
            },
            blocktime,
        );
        erc20.call_contract(
            owner,
            "approve",
            runtime_args! {
                "spender" => Key::Hash(instance.package_hash()),
                "amount" => value
            },
            blocktime,
        );
        TestContract::new(
            &env,
            TEST_SESSION_CODE_WASM,
            TEST_SESSION_CODE_NAME,
            owner,
            runtime_args! {
                "entrypoint" => String::from(BALANCE_OF),
                "package_hash" => Key::Hash(erc20.package_hash()),
                "address" => Key::Account(owner)
            },
            blocktime,
        );
        let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
        assert_eq!(ret, U256::from(1010000000000_u128), "Invalid result");
        instance.deposit(owner, None, value, blocktime);
        assert_eq!(
            instance.balance_of(Address::Account(owner)),
            U256::from(10000000000_u128),
            "Invalid result"
        );
    }

    #[test]
    fn test_withdraw() {
        let (_env, owner, instance, erc20, blocktime) = deploy();
        let claim_rewards: bool = true;
        let value: U256 = U256::from(10000000000_u128);
        erc20.call_contract(
            owner,
            "mint",
            runtime_args! {
                "to" => Key::Account(owner),
                "amount" => value
            },
            blocktime,
        );
        erc20.call_contract(
            owner,
            "approve",
            runtime_args! {
                "spender" => Key::Hash(instance.package_hash()),
                "amount" => value
            },
            blocktime,
        );
        instance.deposit(owner, None, value, blocktime);
        assert_eq!(
            instance.balance_of(Address::Account(owner)),
            U256::from(10000000000_u128),
            "Invalid result"
        );
        instance.withdraw(owner, claim_rewards, value, blocktime);
        assert_eq!(
            instance.balance_of(Address::Account(owner)),
            U256::from(0_u128),
            "Invalid result"
        );
    }

    #[test]
    fn test_kill_me() {
        let (env, owner, instance, _, blocktime) = deploy();
        instance.kill_me(owner, blocktime);
        TestContract::new(
            &env,
            TEST_SESSION_CODE_WASM,
            TEST_SESSION_CODE_NAME,
            owner,
            runtime_args! {
                "entrypoint" => String::from(IS_KILLED),
                "package_hash" => Key::Hash(instance.package_hash()),
            },
            blocktime,
        );
        let ret: bool = env.query_account_named_key(owner, &[IS_KILLED.into()]);
        assert!(ret, "{} {}", true, "Invalid result");
    }
}
mod ownership_and_toggle_external_rewards_claim_test_cases {
    use crate::liquidity_gauge_reward_tests::*;
    #[test]
    fn test_commit_transfer_ownership() {
        let (env, owner, instance, _, blocktime) = deploy();
        let addr: Key = env.next_user().into();
        instance.commit_transfer_ownership(owner, addr, blocktime);
        TestContract::new(
            &env,
            TEST_SESSION_CODE_WASM,
            TEST_SESSION_CODE_NAME,
            owner,
            runtime_args! {
                "entrypoint" => String::from(FUTURE_ADMIN),
                "package_hash" => Key::Hash(instance.package_hash()),
            },
            blocktime,
        );
        let ret: Key = env.query_account_named_key(owner, &[FUTURE_ADMIN.into()]);
        assert_eq!(ret, addr, "Invalid result");
    }

    #[test]
    fn test_apply_transfer_ownership() {
        let (env, owner, instance, _, blocktime) = deploy();
        let addr: Key = env.next_user().into();
        instance.commit_transfer_ownership(owner, addr, blocktime);
        TestContract::new(
            &env,
            TEST_SESSION_CODE_WASM,
            TEST_SESSION_CODE_NAME,
            owner,
            runtime_args! {
                "entrypoint" => String::from(FUTURE_ADMIN),
                "package_hash" => Key::Hash(instance.package_hash()),
            },
            blocktime,
        );
        let ret: Key = env.query_account_named_key(owner, &[FUTURE_ADMIN.into()]);
        assert_eq!(ret, addr, "Invalid result");
        instance.apply_transfer_ownership(owner, blocktime);
        TestContract::new(
            &env,
            TEST_SESSION_CODE_WASM,
            TEST_SESSION_CODE_NAME,
            owner,
            runtime_args! {
                "entrypoint" => String::from(ADMIN),
                "package_hash" => Key::Hash(instance.package_hash()),
            },
            blocktime,
        );
        let ret: Key = env.query_account_named_key(owner, &[ADMIN.into()]);
        assert_eq!(ret, addr, "Invalid result");
    }

    #[test]
    fn test_toggle_external_rewards_claim() {
        let (_env, owner, instance, _, blocktime) = deploy();
        let val: bool = true;
        instance.toggle_external_rewards_claim(owner, val, blocktime);
        let ret: bool = instance.key_value("is_claiming_rewards".to_string());
        assert!(ret, "Invalid result");
    }
}
mod panic_test_cases {
    use crate::liquidity_gauge_reward_tests::*;
    #[should_panic]
    #[test]
    fn test_apply_transfer_ownership_panic() {
        let (env, owner, instance, _, blocktime) = deploy();
        let addr: Key = env.next_user().into();
        instance.apply_transfer_ownership(owner, blocktime);
        TestContract::new(
            &env,
            TEST_SESSION_CODE_WASM,
            TEST_SESSION_CODE_NAME,
            owner,
            runtime_args! {
                "entrypoint" => String::from(ADMIN),
                "package_hash" => Key::Hash(instance.package_hash()),
            },
            blocktime,
        );
        let ret: Key = env.query_account_named_key(owner, &[ADMIN.into()]);
        assert_eq!(ret, addr, "Invalid result");
    }
    #[should_panic]
    #[test]
    fn test_withdraw_panic() {
        let (env, owner, instance, _, blocktime) = deploy();
        let value: U256 = U256::from(10000000000_u128);
        instance.withdraw(owner, true, value, blocktime);
        TestContract::new(
            &env,
            TEST_SESSION_CODE_WASM,
            TEST_SESSION_CODE_NAME,
            owner,
            runtime_args! {
                "entrypoint" => String::from(BALANCE_OF),
                "package_hash" => Key::Hash(instance.package_hash()),
                "address" => Key::Account(owner)
            },
            blocktime,
        );
    }
}
