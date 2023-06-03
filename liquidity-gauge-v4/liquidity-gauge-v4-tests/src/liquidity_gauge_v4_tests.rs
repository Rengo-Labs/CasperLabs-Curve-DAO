use crate::liquidity_gauge_v4_instance::LIQUIDITYGUAGEV4INSTANCEInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U128, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;
use crv20::Address;

pub const TEN_E_NINE: u128 = 1000000000;
const NAME: &str = "LiquidityGaugeV4";
//ERC20
fn deploy_erc20(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "curve-erc20.wasm",
        "rewarded_token",
        owner,
        runtime_args! {
            "name" => "LP token",
            "symbol" => "LPtok",
            "decimals" => 9_u8,
            "initial_supply" => U256::from(TEN_E_NINE * 100000000000000000000)
        },
        0,
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
        LIQUIDITYGUAGEV4INSTANCEInstance::now(),
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
        LIQUIDITYGUAGEV4INSTANCEInstance::now(),
    )
}
//gauge_controller
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
        LIQUIDITYGUAGEV4INSTANCEInstance::now(),
    )
}

//Minter
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
        LIQUIDITYGUAGEV4INSTANCEInstance::now(),
    )
}
// Liquidity Guage V4

fn deploy() -> (TestEnv, AccountHash, TestContract, u64) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let time_now: u64 = LIQUIDITYGUAGEV4INSTANCEInstance::now();
    let erc20 = deploy_erc20(&env, owner);
    let erc20_crv = deploy_erc20_crv(&env, owner);
    let voting_escrow = deploy_voting_escrow(
        &env,
        owner,
        Key::Hash(erc20_crv.package_hash()),
        "Voting Escrow".into(),
        "VT".into(),
        "1".into(),
    );
    let gauge_controller = deploy_gauge_controller(
        &env,
        owner,
        Key::Hash(erc20_crv.package_hash()),
        Key::Hash(voting_escrow.package_hash()),
    );
    let minter = deploy_minter(
        &env,
        owner,
        Key::Hash(gauge_controller.package_hash()),
        Key::Hash(erc20_crv.package_hash()),
    );

    let liquidity_gauge_v4_instance = LIQUIDITYGUAGEV4INSTANCEInstance::new_deploy(
        &env,
        NAME,
        owner,
        Key::Hash(erc20.package_hash()),
        Key::Hash(minter.package_hash()),
        Key::Account(owner),
    );
    // For Minting Purpose
    let to = Key::Hash(liquidity_gauge_v4_instance.package_hash());
    let amount: U256 = U256::from(TEN_E_NINE * 100000000000000000000);
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount},
        time_now,
    );
    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {"spender" =>Address::Contract(liquidity_gauge_v4_instance.package_hash().into()) , "amount" => amount},
        time_now,
    );
    let _name: String = "type".to_string();
    gauge_controller.call_contract(
        owner,
        "add_type",
        runtime_args! {"name" => _name, "weight" => None::<U256> },
        time_now,
    );
    let addr: Key = Key::Account(owner);
    let gauge_type: (bool, U128) = (false, 0.into());
    gauge_controller.call_contract(
        owner,
        "add_gauge",
        runtime_args! {
            "addr" => addr,
            "gauge_type" => gauge_type,
            "weight"=>None::<U256>
        },
        time_now,
    );
    let _name_1: String = "type1".to_string();
    gauge_controller.call_contract(
        owner,
        "add_type",
        runtime_args! {"name" => _name_1, "weight" => None::<U256> },
        time_now,
    );
    let addr1: Key = Key::Hash(liquidity_gauge_v4_instance.package_hash());
    let gauge_type_1: (bool, U128) = (false, 1.into());
    gauge_controller.call_contract(
        owner,
        "add_gauge",
        runtime_args! {
            "addr" => addr1,
            "gauge_type" => gauge_type_1,
            "weight"=>None::<U256>
        },
        time_now,
    );
    (env, owner, liquidity_gauge_v4_instance, time_now)
}
mod t10 {
    use crate::liquidity_gauge_v4_tests::*;
    #[test]
    fn test_claimable_tokens() {
        let (env, owner, contract, time_now) = deploy();
        let contract = LIQUIDITYGUAGEV4INSTANCEInstance::instance(contract);
        let addr = env.next_user();
        TestContract::new(
            &env,
            "liquidity_gauge_v4_session_code.wasm",
            "SessionCode",
            owner,
            runtime_args! {
                "entrypoint" => String::from(CLAIMABLE_TOKENS),
                "package_hash" => Key::Hash(contract.package_hash()),
                "addr"=>Key::from(addr)
            },
            time_now,
        );
        let ret: U256 = env.query_account_named_key(owner, &[CLAIMABLE_TOKENS.into()]);
        assert_eq!(ret, 0.into());
    }
}