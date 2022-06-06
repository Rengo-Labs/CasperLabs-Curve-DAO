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
use contract_utils::{set_key, ContractContext, OnChainContractStorage};
use curve_rewards_crate::{data::*, CURVEREWARDS};
use i_reward_distribution_recipient_crate::IREWARDDISTRIBUTIONRECIPIENT;
use lp_token_wrapper_crate::LPTOKENWRAPPER;
use ownable_crate::OWNABLE;
#[derive(Default)]
struct CurveRewards(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for CurveRewards {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}
impl OWNABLE<OnChainContractStorage> for CurveRewards {}
impl LPTOKENWRAPPER<OnChainContractStorage> for CurveRewards {}
impl IREWARDDISTRIBUTIONRECIPIENT<OnChainContractStorage> for CurveRewards {}
impl CURVEREWARDS<OnChainContractStorage> for CurveRewards {}

impl CurveRewards {
    fn constructor(
        &mut self,
        token: Key,
        reward: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        CURVEREWARDS::init(self, token, reward, contract_hash, package_hash);
    }
}

#[no_mangle]
fn constructor() {
    let token: Key = runtime::get_named_arg("token");
    let reward: Key = runtime::get_named_arg("reward");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    CurveRewards::default().constructor(token, reward, contract_hash, package_hash);
}
#[no_mangle]
fn total_supply() {
    let ret: U256 = LPTOKENWRAPPER::total_supply(&CurveRewards::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn total_supply_js_client() {
    let ret: U256 = LPTOKENWRAPPER::total_supply(&CurveRewards::default());
    set_key("total_supply", ret);
}
#[no_mangle]
fn balance_of() {
    let account: Key = runtime::get_named_arg("account");
    let ret: U256 = LPTOKENWRAPPER::balance_of(&CurveRewards::default(), account);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn stake_lp() {
    let amount: U256 = runtime::get_named_arg("amount");
    LPTOKENWRAPPER::stake(&mut CurveRewards::default(), amount);
}
#[no_mangle]
fn withdraw_lp() {
    let amount: U256 = runtime::get_named_arg("amount");
    LPTOKENWRAPPER::withdraw(&mut CurveRewards::default(), amount);
}
#[no_mangle]
fn set_reward_distribution() {
    let reward_distribution: Key = runtime::get_named_arg("reward_distribution");
    IREWARDDISTRIBUTIONRECIPIENT::set_reward_distribution(
        &CurveRewards::default(),
        reward_distribution,
    );
}
#[no_mangle]
fn last_time_reward_applicable() {
    let ret: U256 = CurveRewards::default().last_time_reward_applicable();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn last_time_reward_applicable_js_client() {
    let ret: U256 = CurveRewards::default().last_time_reward_applicable();
    js_ret(ret);
}
#[no_mangle]
fn reward_per_token() {
    let ret: U256 = CurveRewards::default().reward_per_token();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn reward_per_token_js_client() {
    let ret: U256 = CurveRewards::default().reward_per_token();
    js_ret(ret);
}
#[no_mangle]
fn earned() {
    let account: Key = runtime::get_named_arg("account");
    let ret: U256 = CurveRewards::default().earned(account);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn earned_js_client() {
    let account: Key = runtime::get_named_arg("account");
    let ret: U256 = CurveRewards::default().earned(account);
    js_ret(ret);
}
#[no_mangle]
fn stake() {
    let amount: U256 = runtime::get_named_arg("amount");
    CURVEREWARDS::stake(&mut CurveRewards::default(), amount);
}
#[no_mangle]
fn withdraw() {
    let amount: U256 = runtime::get_named_arg("amount");
    CURVEREWARDS::withdraw(&mut CurveRewards::default(), amount);
}
#[no_mangle]
fn get_reward() {
    CurveRewards::default().get_reward();
}
#[no_mangle]
fn exit() {
    CurveRewards::default().exit();
}
#[no_mangle]
fn notify_reward_amount() {
    let reward: U256 = runtime::get_named_arg("reward");
    CurveRewards::default().notify_reward_amount(reward);
}
//Entry Points
fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("token", Key::cl_type()),
            Parameter::new("reward", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "last_time_reward_applicable",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_per_token",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "earned",
        vec![Parameter::new("account", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "last_time_reward_applicable_js_client",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_per_token_js_client",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "earned_js_client",
        vec![Parameter::new("account", Key::cl_type())],
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
        "get_reward",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "exit",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "notify_reward_amount",
        vec![Parameter::new("reward", U256::cl_type())],
        <()>::cl_type(),
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
        "total_supply_js_client",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("account", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "stake_lp",
        vec![Parameter::new("amount", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "withdraw_lp",
        vec![Parameter::new("amount", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_reward_distribution",
        vec![Parameter::new("reward_distribution", Key::cl_type())],
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
        let (contract_hash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());
        let token: Key = runtime::get_named_arg("token");
        let reward: Key = runtime::get_named_arg("reward");
        // Prepare constructor args
        let constructor_args = runtime_args! {
            "token" => token,
            "reward" => reward,
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
