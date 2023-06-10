use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U128, U256};
use casperlabs_test_env::{TestContract, TestEnv};

use crate::minter_instance::{add_gauge, MINTERInstance};

const NAME: &str = "MINTER";
const TOKEN_NAME: &str = "ERC20";
const TOKEN_SYMBOL: &str = "ERC";
const DECIMALS: u8 = 8;
const INIT_TOTAL_SUPPLY: u64 = 0;

fn deploy_erc20(env: &TestEnv, sender: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "curve-erc20.wasm",
        "erc20",
        sender,
        runtime_args! {
            "initial_supply" => U256::from(0),
            "name" => "Token",
            "symbol" => "ERC20",
            "decimals" => 9_u8
        },
        0,
    )
}

fn deploy() -> (
    TestEnv,
    MINTERInstance,
    AccountHash,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    u64,
) {
    let block_time = MINTERInstance::now();
    let env = TestEnv::new();
    let owner = env.next_user();
    let _token: TestContract = MINTERInstance::deploy_erc20(
        &env,
        owner,
        TOKEN_NAME,
        TOKEN_SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
        block_time,
    );
    let erc20_crv = MINTERInstance::deploy_erc20_crv(&env, owner, block_time);

    let voting_escrow = MINTERInstance::deploy_voting_escrow(
        &env,
        "Voting Escrow",
        owner,
        Key::Hash(erc20_crv.package_hash()),
        "VotingEscrow".into(),
        "VE".into(),
        "1".into(),
        block_time,
    );

    let gauge_controller: TestContract = MINTERInstance::deploy_gauge_controller(
        &env,
        "gauge_controller",
        owner,
        Key::Hash(erc20_crv.package_hash()),
        Key::Hash(voting_escrow.package_hash()),
        block_time,
    );

    let minter: TestContract = MINTERInstance::new_deploy(
        &env,
        NAME,
        owner,
        Key::Hash(erc20_crv.package_hash()),
        Key::Hash(gauge_controller.package_hash()),
        block_time,
    );
    let liquidity_gauge = MINTERInstance::deploy_liquidity_gauge(
        &env,
        "Liquidity Gauge",
        owner,
        Key::Hash(_token.package_hash()),
        Key::Hash(minter.package_hash()),
        Key::Account(owner),
        block_time,
    );
    erc20_crv.call_contract(
        owner,
        "set_minter",
        runtime_args! {"minter"=>Key::Hash(minter.package_hash())},
        block_time,
    );
    (
        env,
        MINTERInstance::instance(minter),
        owner,
        _token,
        voting_escrow,
        gauge_controller,
        liquidity_gauge,
        erc20_crv,
        block_time,
    )
}

#[test]
fn test_deploy() {
    let (
        env,
        minter,
        _owner,
        _token,
        _voting_escrow,
        gauge_controller,
        _liquidity_gauge,
        erc20_crv,
        _,
    ) = deploy();
    let _user = env.next_user();
    assert_eq!(minter.token(), Key::Hash(erc20_crv.package_hash()));
    assert_eq!(
        minter.controller(),
        Key::Hash(gauge_controller.package_hash())
    );
}

#[test]
fn test_minter_mint() {
    let (
        env,
        minter,
        owner,
        token,
        _voting_escrow,
        gauge_controller,
        liquidity_gauge,
        _erc20_crv,
        block_time,
    ) = deploy();
    let _user = env.next_user();
    minter.toggle_approve_mint(owner, Key::from(owner), block_time);
    let name: String = "type".to_string();
    gauge_controller.call_contract(
        owner,
        "add_type",
        runtime_args! {
            "name" => name,
            "weight" => Some(U256::from(1))
        },
        block_time,
    );
    let gauge_type: (bool, U128) = (false, 0.into());
    let weight = U256::from(1000000);
    add_gauge(
        &gauge_controller,
        owner,
        Key::Hash(liquidity_gauge.package_hash()),
        gauge_type,
        Some(weight),
        block_time,
    );
    let liquidity_gauge_1 = MINTERInstance::deploy_liquidity_gauge(
        &env,
        "Liquidity Gauge 1",
        owner,
        Key::Hash(token.package_hash()),
        Key::from(minter.contract_package_hash()),
        Key::Account(owner),
        block_time,
    );
    let name: String = "type1".to_string();
    gauge_controller.call_contract(
        owner,
        "add_type",
        runtime_args! {
            "name" => name,
            "weight" => Some(U256::from(10))
        },
        0,
    );
    let gauge_type: (bool, U128) = (false, 1.into());
    add_gauge(
        &gauge_controller,
        owner,
        Key::Hash(liquidity_gauge_1.package_hash()),
        gauge_type,
        Some(weight),
        block_time,
    );
    minter.mint(
        owner,
        Key::Hash(liquidity_gauge_1.package_hash()),
        block_time,
    );
}

