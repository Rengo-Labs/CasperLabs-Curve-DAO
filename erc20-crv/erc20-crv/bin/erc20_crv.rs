#![no_main]
#![no_std]
extern crate alloc;

use alloc::{collections::BTreeSet, format, string::String, vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use casperlabs_contract_utils::{ContractContext, OnChainContractStorage};
use crv20::{self, Address, CURVEERC20};
use erc20_crv::{self, data, ERC20CRV};

#[derive(Default)]
struct Erc20Crv(OnChainContractStorage);
impl ContractContext<OnChainContractStorage> for Erc20Crv {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}
impl CURVEERC20<OnChainContractStorage> for Erc20Crv {}
impl ERC20CRV<OnChainContractStorage> for Erc20Crv {}

impl Erc20Crv {
    fn constructor(&mut self, contract_hash: ContractHash, package_hash: ContractPackageHash) {
        ERC20CRV::init(self, contract_hash, package_hash);
    }
}

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    Erc20Crv::default().constructor(contract_hash, package_hash);
}
#[no_mangle]
fn set_minter() {
    let minter: Key = runtime::get_named_arg("minter");
    Erc20Crv::default().set_minter(minter);
}
#[no_mangle]
fn burn() {
    let value: U256 = runtime::get_named_arg("value");
    ERC20CRV::burn(&Erc20Crv::default(), value).unwrap_or_revert();
}
#[no_mangle]
fn set_admin() {
    let admin: Key = runtime::get_named_arg("admin");
    Erc20Crv::default().set_admin(admin);
}
#[no_mangle]
fn remove_admin() {
    let admin: Key = runtime::get_named_arg("admin");
    Erc20Crv::default().remove_admin(admin);
}
#[no_mangle]
fn start_epoch_time_write() {
    let ret = Erc20Crv::default().start_epoch_time_write();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn future_epoch_time_write() {
    let ret = Erc20Crv::default().future_epoch_time_write();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn available_supply() {
    let ret = Erc20Crv::default().available_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn mintable_in_timeframe() {
    let start: U256 = runtime::get_named_arg("start");
    let end: U256 = runtime::get_named_arg("end");
    let ret = Erc20Crv::default().mintable_in_timeframe(start, end);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn update_mining_parameters() {
    Erc20Crv::default().update_mining_parameters();
}
#[no_mangle]
fn mint() {
    let to: Address = runtime::get_named_arg("to");
    let amount: U256 = runtime::get_named_arg("amount");
    ERC20CRV::mint(&Erc20Crv::default(), to, amount).unwrap_or_revert();
}
#[no_mangle]
fn transfer_from() {
    let owner: Address = runtime::get_named_arg("owner");
    let recipient: Address = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    CURVEERC20::transfer_from(&Erc20Crv::default(), owner, recipient, amount).unwrap_or_revert();
}
#[no_mangle]
fn approve() {
    let spender: Address = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    CURVEERC20::approve(&Erc20Crv::default(), spender, amount).unwrap_or_revert();
}
#[no_mangle]
fn transfer() {
    let recipient: Address = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    CURVEERC20::transfer(&Erc20Crv::default(), recipient, amount).unwrap_or_revert();
}
#[no_mangle]
fn total_supply() {
    let ret: U256 = CURVEERC20::total_supply(&Erc20Crv::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn increase_allowance() {
    let spender: Address = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    CURVEERC20::increase_allowance(&Erc20Crv::default(), spender, amount).unwrap_or_revert();
}
#[no_mangle]
fn decrease_allowance() {
    let spender: Address = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    CURVEERC20::decrease_allowance(&Erc20Crv::default(), spender, amount).unwrap_or_revert();
}
#[no_mangle]
fn allowance() {
    let owner: Address = runtime::get_named_arg("owner");
    let spender: Address = runtime::get_named_arg("spender");
    let ret: U256 = CURVEERC20::allowance(&Erc20Crv::default(), owner, spender);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

//[no_mangle] of public variables
#[no_mangle]
fn name() {
    runtime::ret(CLValue::from_t(CURVEERC20::name(&Erc20Crv::default())).unwrap_or_revert());
}
#[no_mangle]
fn symbol() {
    runtime::ret(CLValue::from_t(CURVEERC20::symbol(&Erc20Crv::default())).unwrap_or_revert());
}
#[no_mangle]
fn decimals() {
    runtime::ret(CLValue::from_t(CURVEERC20::decimals(&Erc20Crv::default())).unwrap_or_revert());
}
#[no_mangle]
fn balance_of() {
    let address: Address = runtime::get_named_arg("address");
    runtime::ret(
        CLValue::from_t(CURVEERC20::balance_of(&Erc20Crv::default(), address)).unwrap_or_revert(),
    );
}
#[no_mangle]
fn allowances() {
    let owner: Address = runtime::get_named_arg("owner");
    let spender: Address = runtime::get_named_arg("spender");
    runtime::ret(
        CLValue::from_t(CURVEERC20::allowance(&Erc20Crv::default(), owner, spender))
            .unwrap_or_revert(),
    );
}
#[no_mangle]
fn minter() {
    runtime::ret(CLValue::from_t(data::get_minter()).unwrap_or_revert());
}
#[no_mangle]
fn rate() {
    runtime::ret(CLValue::from_t(data::get_rate()).unwrap_or_revert());
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("name", String::cl_type()),
            Parameter::new("symbol", String::cl_type()),
            Parameter::new("decimal", u8::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_minter",
        vec![Parameter::new("minter", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "burn",
        vec![Parameter::new("value", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_admin",
        vec![Parameter::new("admin", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "remove_admin",
        vec![Parameter::new("admin", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "update_mining_parameters",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "start_epoch_time_write",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "available_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "mintable_in_timeframe",
        vec![
            Parameter::new("start", U256::cl_type()),
            Parameter::new("end", U256::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "mint",
        vec![
            Parameter::new("to", Address::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer",
        vec![
            Parameter::new("recipient", Address::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("spender", Address::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("owner", Address::cl_type()),
            Parameter::new("recipient", Address::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "increase_allowance",
        vec![
            Parameter::new("spender", Address::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "decrease_allowance",
        vec![
            Parameter::new("spender", Address::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "allowance",
        vec![
            Parameter::new("owner", Address::cl_type()),
            Parameter::new("spender", Address::cl_type()),
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "allowance",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    //entry points of public variables
    entry_points.add_entry_point(EntryPoint::new(
        "name",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "symbol",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "decimals",
        vec![],
        u8::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("address", Address::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "allowances",
        vec![
            Parameter::new("owner", Address::cl_type()),
            Parameter::new("spender", Address::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "minter",
        vec![],
        Address::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "rate",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "future_epoch_time_write",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "future_epoch_time_write",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    // Contract name must be same for all new versions of the contracts
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        let name: String = runtime::get_named_arg("name");
        let symbol: String = runtime::get_named_arg("symbol");
        let decimals: u8 = runtime::get_named_arg("decimals");

        // Build new package with initial a first version of the contract.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        let (contract_hash, _) = storage::add_contract_version(
            package_hash,
            get_entry_points(),
            Erc20Crv::default()
                .named_keys_erc20crv(name, symbol, decimals)
                .unwrap_or_revert(),
        );

        let constructor_args = runtime_args! {
                "contract_hash" => contract_hash,
                "package_hash"=> package_hash
        };

        // Add the constructor group to the package hash with a single URef.
        let constructor_access: URef =
            storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
                .unwrap_or_revert()
                .pop()
                .unwrap_or_revert();

        // Call the constructor entry point
        let _: () =
            runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

        // Remove all URefs from the constructor group, so no one can call it for the second time.
        let mut urefs = BTreeSet::new();
        urefs.insert(constructor_access);
        storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
            .unwrap_or_revert();

        // Store contract in the account's named keys.
        runtime::put_key(
            &format!("{}_package_hash", contract_name),
            package_hash.into(),
        );
        runtime::put_key(
            &format!("{}_package_hash_wrapped", contract_name),
            storage::new_uref(package_hash).into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
        runtime::put_key(
            &format!("{}_package_access_token", contract_name),
            access_token.into(),
        );
    } else {
        // this is a contract upgrade
        let package_hash: ContractPackageHash =
            runtime::get_key(&format!("{}_package_hash", contract_name))
                .unwrap_or_revert()
                .into_hash()
                .unwrap()
                .into();

        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        // update contract hash
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
    }
}
