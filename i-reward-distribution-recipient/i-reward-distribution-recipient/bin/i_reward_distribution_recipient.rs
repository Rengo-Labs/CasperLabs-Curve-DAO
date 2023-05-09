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
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef,
};
use casperlabs_contract_utils::{ContractContext, OnChainContractStorage};
use casperlabs_i_reward_distribution_recipient::{IREWARDDISTRIBUTIONRECIPIENT, OWNABLE};

#[derive(Default)]
struct IRewardDistributionRecipient(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for IRewardDistributionRecipient {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl IREWARDDISTRIBUTIONRECIPIENT<OnChainContractStorage> for IRewardDistributionRecipient {}
impl OWNABLE<OnChainContractStorage> for IRewardDistributionRecipient {}

impl IRewardDistributionRecipient {
    fn constructor(&mut self, contract_hash: ContractHash, package_hash: ContractPackageHash) {
        IREWARDDISTRIBUTIONRECIPIENT::init(self, contract_hash, package_hash);
    }
}

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    IRewardDistributionRecipient::default().constructor(contract_hash, package_hash);
}

// Get the owner address of the contract
#[no_mangle]
fn owner() {
    let ret: Key = OWNABLE::owner(&IRewardDistributionRecipient::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// To check if the caller is owner or not
#[no_mangle]
fn is_owner() {
    let ret: bool = OWNABLE::is_owner(&IRewardDistributionRecipient::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// Revoke the admin address by chaning admin to ZERO_ADDRESS
#[no_mangle]
fn renounce_ownership() {
    OWNABLE::renounce_ownership(&mut IRewardDistributionRecipient::default());
}

// Change the ownership to the provided address `new_owner`
/// # Parameters
/// * `new_owner` - To be owner address
#[no_mangle]
fn transfer_ownership() {
    let new_owner: Key = runtime::get_named_arg("new_owner");
    OWNABLE::transfer_ownership(&mut IRewardDistributionRecipient::default(), new_owner);
}

// Set the reward_distribution contract package address
/// # Parameters
/// * `reward_distribution` - reward_distribution contract package address
#[no_mangle]
fn set_reward_distribution() {
    let reward_distribution: Key = runtime::get_named_arg("reward_distribution");
    IRewardDistributionRecipient::default().set_reward_distribution(reward_distribution);
}
//Entry Points
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
        "set_reward_distribution",
        vec![Parameter::new("reward_distribution", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "owner",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "is_owner",
        vec![],
        bool::cl_type(),
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
        let (contract_hash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());
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
