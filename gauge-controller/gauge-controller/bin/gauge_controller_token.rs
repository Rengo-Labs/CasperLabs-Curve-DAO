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
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U128,
    U256,
};
use casperlabs_contract_utils::{ContractContext, OnChainContractStorage};
use gauge_controller_crate::{
    data::{Point, VotedSlope},
    utils::*,
    GAUGECONLTROLLER,
};

#[derive(Default)]
struct Token(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Token {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl GAUGECONLTROLLER<OnChainContractStorage> for Token {}
impl Token {
    fn constructor(
        &mut self,
        token: Key,
        voting_escrow: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        GAUGECONLTROLLER::init(
            self,
            token,
            voting_escrow,
            Key::from(contract_hash),
            package_hash,
        );
    }
}

#[no_mangle]
fn constructor() {
    let token: Key = runtime::get_named_arg::<Key>("token");
    let voting_escrow: Key = runtime::get_named_arg("voting_escrow");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");

    Token::default().constructor(token, voting_escrow, contract_hash, package_hash);
}

/// @notice Transfer ownership of GaugeController to `addr`
/// @param addr Address to have ownership transferred to

#[no_mangle]
fn commit_transfer_ownership() {
    let addr: Key = runtime::get_named_arg::<Key>("addr");
    Token::default().commit_transfer_ownership(addr);
}

/// @notice Apply pending ownership transfer

#[no_mangle]
fn apply_transfer_ownership() {
    Token::default().apply_transfer_ownership();
}

/// @notice Get gauge type for address
/// @param _addr Gauge address
/// @return Gauge type id
#[no_mangle]
fn gauge_types() {
    let addr: Key = runtime::get_named_arg::<Key>("addr");
    let ret: i128 = Token::default().gauge_types(addr);
    runtime::ret(CLValue::from_t(i128_to_tuple(ret)).unwrap_or_revert());
}

/// @notice Checkpoint to fill data common for all gauges

#[no_mangle]
fn checkpoint() {
    Token::default().checkpoint();
}

/// @notice Checkpoint to fill data for both a specific gauge and common for all gauges
/// @param addr Gauge address

#[no_mangle]
fn checkpoint_gauge() {
    let addr: Key = runtime::get_named_arg::<Key>("addr");
    Token::default().checkpoint_gauge(addr);
}

#[no_mangle]
fn package_hash() {
    let ret: ContractPackageHash = Token::default().get_package_hash();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
/// @notice Get Gauge relative weight (not more than 1.0) normalized to 1e9
///         (e.g. 1.0 == 1e9). Inflation which will be received by it is
///         inflation_rate * relative_weight / 1e9
/// @param addr Gauge address
/// @param time Relative weight at the specified timestamp in the past or present
/// @return Value of relative weight normalized to 1e9

#[no_mangle]
fn gauge_relative_weight() {
    let addr: Key = runtime::get_named_arg("addr");
    let time: Option<U256> = runtime::get_named_arg("time");
    let ret: U256 = Token::default().gauge_relative_weight(addr, time);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Get gauge weight normalized to 1e9 and also fill all the unfilled
///         values for type and gauge records
/// @dev Any address can call, however nothing is recorded if the values are filled already
/// @param addr Gauge address
/// @param time Relative weight at the specified timestamp in the past or present
/// @return Value of relative weight normalized to 1e9

#[no_mangle]
fn gauge_relative_weight_write() {
    let addr: Key = runtime::get_named_arg("addr");
    let time: Option<U256> = runtime::get_named_arg("time");
    let ret: U256 = Token::default().gauge_relative_weight_write(addr, time);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Change gauge type `type_id` weight to `weight`
/// @param type_id Gauge type id
/// @param weight New Gauge weight

#[no_mangle]
fn change_type_weight() {
    let type_id: (bool, U128) = runtime::get_named_arg("type_id");
    let weight: U256 = runtime::get_named_arg("weight");

    Token::default().change_type_weight(tuple_to_i128(type_id), weight);
}

/// @notice Change weight of gauge `addr` to `weight`
/// @param addr `GaugeController` contract address
/// @param weight New Gauge weight

#[no_mangle]
fn change_gauge_weight() {
    let addr: Key = runtime::get_named_arg("addr");
    let weight: U256 = runtime::get_named_arg("weight");

    Token::default().change_gauge_weight(addr, weight);
}

/// @notice Get current gauge weight
/// @param addr Gauge address
/// @return Gauge weight

#[no_mangle]
fn get_gauge_weight() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: U256 = Token::default().get_gauge_weight(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Get current type weight
/// @param type_id Type id
/// @return Type weight

#[no_mangle]
fn get_type_weight() {
    let type_id: (bool, U128) = runtime::get_named_arg("type_id");
    let ret: U256 = Token::default().get_type_weight(tuple_to_i128(type_id));
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Get current total (type-weighted) weight
/// @return Total weight

#[no_mangle]
fn get_total_weight() {
    let ret: U256 = Token::default().get_total_weight();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Get sum of gauge weights per type
/// @param type_id Type id
/// @return Sum of gauge weights

#[no_mangle]
fn get_weights_sum_per_type() {
    let type_id: (bool, U128) = runtime::get_named_arg("type_id");
    let ret: U256 = Token::default().get_weights_sum_per_type(tuple_to_i128(type_id));
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn changes_sum() {
    let owner: (bool, U128) = runtime::get_named_arg("owner");
    let spender: U256 = runtime::get_named_arg("spender");
    let ret: U256 = Token::default().changes_sum(tuple_to_i128(owner), spender);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn changes_weight() {
    let owner: Key = runtime::get_named_arg("owner");
    let spender: U256 = runtime::get_named_arg("spender");
    let ret: U256 = Token::default().changes_weight(owner, spender);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn gauge_type_names() {
    let owner: (bool, U128) = runtime::get_named_arg("owner");
    let ret: String = Token::default().gauge_type_names(tuple_to_i128(owner));
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// # we increment values by 1 prior to storing them here so we can rely on a value
/// # of zero as meaning the gauge has not been set

#[no_mangle]
fn gauge_types_() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: i128 = Token::default().gauge_types_(owner);
    runtime::ret(CLValue::from_t(i128_to_tuple(ret)).unwrap_or_revert());
}
#[no_mangle]
fn gauges() {
    let owner: U256 = runtime::get_named_arg("owner");
    let ret: Key = Token::default().gauges(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn last_user_vote() {
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    let ret: U256 = Token::default().last_user_vote(owner, spender);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn points_sum() {
    let owner: (bool, U128) = runtime::get_named_arg("owner");
    let spender: U256 = runtime::get_named_arg("spender");
    let ret: Point = Token::default().points_sum(tuple_to_i128(owner), spender);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn points_total() {
    let owner: U256 = runtime::get_named_arg("owner");
    let ret: U256 = Token::default().points_total(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn points_type_weight() {
    let owner: (bool, U128) = runtime::get_named_arg("owner");
    let spender: U256 = runtime::get_named_arg("spender");
    let ret: U256 = Token::default().points_type_weight(tuple_to_i128(owner), spender);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn points_weight() {
    let owner: Key = runtime::get_named_arg("owner");
    let spender: U256 = runtime::get_named_arg("spender");
    let ret: Point = Token::default().points_weight(owner, spender);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn time_sum() {
    let owner: U256 = runtime::get_named_arg("owner");
    let ret: U256 = Token::default().time_sum(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn time_type_weight() {
    let owner: U256 = runtime::get_named_arg("owner");
    let ret: U256 = Token::default().time_type_weight(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn time_weight() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = Token::default().time_weight(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn vote_user_power() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = Token::default().vote_user_power(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn vote_user_slopes() {
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    let ret: VotedSlope = Token::default().vote_user_slopes(owner, spender);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
// TimeWeight, VoteUserPower, VoteUserSlopes,
#[no_mangle]
fn time_total() {
    let ret: U256 = Token::default().time_total();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn token() {
    let ret: Key = Token::default().token();
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
fn voting_escrow() {
    let ret: Key = Token::default().voting_escrow();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn n_gauge_types() {
    let ret: i128 = Token::default().n_gauge_types();
    runtime::ret(CLValue::from_t(i128_to_tuple(ret)).unwrap_or_revert());
}
#[no_mangle]
fn n_gauges() {
    let ret: i128 = Token::default().n_gauges();
    runtime::ret(CLValue::from_t(i128_to_tuple(ret)).unwrap_or_revert());
}

/// @notice Add gauge type with name `_name` and weight `weight`
/// @param _name Name of gauge type
/// @param weight Weight of gauge type
#[inline(always)]
#[no_mangle]
fn add_type() {
    let name: String = runtime::get_named_arg("name");
    let weight: Option<U256> = runtime::get_named_arg("weight");
    Token::default().add_type(name, weight);
}

/// @notice Add gauge `addr` of type `gauge_type` with weight `weight`
/// @param addr Gauge address
/// @param gauge_type Gauge type
/// @param weight Gauge weight
#[inline(always)]
#[no_mangle]
fn add_gauge() {
    let addr: Key = runtime::get_named_arg("addr");
    let gauge_type: (bool, U128) = runtime::get_named_arg("gauge_type");
    let weight: Option<U256> = runtime::get_named_arg("weight");
    Token::default().add_gauge(addr, tuple_to_i128(gauge_type), weight);
}

/// @notice Allocate voting power for changing pool weights
/// @param _gauge_addr Gauge which `msg.sender` votes for
/// @param _user_weight Weight for a gauge in bps (units of 0.01%). Minimal is 0.01%. Ignored if 0
#[no_mangle]
fn vote_for_gauge_weights() {
    let gauge_addr: Key = runtime::get_named_arg("gauge_addr");
    let user_weight: U256 = runtime::get_named_arg("user_weight");
    Token::default().vote_for_gauge_weights(gauge_addr, user_weight);
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
        let voting_escrow: Key = runtime::get_named_arg("voting_escrow");

        // Prepare constructor args
        let constructor_args = runtime_args! {
            "token" => token,
            "voting_escrow" => voting_escrow,
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

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("token", Key::cl_type()),
            Parameter::new("voting_escrow", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "commit_transfer_ownership",
        vec![Parameter::new("addr", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "apply_transfer_ownership",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "checkpoint",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "checkpoint_gauge",
        vec![Parameter::new("addr", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "gauge_types",
        vec![Parameter::new("addr", Key::cl_type())],
        CLType::Tuple2([Box::new(CLType::Bool), Box::new(CLType::U128)]),
        EntryPointAccess::Public,
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
        "n_gauge_types",
        vec![],
        CLType::Tuple2([Box::new(CLType::Bool), Box::new(CLType::U128)]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "n_gauges",
        vec![],
        CLType::Tuple2([Box::new(CLType::Bool), Box::new(CLType::U128)]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "voting_escrow",
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
        "admin",
        vec![],
        Key::cl_type(),
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
        "time_total",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "vote_user_slopes",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
        ],
        VotedSlope::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "vote_user_power",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "time_weight",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "time_type_weight",
        vec![Parameter::new("owner", U256::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "time_sum",
        vec![Parameter::new("owner", U256::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "points_weight",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", U256::cl_type()),
        ],
        Point::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "points_type_weight",
        vec![
            Parameter::new(
                "owner",
                CLType::Tuple2([Box::new(CLType::Bool), Box::new(CLType::U128)]),
            ),
            Parameter::new("spender", U256::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "points_total",
        vec![Parameter::new("owner", U256::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "points_sum",
        vec![
            Parameter::new(
                "owner",
                CLType::Tuple2([Box::new(CLType::Bool), Box::new(CLType::U128)]),
            ),
            Parameter::new("spender", U256::cl_type()),
        ],
        Point::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "last_user_vote",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "gauges",
        vec![Parameter::new("owner", U256::cl_type())],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "gauge_types_",
        vec![Parameter::new("owner", Key::cl_type())],
        CLType::Tuple2([Box::new(CLType::Bool), Box::new(CLType::U128)]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "gauge_type_names",
        vec![Parameter::new(
            "owner",
            CLType::Tuple2([Box::new(CLType::Bool), Box::new(CLType::U128)]),
        )],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "changes_weight",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", U256::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "changes_sum",
        vec![
            Parameter::new(
                "owner",
                CLType::Tuple2([Box::new(CLType::Bool), Box::new(CLType::U128)]),
            ),
            Parameter::new("spender", U256::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "gauge_relative_weight",
        vec![
            Parameter::new("addr", Key::cl_type()),
            Parameter::new("time", CLType::Option(Box::new(U256::cl_type()))),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "gauge_relative_weight_write",
        vec![
            Parameter::new("addr", Key::cl_type()),
            Parameter::new("time", CLType::Option(Box::new(U256::cl_type()))),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "change_type_weight",
        vec![
            Parameter::new(
                "type_id",
                CLType::Tuple2([Box::new(CLType::Bool), Box::new(CLType::U128)]),
            ),
            Parameter::new("weight", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "change_gauge_weight",
        vec![
            Parameter::new("addr", Key::cl_type()),
            Parameter::new("weight", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_gauge_weight",
        vec![Parameter::new("addr", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_type_weight",
        vec![Parameter::new(
            "type_id",
            CLType::Tuple2([Box::new(CLType::Bool), Box::new(CLType::U128)]),
        )],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_total_weight",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_weights_sum_per_type",
        vec![Parameter::new(
            "type_id",
            CLType::Tuple2([Box::new(CLType::Bool), Box::new(CLType::U128)]),
        )],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "add_type",
        vec![
            Parameter::new("name", String::cl_type()),
            Parameter::new("weight", CLType::Option(Box::new(U256::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "add_gauge",
        vec![
            Parameter::new("addr", Key::cl_type()),
            Parameter::new(
                "type_id",
                CLType::Tuple2([Box::new(CLType::Bool), Box::new(CLType::U128)]),
            ),
            Parameter::new("weight", CLType::Option(Box::new(U256::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "vote_for_gauge_weights",
        vec![
            Parameter::new("gauge_addr", Key::cl_type()),
            Parameter::new("user_weight", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}
