use crate::liquidity_gauge_v4_instance::{address_to_str, LIQUIDITYGUAGEV4INSTANCEInstance};
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U128, U256};
use casperlabs_test_env::{now, TestContract, TestEnv};
use common::{keys::BALANCES, utils::i128_to_tuple};
use crv20::Address;

pub const TEN_E_NINE: u128 = 1000000000;

// Mock LP Token
fn deploy_mock_lp(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "curve-erc20.wasm",
        "Lp Token",
        owner,
        runtime_args! {
            "name" => "LP Token",
            "symbol" => "LP",
            "decimals" => 9_u8,
            "initial_supply" => U256::from(TEN_E_NINE * 100000000000000000000)
        },
        now(),
    )
}
// CRV
fn deploy_erc20_crv(env: &TestEnv, sender: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "erc20-crv.wasm",
        "erc20-crv",
        sender,
        runtime_args! {
            "name" => "CRV",
            "symbol" => "ERC20CRV",
            "decimals" => 9_u8
        },
        now(),
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
        now(),
    )
}
// Gauge controller
fn deploy_gauge_controller(
    env: &TestEnv,
    sender: AccountHash,
    token: Key,
    voting_escrow: Key,
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
        now(),
    )
}
// Minter
fn deploy_minter(env: &TestEnv, sender: AccountHash, controller: Key, token: Key) -> TestContract {
    TestContract::new(
        env,
        "minter-token.wasm",
        "minter",
        sender,
        runtime_args! {
            "controller" => controller,
            "token" => token,
        },
        now(),
    )
}

fn i128_to_f64_checked(value: i128) -> f64 {
    let ret = value as f64;
    // float last digit rounding fix
    if value != ret as i128 && value + 1 != ret as i128 && value - 1 != ret as i128 {
        assert!(false, "Converstion failed");
    }
    ret
}

fn u256_to_f64_checked(value: U256) -> f64 {
    let ret = value.as_u128() as f64;
    // float last digit rounding fix
    if value != U256::from(ret as u128)
        && value + 1 != U256::from(ret as u128)
        && value - 1 != U256::from(ret as u128)
    {
        assert!(false, "Converstion failed");
    }
    ret
}

fn approx(a: f64, b: f64, precision: f64) -> bool {
    if a == 0.0 && b == 0.0 {
        return true;
    }
    return 2.0 * (a - b).abs() / (a + b) <= precision;
}

