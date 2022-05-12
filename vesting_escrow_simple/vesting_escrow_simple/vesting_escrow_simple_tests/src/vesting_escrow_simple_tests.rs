use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use test_env::{TestContract, TestEnv};

use crate::vesting_escrow_simple_instance::VESTINGESCROWSIMPLEInstance;
pub const TEN_E_NINE: u128 = 1000000000;
fn deploy_erc20(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "erc20-token.wasm",
        "erc2020",
        owner,
        runtime_args! {
            "name" => "ERC",
            "symbol" => "ERC20",
            "decimals" => 9 as u8,
            "initial_supply" => U256::from(TEN_E_NINE*1000)
        },
    )
}
fn deploy() -> (TestEnv, AccountHash, TestContract, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let erc20 = deploy_erc20(&env, owner);

    let contract = VESTINGESCROWSIMPLEInstance::new(
        &env,
        "VESTINGESCROWSIMPLE",
        owner,
        Key::Hash(erc20.package_hash()),
    );
    let proxy = VESTINGESCROWSIMPLEInstance::proxy(
        &env,
        "VESTINGESCROWSIMPLEPROXY",
        owner,
        Key::Hash(contract.contract_hash()),
    );
    let key: ContractPackageHash = contract.query_named_key("self_package_hash".to_string());
    let to: Key = Key::from(key);
    let amount: U256 = U256::from(TEN_E_NINE * 100);

    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount},
    );
    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {"spender" => to , "amount" => amount},
    );

    (env, owner, contract, proxy)
}

#[test]
fn test_deploy() {
    let (_, _, _, _) = deploy();
}
// #[test]
// fn initialize() {
//     let (env, owner, contract, proxy) = deploy();
//     let erc20 = deploy_erc20(&env, owner);
//     let token = Key::Hash(erc20.package_hash());
//     let key: ContractPackageHash = contract.query_named_key("self_package_hash".to_string());
//     let to: Key = Key::from(key);
//     let amount: U256 = U256::from(TEN_E_NINE * 100);
//     erc20.call_contract(
//         owner,
//         "mint",
//         runtime_args! {"to" => to , "amount" => amount},
//     );
//     erc20.call_contract(
//         owner,
//         "approve",
//         runtime_args! {"spender" => to , "amount" => amount},
//     );
//     let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
//     let admin_arg: Key = Key::from_formatted_str(
//         "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
//     )
//     .unwrap();
//     let recipient: Key = Key::Account(owner);
//     let start_time: U256 = 10.into();
//     let end_time: U256 = 20.into();
//     let can_disable: bool = false;

//     contract.initialize(
//         owner,
//         admin_arg,
//         token,
//         recipient,
//         amount,
//         start_time,
//         end_time,
//         can_disable,
//     );
// }
#[test]
fn toggle_disable() {
    let (env, owner, contract, proxy) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
    let _recipient_arg: Key = Key::Account(owner);
    contract.toggle_disable(owner, _recipient_arg);
}
#[test]
fn disable_can_disable() {
    let (env, owner, contract, proxy) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);

    contract.disable_can_disable(owner);
}
#[test]
fn vested_of() {
    let (env, owner, contract, proxy) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
    let proxy = VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
    let recipient_arg: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();
    proxy.vested_of(owner, recipient_arg);
    let res: U256 = proxy.result();
    assert_eq!(res, 100.into());
}
#[test]
fn vested_supply() {
    let (env, owner, contract, proxy) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
    let proxy = VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
    proxy.vested_supply(owner);
    let res: U256 = proxy.result();
    assert_eq!(res, 0.into());
}
#[test]
fn locked_supply() {
    let (env, owner, contract, proxy) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
    let proxy = VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
    proxy.locked_supply(owner);
    let res: U256 = proxy.result();
    //println!("{:}",res);
    assert_eq!(res, 100.into());
}
#[test]
fn balance_of_vest() {
    let (env, owner, contract, proxy) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
    let proxy = VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
    let recipient_arg: Key = Key::Account(owner);
    proxy.balance_of_vest(owner, recipient_arg);
    let res: U256 = proxy.result();
    assert_eq!(res, 0.into());
}
#[test]
fn commit_transfer_ownership() {
    let (env, owner, contract, proxy) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
    let proxy = VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
    let addr: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();
    proxy.commit_transfer_ownership(owner, addr);
    let res: bool = proxy.result();
    assert_eq!(res, true);
}
#[test]
fn apply_transfer_ownership() {
    let (env, owner, contract, proxy) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
    let proxy = VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
    proxy.apply_transfer_ownership(owner);
    let res: bool = proxy.result();
    assert_eq!(res, true);
}
#[test]
fn claim() {
    let (env, owner, contract, proxy) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
    let addr_arg: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();

    contract.claim(owner, addr_arg);
}
