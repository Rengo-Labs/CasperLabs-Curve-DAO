#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;
use alloc::vec::Vec;

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
use vesting_escrow_crate::VESTINGESCROW;

#[derive(Default)]
struct Token(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Token {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}
impl VESTINGESCROW<OnChainContractStorage> for Token {}
#[allow(clippy::too_many_arguments)]
impl Token {
    fn constructor(
        &mut self,
        _token: Key,
        _start_time: U256,
        _end_time: U256,
        _can_disable: bool,
        _fund_admins: Vec<String>,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        VESTINGESCROW::init(
            self,
            _token,
            _start_time,
            _end_time,
            _can_disable,
            _fund_admins,
            Key::from(contract_hash),
            package_hash,
        );
    }
}

/// @param _token Address of the ERC20 token being distributed
/// @param _start_time Timestamp at which the distribution starts. Should be in
///     the future, so that we have enough time to VoteLock everyone
/// @param _end_time Time until everything should be vested
/// @param _can_disable Whether admin can disable accounts in this deployment
/// @param _fund_admins Temporary admin accounts used only for funding

#[no_mangle]
fn constructor() {
    let token: Key = runtime::get_named_arg::<Key>("token");
    let start_time: U256 = runtime::get_named_arg("start_time");
    let end_time: U256 = runtime::get_named_arg("end_time");
    let can_disable: bool = runtime::get_named_arg("can_disable");
    let fund_admins: Vec<String> = runtime::get_named_arg("fund_admins");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");

    Token::default().constructor(
        token,
        start_time,
        end_time,
        can_disable,
        fund_admins,
        contract_hash,
        package_hash,
    );
}

#[no_mangle]
fn package_hash() {
    let ret: ContractPackageHash = Token::default().get_package_hash();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn admin() {
    let ret: Key = Token::default().admin();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn future_admin() {
    let ret: Key = Token::default().future_admin();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn fund_admins() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: bool = Token::default().fund_admins(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn disabled_at() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = Token::default().disabled_at(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn initial_locked() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = Token::default().initial_locked(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn total_claimed() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = Token::default().total_claimed(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn claim() {
    let owner: Option<Key> = runtime::get_named_arg("owner");
    Token::default().claim(owner);
}
#[no_mangle]
fn start_time() {
    let ret: U256 = Token::default().start_time();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn end_time() {
    let ret: U256 = Token::default().end_time();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn token() {
    let ret: Key = Token::default().token();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn fund_admins_enabled() {
    let ret: bool = Token::default().fund_admins_enabled();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn initial_locked_supply() {
    let ret: U256 = Token::default().initial_locked_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn unallocated_supply() {
    let ret: U256 = Token::default().unallocated_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn can_disable() {
    let ret: bool = Token::default().can_disable();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Accept a pending ownership transfer

#[no_mangle]
fn apply_transfer_ownership() {
    let ret: bool = Token::default().apply_transfer_ownership();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Transfer ownership of GaugeController to `addr`
/// @param addr Address to have ownership transferred to

#[no_mangle]
fn commit_transfer_ownership() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: bool = Token::default().commit_transfer_ownership(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Disable the funding admin accounts

#[no_mangle]
fn disable_fund_admins() {
    Token::default().disable_fund_admins();
}

/// @notice Disable the ability to call `toggle_disable`

#[no_mangle]
fn disable_can_disable() {
    Token::default().disable_can_disable();
}

/// @notice Disable or re-enable a vested address's ability to claim tokens
/// @dev When disabled, the address is only unable to claim tokens which are still
///      locked at the time of this call. It is not possible to block the claim
///      of tokens which have already vested.
/// @param _recipient Address to disable or enable

#[no_mangle]
fn toggle_disable() {
    let recipient: Key = runtime::get_named_arg("recipient");
    Token::default().toggle_disable(recipient);
}

/// @notice Get the total number of tokens which have vested, that are held
///         by this contract

#[no_mangle]
fn vested_supply() {
    let ret: U256 = Token::default().vested_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Get the total number of tokens which are still locked
///         (have not yet vested)

#[no_mangle]
fn locked_supply() {
    let ret: U256 = Token::default().locked_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Get the number of tokens which have vested for a given address
/// @param _recipient address to check

#[no_mangle]
fn vested_of() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let ret: U256 = Token::default().vested_of(recipient);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Get the number of unclaimed, vested tokens for a given address
/// @param _recipient address to check

#[no_mangle]
fn balance_of() {
    let address: Key = runtime::get_named_arg("address");
    let ret: U256 = Token::default().balance_of(address);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Get the number of locked tokens for a given address
/// @param _recipient address to check

#[no_mangle]
fn locked_of() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let ret: U256 = Token::default().locked_of(recipient);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Transfer vestable tokens into the contract
/// @dev Handled separate from `fund` to reduce transaction count when using funding admins
/// @param _amount Number of tokens to transfer

#[no_mangle]
fn add_tokens() {
    let amount: U256 = runtime::get_named_arg("amount");
    Token::default().add_tokens(amount);
}

/// @notice Vest tokens for multiple recipients
/// @param _recipients List of addresses to fund
/// @param _amounts Amount of vested tokens for each address

#[no_mangle]
fn fund() {
    let recipients: Vec<String> = runtime::get_named_arg("recipients");
    let amounts: Vec<U256> = runtime::get_named_arg("amounts");
    Token::default().fund(recipients, amounts);
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
        let token: Key = runtime::get_named_arg("token");
        let start_time: U256 = runtime::get_named_arg("start_time");
        let end_time: U256 = runtime::get_named_arg("end_time");
        let can_disable: bool = runtime::get_named_arg("can_disable");
        let fund_admins: Vec<String> = runtime::get_named_arg("fund_admins");
        // Prepare constructor args
        let constructor_args = runtime_args! {
            "token" => token,
            "start_time" => start_time,
            "end_time" => end_time,
            "can_disable" => can_disable,
            "fund_admins" => fund_admins,
            "contract_hash" => contract_hash,
            "package_hash"=> package_hash,
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

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
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

    entry_points.add_entry_point(EntryPoint::new(
        "initial_locked",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "add_tokens",
        vec![Parameter::new("amount", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "fund",
        vec![
            Parameter::new("recipients", CLType::List(Box::new(String::cl_type()))),
            Parameter::new("amounts", CLType::List(Box::new(U256::cl_type()))),
        ],
        <()>::cl_type(),
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
        "total_claimed",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "fund_admins",
        vec![Parameter::new("owner", Key::cl_type())],
        bool::cl_type(),
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
        "initial_locked_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "unallocated_supply",
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
        "fund_admins_enabled",
        vec![],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "disable_fund_admins",
        vec![],
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
        "toggle_disable",
        vec![Parameter::new("recipient", Key::cl_type())],
        bool::cl_type(),
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
        "vested_of",
        vec![Parameter::new("recipient", Key::cl_type())],
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
        U256::cl_type(),
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
        "locked_of",
        vec![Parameter::new("recipient", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points
}
