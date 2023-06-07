#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;

use alloc::{boxed::Box, collections::BTreeSet, format, string::String};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use casperlabs_contract_utils::{ContractContext, OnChainContractStorage};
use vesting_escrow_factory_crate::VESTINGESCROWFACTORY;
use vesting_escrow_simple_crate::{data as ves_data, VESTINGESCROWSIMPLE};

#[derive(Default)]
struct Token(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Token {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}
impl VESTINGESCROWSIMPLE<OnChainContractStorage> for Token {}
impl VESTINGESCROWFACTORY<OnChainContractStorage> for Token {}
impl Token {
    fn constructor_vef(
        &mut self,
        _target: Key,
        _admin: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        VESTINGESCROWFACTORY::init(
            self,
            _target,
            _admin,
            Key::from(contract_hash),
            package_hash,
        );
    }
}

/// @notice Contract constructor
/// @dev Prior to deployment you must deploy one copy of `VestingEscrowSimple` which
///      is used as a library for vesting contracts deployed by this factory
/// @param _target `VestingEscrowSimple` contract address

#[no_mangle]
fn constructor_vef() {
    let target: Key = runtime::get_named_arg::<Key>("target");
    let admin: Key = runtime::get_named_arg("admin");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");

    Token::default().constructor_vef(target, admin, contract_hash, package_hash);
}

#[no_mangle]
fn package_hash() {
    let ret: ContractPackageHash = Token::default().get_package_hash();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn admin_vef() {
    let ret: Key = Token::default().admin();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn target() {
    let ret: Key = Token::default().target();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn future_admin_vef() {
    let ret: Key = Token::default().future_admin();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Accept a pending ownership transfer

#[no_mangle]
fn apply_transfer_ownership_vef() {
    let ret: bool = Token::default().apply_transfer_ownership();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Transfer ownership of GaugeController to `addr`
/// @param addr Address to have ownership transferred to

#[no_mangle]
fn commit_transfer_ownership_vef() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: bool = Token::default().commit_transfer_ownership(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Deploy a new vesting contract
/// @dev Each contract holds tokens which vest for a single account. Tokens
///         must be sent to this contract via the regular `ERC20.transfer` method
///         prior to calling this method.
/// @param _token Address of the ERC20 token being distributed
/// @param _recipient Address to vest tokens for
/// @param _amount Amount of tokens being vested for `_recipient`
/// @param _can_disable Can admin disable recipient's ability to claim tokens?
/// @param _vesting_duration Time period over which tokens are released
/// @param _vesting_start Epoch time when tokens begin to vest

#[no_mangle]
fn deploy_vesting_contract() {
    let token: Key = runtime::get_named_arg("token");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let can_disable: bool = runtime::get_named_arg("can_disable");
    let vesting_duration: U256 = runtime::get_named_arg("vesting_duration");
    let vesting_start: Option<U256> = runtime::get_named_arg("vesting_start");
    let ret: Key = Token::default().deploy_vesting_contract(
        token,
        recipient,
        amount,
        can_disable,
        vesting_duration,
        vesting_start,
    );
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

//VESTING ESCROW SIMPLE NO MANGLE
#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    VESTINGESCROWSIMPLE::init(&Token::default(), contract_hash, package_hash);
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
    let ret = VESTINGESCROWSIMPLE::initialize(
        &Token::default(),
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
    VESTINGESCROWSIMPLE::toggle_disable(&Token::default(), recipient);
}

#[no_mangle]
fn disable_can_disable() {
    VESTINGESCROWSIMPLE::disable_can_disable(&Token::default());
}
#[no_mangle]
fn vested_of() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let ret = VESTINGESCROWSIMPLE::vested_of(&Token::default(), recipient);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn balance_of() {
    let address: Key = runtime::get_named_arg("address");
    let ret = VESTINGESCROWSIMPLE::balance_of(&Token::default(), address);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn vested_supply() {
    let ret = VESTINGESCROWSIMPLE::vested_supply(&Token::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn locked_supply() {
    let ret = VESTINGESCROWSIMPLE::locked_supply(&Token::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn locked_of() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let ret = VESTINGESCROWSIMPLE::locked_of(&Token::default(), recipient);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn commit_transfer_ownership() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret = VESTINGESCROWSIMPLE::commit_transfer_ownership(&Token::default(), addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn apply_transfer_ownership() {
    let ret = VESTINGESCROWSIMPLE::apply_transfer_ownership(&Token::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn claim() {
    let addr: Option<Key> = runtime::get_named_arg("addr");
    VESTINGESCROWSIMPLE::claim(&Token::default(), addr);
}
//[no_mangle] of public variables
#[no_mangle]
fn token() {
    runtime::ret(CLValue::from_t(ves_data::get_token()).unwrap_or_revert());
}
#[no_mangle]
fn start_time() {
    runtime::ret(CLValue::from_t(ves_data::get_start_time()).unwrap_or_revert());
}
#[no_mangle]
fn end_time() {
    runtime::ret(CLValue::from_t(ves_data::get_end_time()).unwrap_or_revert());
}
#[no_mangle]
fn initial_locked_supply() {
    runtime::ret(CLValue::from_t(ves_data::get_initial_locked_supply()).unwrap_or_revert());
}
#[no_mangle]
fn can_disable() {
    runtime::ret(CLValue::from_t(ves_data::get_can_disable()).unwrap_or_revert());
}
#[no_mangle]
fn admin() {
    runtime::ret(CLValue::from_t(ves_data::get_admin()).unwrap_or_revert());
}
#[no_mangle]
fn future_admin() {
    runtime::ret(CLValue::from_t(ves_data::get_future_admin()).unwrap_or_revert());
}
#[no_mangle]
fn initial_locked() {
    let owner: Key = runtime::get_named_arg("owner");
    runtime::ret(
        CLValue::from_t(ves_data::InitialLocked::instance().get(&owner)).unwrap_or_revert(),
    );
}
#[no_mangle]
fn total_claimed() {
    let owner: Key = runtime::get_named_arg("owner");
    runtime::ret(
        CLValue::from_t(ves_data::TotalClaimed::instance().get(&owner)).unwrap_or_revert(),
    );
}
#[no_mangle]
fn disabled_at() {
    let owner: Key = runtime::get_named_arg("owner");
    runtime::ret(CLValue::from_t(ves_data::DisableddAt::instance().get(&owner)).unwrap_or_revert());
}

#[no_mangle]
fn call() {
    // Contract name must be same for all new versions of the contracts
    let contract_name: String = runtime::get_named_arg("contract_name");

    // If this is the first deployment
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        // Build new package with initial a first version of the contract.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        let (contract_hash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());
        // Read arguments for the constructor call.
        let target: Key = runtime::get_named_arg("target");
        let admin: Key = runtime::get_named_arg("admin");

        // Prepare constructor args
        let constructor_args = runtime_args! {
            "target" => target,
            "admin" => admin,
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
        let _: () = runtime::call_versioned_contract(
            package_hash,
            None,
            "constructor_vef",
            constructor_args,
        );

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

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor_vef",
        vec![
            Parameter::new("token", Key::cl_type()),
            Parameter::new("start_time", U256::cl_type()),
            Parameter::new("end_time", U256::cl_type()),
            Parameter::new("can_disable", bool::cl_type()),
            Parameter::new("fund_admins", CLType::List(Box::new(String::cl_type()))),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "package_hash",
        vec![],
        ContractPackageHash::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "admin_vef",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "target",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "future_admin_vef",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "commit_transfer_ownership_vef",
        vec![Parameter::new("addr", Key::cl_type())],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "apply_transfer_ownership_vef",
        vec![],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "deploy_vesting_contract",
        vec![
            Parameter::new("token", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
            Parameter::new("can_disable", bool::cl_type()),
            Parameter::new("vesting_duration", U256::cl_type()),
            Parameter::new("vesting_start", CLType::Option(Box::new(U256::cl_type()))),
        ],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    // ENTRYPOINTS OF VESTING ESCROW SIMPLE
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("token", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "initialize",
        vec![
            Parameter::new("admin", Key::cl_type()),
            Parameter::new("token", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
            Parameter::new("start_time", U256::cl_type()),
            Parameter::new("end_time", U256::cl_type()),
            Parameter::new("can_disable", bool::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "toggle_disable",
        vec![Parameter::new("recipient", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "disable_can_disable",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "vested_of",
        vec![Parameter::new("recipient", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "vested_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "locked_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "locked_of",
        vec![Parameter::new("recipient", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "commit_transfer_ownership",
        vec![Parameter::new("addr", Key::cl_type())],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "apply_transfer_ownership",
        vec![],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("address", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claim",
        vec![Parameter::new(
            "addr",
            CLType::Option(Box::new(CLType::Key)),
        )],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "token",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "start_time",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "end_time",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "initial_locked",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_claimed",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "initial_locked_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "can_disable",
        vec![],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "disabled_at",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "admin",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "future_admin",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}
