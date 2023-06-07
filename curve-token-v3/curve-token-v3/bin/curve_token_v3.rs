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
use curve_token_v3_crate::{self, data, CURVETOKENV3};

#[derive(Default)]
struct CurveTokenV3(OnChainContractStorage);
impl ContractContext<OnChainContractStorage> for CurveTokenV3 {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl CURVEERC20<OnChainContractStorage> for CurveTokenV3 {}
impl CURVETOKENV3<OnChainContractStorage> for CurveTokenV3 {}

impl CurveTokenV3 {
    fn constructor(&mut self, contract_hash: ContractHash, package_hash: ContractPackageHash) {
        CURVETOKENV3::init(self, contract_hash, package_hash);
    }
}

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    CurveTokenV3::default().constructor(contract_hash, package_hash);
}
#[no_mangle]
fn decimals() {
    let ret = CURVETOKENV3::decimals(&CurveTokenV3::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn transfer() {
    let recipient: Address = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    CURVETOKENV3::transfer(&CurveTokenV3::default(), recipient, amount).unwrap_or_revert();
}
#[no_mangle]
fn transfer_from() {
    let owner: Address = runtime::get_named_arg("owner");
    let recipient: Address = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    CURVETOKENV3::transfer_from(&CurveTokenV3::default(), owner, recipient, amount)
        .unwrap_or_revert();
}
#[no_mangle]
fn approve() {
    let spender: Address = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    CURVETOKENV3::approve(&CurveTokenV3::default(), spender, amount).unwrap_or_revert();
}
#[no_mangle]
fn increase_allowance() {
    let spender: Address = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    CURVEERC20::increase_allowance(&CurveTokenV3::default(), spender, amount).unwrap_or_revert();
}
#[no_mangle]
fn decrease_allowance() {
    let spender: Address = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    CURVETOKENV3::decrease_allowance(&CurveTokenV3::default(), spender, amount).unwrap_or_revert();
}
#[no_mangle]
fn mint() {
    let to: Address = runtime::get_named_arg("to");
    let amount: U256 = runtime::get_named_arg("amount");
    CURVETOKENV3::mint(&CurveTokenV3::default(), to, amount).unwrap_or_revert();
}
#[no_mangle]
fn burn_from() {
    let from: Address = runtime::get_named_arg("from");
    let amount: U256 = runtime::get_named_arg("amount");
    CurveTokenV3::default()
        .burn_from(from, amount)
        .unwrap_or_revert();
}
#[no_mangle]
fn set_minter() {
    let minter: Key = runtime::get_named_arg("minter");
    CurveTokenV3::default().set_minter(minter);
}
#[no_mangle]
fn set_name() {
    let name: String = runtime::get_named_arg("name");
    let symbol: String = runtime::get_named_arg("symbol");
    CURVETOKENV3::set_name(&CurveTokenV3::default(), name, symbol);
}
//[no_mangle] of public variables
#[no_mangle]
fn name() {
    runtime::ret(CLValue::from_t(CURVEERC20::name(&CurveTokenV3::default())).unwrap_or_revert());
}
#[no_mangle]
fn symbol() {
    runtime::ret(CLValue::from_t(CURVEERC20::symbol(&CurveTokenV3::default())).unwrap_or_revert());
}
#[no_mangle]
fn balance_of() {
    let address: Address = runtime::get_named_arg("address");
    runtime::ret(
        CLValue::from_t(CURVEERC20::balance_of(&CurveTokenV3::default(), address)).unwrap_or_revert(),
    );
}
#[no_mangle]
fn allowance() {
    let owner: Address = runtime::get_named_arg("owner");
    let spender: Address = runtime::get_named_arg("spender");
    runtime::ret(
        CLValue::from_t(CURVEERC20::allowance(
            &CurveTokenV3::default(),
            owner,
            spender,
        ))
        .unwrap_or_revert(),
    );
}
#[no_mangle]
fn total_supply() {
    runtime::ret(
        CLValue::from_t(CURVEERC20::total_supply(&CurveTokenV3::default())).unwrap_or_revert(),
    );
}
#[no_mangle]
fn minter() {
    runtime::ret(CLValue::from_t(data::get_minter()).unwrap_or_revert());
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "decimals",
        vec![],
        U256::cl_type(),
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
        "burn_from",
        vec![
            Parameter::new("from", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
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
        "set_name",
        vec![
            Parameter::new("name", String::cl_type()),
            Parameter::new("symbol", String::cl_type()),
        ],
        <()>::cl_type(),
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
        "total_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "minter",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("address", Address::cl_type())],
        CLType::U256,
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
    entry_points
}

#[no_mangle]
fn call() {
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        let name: String = runtime::get_named_arg("name");
        let symbol: String = runtime::get_named_arg("symbol");
        // Build new package with initial a first version of the contract.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        let (contract_hash, _) = storage::add_contract_version(
            package_hash,
            get_entry_points(),
            CURVETOKENV3::named_keys(&CurveTokenV3::default(), name, symbol).unwrap_or_revert(),
        );
        // Prepare constructor args
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
