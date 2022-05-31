#![no_main]
#![no_std]
extern crate alloc;
use alloc::{collections::BTreeSet, format, vec, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::Bytes, runtime_args, CLTyped, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use gauge_proxy_crate::{self, GAUGEPROXY};

#[derive(Default)]
struct GaugeProxy(OnChainContractStorage);
impl ContractContext<OnChainContractStorage> for GaugeProxy {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl GAUGEPROXY<OnChainContractStorage> for GaugeProxy {}
impl GaugeProxy {
    fn constructor(
        &mut self,
        ownership_admin: Key,
        emergency_admin: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        GAUGEPROXY::init(
            self,
            ownership_admin,
            emergency_admin,
            contract_hash,
            package_hash,
        );
    }
}

#[no_mangle]
fn constructor() {
    let ownership_admin: Key = runtime::get_named_arg("ownership_admin");
    let emergency_admin: Key = runtime::get_named_arg("emergency_admin");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    GaugeProxy::default().constructor(
        ownership_admin,
        emergency_admin,
        contract_hash,
        package_hash,
    );
}

#[no_mangle]
fn commit_set_admins() {
    let o_admin: Key = runtime::get_named_arg("o_admin");
    let e_admin: Key = runtime::get_named_arg("e_admin");
    GaugeProxy::default().commit_set_admins(o_admin, e_admin);
}

#[no_mangle]
fn accept_set_admins() {
    GaugeProxy::default().accept_set_admins();
}

#[no_mangle]
fn commit_transfer_ownership() {
    let gauge: Key = runtime::get_named_arg("gauge");
    let new_owner: Key = runtime::get_named_arg("new_owner");
    GaugeProxy::default().commit_transfer_ownership(gauge, new_owner);
}

#[no_mangle]
fn accept_transfer_ownership() {
    let gauge: Key = runtime::get_named_arg("gauge");
    GaugeProxy::default().accept_transfer_ownership(gauge);
}

#[no_mangle]
fn set_killed() {
    let gauge: Key = runtime::get_named_arg("gauge");
    let is_killed: bool = runtime::get_named_arg("is_killed");
    GaugeProxy::default().set_killed(gauge, is_killed);
}

#[no_mangle]
fn set_rewards() {
    let gauge: Key = runtime::get_named_arg("gauge");
    let reward_contract: Key = runtime::get_named_arg("reward_contract");
    let sigs: Bytes = runtime::get_named_arg("sigs");
    let reward_tokens: Vec<Key> = runtime::get_named_arg("reward_tokens");
    GaugeProxy::default().set_rewards(gauge, reward_contract, sigs, reward_tokens);
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("ownership_admin", Key::cl_type()),
            Parameter::new("emergency_admin", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "commit_set_admins",
        vec![
            Parameter::new("o_admin", Key::cl_type()),
            Parameter::new("e_admin", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "accept_set_admins",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "commit_transfer_ownership",
        vec![
            Parameter::new("gauge", Key::cl_type()),
            Parameter::new("new_owner", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "accept_transfer_ownership",
        vec![Parameter::new("gauge", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_killed",
        vec![
            Parameter::new("gauge", Key::cl_type()),
            Parameter::new("is_killed", bool::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_rewards",
        vec![
            Parameter::new("gauge", Key::cl_type()),
            Parameter::new("reward_contract", Key::cl_type()),
            Parameter::new("sigs", Bytes::cl_type()),
            Parameter::new("reward_tokens", Vec::<Key>::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    // Store contract in the account's named keys. Contract name must be same for all new versions of the contracts
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");

    // If this is the first deployment
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        // Build new package.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        // add a first version to this package
        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        let ownership_admin: Key = runtime::get_named_arg("ownership_admin");
        let emergency_admin: Key = runtime::get_named_arg("emergency_admin");
        let constructor_args = runtime_args! {
            "ownership_admin" => ownership_admin,
            "emergency_admin" => emergency_admin,
            "package_hash" => package_hash,
            "contract_hash" => contract_hash,
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
    }
    // If contract package did already exist
    else {
        // get the package
        let package_hash: ContractPackageHash =
            runtime::get_key(&format!("{}_package_hash", contract_name))
                .unwrap_or_revert()
                .into_hash()
                .unwrap()
                .into();
        // create new version and install it
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