#[test]
fn test_minter_mint_many() {
    let (
        env,
        minter,
        owner,
        token,
        _voting_escrow,
        gauge_controller,
        liquidity_gauge,
        _erc20_crv,
        block_time,
    ) = deploy();

    let name: String = "type".to_string();
    gauge_controller.call_contract(
        owner,
        "add_type",
        runtime_args! {
            "name" => name,
            "weight" => Some(U256::from(100))
        },
        block_time,
    );
    let gauge_type: (bool, U128) = (false, 0.into());
    let weight = U256::from(1000000);
    add_gauge(
        &gauge_controller,
        owner,
        Key::Hash(liquidity_gauge.package_hash()),
        gauge_type,
        Some(weight),
        block_time,
    );
    let liquidity_gauge_1 = MINTERInstance::deploy_liquidity_gauge(
        &env,
        "Liquidity Gauge 1",
        owner,
        Key::Hash(token.package_hash()),
        Key::from(minter.contract_package_hash()),
        Key::Account(owner),
        block_time,
    );
    let name: String = "type1".to_string();
    gauge_controller.call_contract(
        owner,
        "add_type",
        runtime_args! {
            "name" => name,
            "weight" => Some(U256::from(1000))
        },
        block_time,
    );
    let gauge_type: (bool, U128) = (false, 1.into());
    add_gauge(
        &gauge_controller,
        owner,
        Key::Hash(liquidity_gauge_1.package_hash()),
        gauge_type,
        Some(weight),
        block_time,
    );
    let gauge_addrs: Vec<String> =
        vec![Key::Hash(liquidity_gauge_1.package_hash()).to_formatted_string()];
    minter.mint_many(owner, gauge_addrs, block_time);
}

#[test]
fn test_minter_mint_for() {
    let (
        env,
        minter,
        owner,
        token,
        _voting_escrow,
        gauge_controller,
        liquidity_gauge,
        _erc20_crv,
        block_time,
    ) = deploy();
    minter.toggle_approve_mint(owner, Key::from(owner), block_time);
    let name: String = "type".to_string();
    gauge_controller.call_contract(
        owner,
        "add_type",
        runtime_args! {
            "name" => name,
            "weight" => Some(U256::from(2))
        },
        0,
    );
    let gauge_type: (bool, U128) = (false, 0.into());
    let weight = U256::from(1000000);
    add_gauge(
        &gauge_controller,
        owner,
        Key::Hash(liquidity_gauge.package_hash()),
        gauge_type,
        Some(weight),
        block_time,
    );
    let liquidity_gauge_1 = MINTERInstance::deploy_liquidity_gauge(
        &env,
        "Liquidity Gauge 1",
        owner,
        Key::Hash(token.package_hash()),
        Key::from(minter.contract_package_hash()),
        Key::Account(owner),
        block_time,
    );
    let name: String = "type1".to_string();
    gauge_controller.call_contract(
        owner,
        "add_type",
        runtime_args! {
            "name" => name,
            "weight" => Some(U256::from(20))
        },
        block_time,
    );
    let gauge_type: (bool, U128) = (false, 1.into());
    add_gauge(
        &gauge_controller,
        owner,
        Key::Hash(liquidity_gauge_1.package_hash()),
        gauge_type,
        Some(weight),
        block_time,
    );

    minter.mint_for(
        owner,
        Key::Hash(liquidity_gauge_1.package_hash()),
        Key::from(owner),
        block_time,
    );
}

#[test]
fn test_minter_toggle_approve_mint() {
    let (
        _env,
        minter,
        owner,
        token,
        _voting_escrow,
        _gauge_controller,
        _liquidity_gauge_reward,
        _erc20_crv,
        block_time,
    ) = deploy();

    minter.toggle_approve_mint(owner, Key::Hash(token.package_hash()), block_time);
}
#[test]
fn test_minter_mint_with_deposit() {
    let (
        env,
        minter,
        owner,
        _token,
        _voting_escrow,
        gauge_controller,
        liquidity_gauge,
        _erc20_crv,
        block_time,
    ) = deploy();
    let _user = env.next_user();
    minter.toggle_approve_mint(owner, Key::from(owner), block_time);
    let name: String = "type".to_string();
    gauge_controller.call_contract(
        owner,
        "add_type",
        runtime_args! {
            "name" => name,
            "weight" => Some(U256::from(200))
        },
        block_time,
    );
    let gauge_type: (bool, U128) = (false, 0.into());
    let weight = U256::from(1000000);
    add_gauge(
        &gauge_controller,
        owner,
        Key::Hash(liquidity_gauge.package_hash()),
        gauge_type,
        Some(weight),
        block_time,
    );
    let lp_token = deploy_erc20(&env, owner);
    let value: U256 = 100000000.into();
    let liquidity_gauge_1 = MINTERInstance::deploy_liquidity_gauge(
        &env,
        "Liquidity Gauge 1",
        owner,
        Key::Hash(lp_token.package_hash()),
        Key::from(minter.contract_package_hash()),
        Key::Account(owner),
        block_time,
    );
    let name: String = "type1".to_string();
    gauge_controller.call_contract(
        owner,
        "add_type",
        runtime_args! {
            "name" => name,
            "weight" => Some(U256::from(2000))
        },
        0,
    );
    let gauge_type: (bool, U128) = (false, 1.into());
    add_gauge(
        &gauge_controller,
        owner,
        Key::Hash(liquidity_gauge_1.package_hash()),
        gauge_type,
        Some(weight),
        block_time,
    );
    let mint_value: U256 = (10000000000_u128).into();
    lp_token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => mint_value
        },
        block_time,
    );
    let approve_value: U256 = (1000000000_u128).into();
    lp_token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(liquidity_gauge_1.package_hash()),
            "amount" => approve_value
        },
        block_time,
    );
    liquidity_gauge_1.call_contract(
        owner,
        "deposit",
        runtime_args! {
            "value" => value,
            "addr" =>  None::<Key>,
            "claim_rewards" => None::<bool>
        },
        block_time,
    );
    minter.mint(
        owner,
        Key::Hash(liquidity_gauge_1.package_hash()),
        block_time,
    );
}