#[test]
fn integration_test_for_minted_tokens_checking() {
    let mut time = now();
    let amount: U256 = TEN_E_NINE.into();

    let env = TestEnv::new();
    let (admin, bob, charlie, dan) = (
        env.next_user(),
        env.next_user(),
        env.next_user(),
        env.next_user(),
    );

    let token = deploy_erc20_crv(&env, admin);
    let voting_escrow = deploy_voting_escrow(
        &env,
        admin,
        Key::Hash(token.package_hash()),
        "Voting Escrow".into(),
        "VT".into(),
        "1".into(),
    );
    let gauge_controller = deploy_gauge_controller(
        &env,
        admin,
        Key::Hash(token.package_hash()),
        Key::Hash(voting_escrow.package_hash()),
    );
    let minter = deploy_minter(
        &env,
        admin,
        Key::Hash(gauge_controller.package_hash()),
        Key::Hash(token.package_hash()),
    );

    let mock_lp_token = deploy_mock_lp(&env, admin);

    let three_gauges = [
        LIQUIDITYGUAGEV4INSTANCEInstance::new_deploy(
            &env,
            "LG-1",
            admin,
            Key::Hash(mock_lp_token.package_hash()),
            Key::Hash(minter.package_hash()),
            Key::Account(admin),
        ),
        LIQUIDITYGUAGEV4INSTANCEInstance::new_deploy(
            &env,
            "LG-2",
            admin,
            Key::Hash(mock_lp_token.package_hash()),
            Key::Hash(minter.package_hash()),
            Key::Account(admin),
        ),
        LIQUIDITYGUAGEV4INSTANCEInstance::new_deploy(
            &env,
            "LG-3",
            admin,
            Key::Hash(mock_lp_token.package_hash()),
            Key::Hash(minter.package_hash()),
            Key::Account(admin),
        ),
    ];

    token.call_contract(
        admin,
        "set_minter",
        runtime_args! {"minter"=>Key::Hash(minter.package_hash())},
        time,
    );

    let type_weights = [500_000_000i128, 2_000_000_000i128];
    let gauge_weights = [2_000_000_000i128, 1_000_000_000i128, 500_000_000i128];
    let gauge_types = [0i128, 0i128, 1i128];

    // Set up types
    for (i, w) in type_weights.iter().enumerate() {
        gauge_controller.call_contract(
            admin,
            "add_type",
            runtime_args! {"name" => "Liquidity", "weight" => None::<U256> },
            time,
        );
        gauge_controller.call_contract(
            admin,
            "change_type_weight",
            runtime_args! {"type_id" => (false, U128::from(i)), "weight" => U256::from(*w) },
            time,
        );
    }

    // Set up gauges
    for (g, (t, w)) in three_gauges
        .iter()
        .zip(gauge_types.iter().zip(gauge_weights.iter()))
    {
        gauge_controller.call_contract(
            admin,
            "add_gauge",
            runtime_args! {
                "addr" => Key::Hash(g.package_hash()),
                "gauge_type" => i128_to_tuple(*t),
                "weight" => Some(U256::from(*w))
            },
            time,
        );
    }

    // Transfer tokens to Bob, Charlie and Dan
    for user in [bob, charlie, dan].iter() {
        mock_lp_token.call_contract(
            admin,
            "transfer",
            runtime_args! {"recipient" => Address::Account(*user),"amount" => amount},
            time,
        );
    }

    // For weights to activate
    time += 7 * 86400000;

    // Bob and Charlie deposit to gauges with different weights
    mock_lp_token.call_contract(
        bob,
        "approve",
        runtime_args! {"spender" => Address::Contract(three_gauges[1].package_hash().into()), "amount" => amount},
        time,
    );
    three_gauges[1].call_contract(
        bob,
        "deposit",
        runtime_args! {
            "value" => amount,
            "addr" => None::<Key>,
            "claim_rewards" => None::<bool>,
        },
        time,
    );
    mock_lp_token.call_contract(
        charlie,
        "approve",
        runtime_args! {"spender" => Address::Contract(three_gauges[2].package_hash().into()), "amount" => amount},
        time,
    );
    three_gauges[2].call_contract(
        charlie,
        "deposit",
        runtime_args! {
            "value" => amount,
            "addr" => None::<Key>,
            "claim_rewards" => None::<bool>,
        },
        time,
    );

    let dt = 30 * 86400000;
    time += dt;

    mock_lp_token.call_contract(
        dan,
        "approve",
        runtime_args! {"spender" => Address::Contract(three_gauges[1].package_hash().into()), "amount" => amount},
        time,
    );
    three_gauges[1].call_contract(
        dan,
        "deposit",
        runtime_args! {
            "value" => amount,
            "addr" => None::<Key>,
            "claim_rewards" => None::<bool>,
        },
        time,
    );

    time += dt;

    // Commented as it should fial
    // // Cannot withdraw too much
    // three_gauges[1].call_contract(
    //     bob,
    //     "withdraw",
    //     runtime_args! {
    //         "value" => amount + 1,
    //         "claim_rewards" => None::<bool>,
    //     },
    //     time,
    // );

    // Withdraw
    three_gauges[1].call_contract(
        bob,
        "withdraw",
        runtime_args! {
            "value" => amount,
            "claim_rewards" => None::<bool>,
        },
        time,
    );
    three_gauges[2].call_contract(
        charlie,
        "withdraw",
        runtime_args! {
            "value" => amount,
            "claim_rewards" => None::<bool>,
        },
        time,
    );
    three_gauges[1].call_contract(
        dan,
        "withdraw",
        runtime_args! {
            "value" => amount,
            "claim_rewards" => None::<bool>,
        },
        time,
    );

    for user in [bob, charlie, dan].iter() {
        assert_eq!(
            mock_lp_token.query::<U256>(BALANCES, address_to_str(&Address::Account(*user))),
            amount
        );
    }

    // Claim for Bob now
    minter.call_contract(
        bob,
        "mint",
        runtime_args! {"gauge_addr"=>Key::Hash(three_gauges[1].package_hash())},
        time,
    );
    let bob_tokens = token.query::<U256>(BALANCES, address_to_str(&Address::Account(bob)));

    time += dt;

    // This won't give anything
    minter.call_contract(
        bob,
        "mint",
        runtime_args! {"gauge_addr"=>Key::Hash(three_gauges[1].package_hash())},
        time,
    );
    assert_eq!(
        bob_tokens,
        token.query::<U256>(BALANCES, address_to_str(&Address::Account(bob))),
    );

    minter.call_contract(
        charlie,
        "mint",
        runtime_args! {"gauge_addr"=>Key::Hash(three_gauges[2].package_hash())},
        time,
    );
    let charlie_tokens = token.query::<U256>(BALANCES, address_to_str(&Address::Account(charlie)));
    minter.call_contract(
        dan,
        "mint",
        runtime_args! {"gauge_addr"=>Key::Hash(three_gauges[1].package_hash())},
        time,
    );
    let dan_tokens = token.query::<U256>(BALANCES, address_to_str(&Address::Account(dan)));

    let s = bob_tokens + charlie_tokens + dan_tokens;
    let ww: Vec<_> = gauge_weights
        .iter()
        .zip(gauge_types.iter())
        .map(|(w, t)| w * type_weights[*t as usize])
        .collect();
    let sw = ww[1] + ww[2]; // Gauge 0 not used

    // Bob and Charlie were there for full time, gauges 1 and 2
    // Dan was in gauge 1 for half the time
    assert!(approx(
        u256_to_f64_checked(bob_tokens) / u256_to_f64_checked(s),
        0.75 * i128_to_f64_checked(ww[1]) / i128_to_f64_checked(sw), // 0.75 == 3/4
        2e-2
    ));
    assert!(approx(
        u256_to_f64_checked(charlie_tokens) / u256_to_f64_checked(s),
        i128_to_f64_checked(ww[2]) / i128_to_f64_checked(sw) as f64,
        2e-2
    ));
    assert!(approx(
        u256_to_f64_checked(dan_tokens) / u256_to_f64_checked(s),
        0.25 * i128_to_f64_checked(ww[1]) / i128_to_f64_checked(sw), // 0.25 == 1/4
        2e-2
    ));
}
