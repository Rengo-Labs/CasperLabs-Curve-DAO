#![no_main]
#![no_std]

extern crate alloc;
use alloc::{collections::BTreeSet, format, vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use casperlabs_contract_utils::{ContractContext, OnChainContractStorage};
use casperlabs_lp_token_wrapper::{self, data::*, Address, CURVEERC20, LPTOKENWRAPPER};

#[derive(Default)]
struct LpTokenWrapper(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for LpTokenWrapper {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}
impl LPTOKENWRAPPER<OnChainContractStorage> for LpTokenWrapper {}
impl CURVEERC20<OnChainContractStorage> for LpTokenWrapper {}

impl LpTokenWrapper {
    fn constructor(
        &self,
        uni: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        LPTOKENWRAPPER::init(self, uni, contract_hash, package_hash);
    }
}

#[no_mangle]
fn constructor() {
    let uni: Key = runtime::get_named_arg("uni");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    LpTokenWrapper::default().constructor(uni, contract_hash, package_hash);
}

// This function is to return the total token supply
#[no_mangle]
fn total_supply() {
    let ret: U256 = LpTokenWrapper::default().total_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// This function is to return the balance of the passed address
/// # Parameters
/// * `owner` - Address that holds the account address of the user
#[no_mangle]
fn balance_of() {
    let owner: Address = runtime::get_named_arg("owner");
    let ret: U256 = LpTokenWrapper::default().balance_of(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// This function is to stake the curve rewards amount
/// # Parameters
/// * `amount` - Amount to be staked
#[no_mangle]
fn stake() {
    let amount: U256 = runtime::get_named_arg("amount");
    LpTokenWrapper::default().stake(amount);
}

// This function is to withdraw the curve rewards amount
/// # Parameters
/// * `amount` - Amount to be withdrawn
#[no_mangle]
fn withdraw() {
    let amount: U256 = runtime::get_named_arg("amount");
    LpTokenWrapper::default().withdraw(amount);
}

//Variables
#[no_mangle]
fn uni() {
    runtime::ret(CLValue::from_t(get_uni()).unwrap_or_revert());
}

//Entry Points
fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("uni", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
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
        "balance_of",
        vec![Parameter::new("owner", Address::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "stake",
        vec![Parameter::new("amount", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "withdraw",
        vec![Parameter::new("amount", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "uni",
        vec![],
        Key::cl_type(),
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
        // Build new package with initial a first version of the contract.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        let (contract_hash, _) = storage::add_contract_version(
            package_hash,
            get_entry_points(),
            LpTokenWrapper::default()
                .named_keys("Lp Token".into(), "LP".into(), 9, 0.into())
                .unwrap_or_revert(),
        );
        let uni: Key = runtime::get_named_arg("uni");
        // Prepare constructor args
        let constructor_args = runtime_args! {
            "uni" => uni,
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
