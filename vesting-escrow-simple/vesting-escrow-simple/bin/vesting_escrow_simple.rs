#![no_main]
#![no_std]

extern crate alloc;
use alloc::{collections::BTreeSet, format};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLValue, ContractHash, ContractPackageHash, Key, RuntimeArgs, URef, U256,
};
use casperlabs_contract_utils::{ContractContext, OnChainContractStorage};
use casperlabs_erc20::{self, ERC20};
use vesting_escrow_simple_crate::{data, entry_points, VESTINGESCROWSIMPLE};

#[derive(Default)]
struct VestingEscrowSimple(OnChainContractStorage);
impl ContractContext<OnChainContractStorage> for VestingEscrowSimple {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}
impl ERC20<OnChainContractStorage> for VestingEscrowSimple {}
impl VESTINGESCROWSIMPLE<OnChainContractStorage> for VestingEscrowSimple {}

impl VestingEscrowSimple {
    fn constructor(&mut self, contract_hash: ContractHash, package_hash: ContractPackageHash) {
        VESTINGESCROWSIMPLE::init(self, contract_hash, package_hash);
    }
}

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    VestingEscrowSimple::default().constructor(contract_hash, package_hash);
}
#[no_mangle]
fn initialize() {
    let admin: Key = runtime::get_named_arg("admin");
    let token: Key = runtime::get_named_arg("token");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let start_time: U256 = runtime::get_named_arg("start_time");
    let end_time: U256 = runtime::get_named_arg("end_time");
    let can_disable: bool = runtime::get_named_arg("can_disable");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let ret = VestingEscrowSimple::default().initialize(
        admin,
        token,
        recipient,
        amount,
        start_time,
        end_time,
        can_disable,
        contract_hash,
        package_hash,
    );
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn toggle_disable() {
    let recipient: Key = runtime::get_named_arg("recipient");
    VestingEscrowSimple::default().toggle_disable(recipient);
}

#[no_mangle]
fn disable_can_disable() {
    VestingEscrowSimple::default().disable_can_disable();
}
#[no_mangle]
fn vested_of() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let ret = VestingEscrowSimple::default().vested_of(recipient);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn balance_of() {
    let address: Key = runtime::get_named_arg("address");
    let ret = VestingEscrowSimple::default().balance_of(address);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn vested_supply() {
    let ret = VestingEscrowSimple::default().vested_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn locked_supply() {
    let ret = VestingEscrowSimple::default().locked_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn locked_of() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let ret = VestingEscrowSimple::default().locked_of(recipient);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn commit_transfer_ownership() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret = VestingEscrowSimple::default().commit_transfer_ownership(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn apply_transfer_ownership() {
    let ret = VestingEscrowSimple::default().apply_transfer_ownership();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn claim() {
    let addr: Option<Key> = runtime::get_named_arg("addr");
    VestingEscrowSimple::default().claim(addr);
}
//[no_mangle] of public variables
#[no_mangle]
fn token() {
    runtime::ret(CLValue::from_t(data::get_token()).unwrap_or_revert());
}
#[no_mangle]
fn start_time() {
    runtime::ret(CLValue::from_t(data::get_start_time()).unwrap_or_revert());
}
#[no_mangle]
fn end_time() {
    runtime::ret(CLValue::from_t(data::get_end_time()).unwrap_or_revert());
}
#[no_mangle]
fn initial_locked_supply() {
    runtime::ret(CLValue::from_t(data::get_initial_locked_supply()).unwrap_or_revert());
}
#[no_mangle]
fn can_disable() {
    runtime::ret(CLValue::from_t(data::get_can_disable()).unwrap_or_revert());
}
#[no_mangle]
fn admin() {
    runtime::ret(CLValue::from_t(data::get_admin()).unwrap_or_revert());
}
#[no_mangle]
fn future_admin() {
    runtime::ret(CLValue::from_t(data::get_future_admin()).unwrap_or_revert());
}
#[no_mangle]
fn initial_locked() {
    let owner: Key = runtime::get_named_arg("owner");
    runtime::ret(CLValue::from_t(data::InitialLocked::instance().get(&owner)).unwrap_or_revert());
}
#[no_mangle]
fn total_claimed() {
    let owner: Key = runtime::get_named_arg("owner");
    runtime::ret(CLValue::from_t(data::TotalClaimed::instance().get(&owner)).unwrap_or_revert());
}
#[no_mangle]
fn disabled_at() {
    let owner: Key = runtime::get_named_arg("owner");
    runtime::ret(CLValue::from_t(data::DisableddAt::instance().get(&owner)).unwrap_or_revert());
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
            entry_points::get_entry_points(),
            Default::default(),
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

        let (contract_hash, _): (ContractHash, _) = storage::add_contract_version(
            package_hash,
            entry_points::get_entry_points(),
            Default::default(),
        );
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
