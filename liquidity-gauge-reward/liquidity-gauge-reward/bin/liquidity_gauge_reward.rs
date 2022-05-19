#![no_main]
#![no_std]
extern crate alloc;
use alloc::{collections::BTreeSet, format, string::String, vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U128,
    U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use liquidity_gauge_reward_crate::{self, data::*, LIQUIDITYGAUGEREWARD};

#[derive(Default)]
struct LiquidityGaugeReward(OnChainContractStorage);
impl ContractContext<OnChainContractStorage> for LiquidityGaugeReward {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl LIQUIDITYGAUGEREWARD<OnChainContractStorage> for LiquidityGaugeReward {}
impl LiquidityGaugeReward {
    fn constructor(
        &mut self,
        lp_addr: Key,
        minter: Key,
        reward_contract: Key,
        rewarded_token: Key,
        admin: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        LIQUIDITYGAUGEREWARD::init(
            self,
            lp_addr,
            minter,
            reward_contract,
            rewarded_token,
            admin,
            contract_hash,
            package_hash,
        );
    }
}

#[no_mangle]
fn constructor() {
    let lp_addr: Key = runtime::get_named_arg("lp_addr");
    let minter: Key = runtime::get_named_arg("minter");
    let reward_contract: Key = runtime::get_named_arg("reward_contract");
    let rewarded_token: Key = runtime::get_named_arg("rewarded_token");
    let admin: Key = runtime::get_named_arg("admin");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    LiquidityGaugeReward::default().constructor(
        lp_addr,
        minter,
        reward_contract,
        rewarded_token,
        admin,
        contract_hash,
        package_hash,
    );
}

#[no_mangle]
fn user_checkpoint() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: bool = LiquidityGaugeReward::default().user_checkpoint(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn user_checkpoint_js_client() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: bool = LiquidityGaugeReward::default().user_checkpoint(addr);
    js_ret(ret);
}

#[no_mangle]
fn claimable_tokens() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: U256 = LiquidityGaugeReward::default().claimable_tokens(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn claimable_tokens_js_client() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: U256 = LiquidityGaugeReward::default().claimable_tokens(addr);
    js_ret(ret);
}

#[no_mangle]
fn claimable_reward() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: U256 = LiquidityGaugeReward::default().claimable_reward(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn claimable_reward_js_client() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: U256 = LiquidityGaugeReward::default().claimable_reward(addr);
    js_ret(ret);
}

#[no_mangle]
fn kick() {
    let addr: Key = runtime::get_named_arg("addr");
    LiquidityGaugeReward::default().kick(addr);
}

#[no_mangle]
fn set_approve_deposit() {
    let addr: Key = runtime::get_named_arg("addr");
    let can_deposit: bool = runtime::get_named_arg("can_deposit");
    LiquidityGaugeReward::default().set_approve_deposit(addr, can_deposit);
}

#[no_mangle]
fn deposit() {
    let value: U256 = runtime::get_named_arg("value");
    let addr: Key = runtime::get_named_arg("addr");
    LiquidityGaugeReward::default().deposit(value, addr);
}

#[no_mangle]
fn withdraw() {
    let value: U256 = runtime::get_named_arg("value");
    let claim_rewards: bool = runtime::get_named_arg("claim_rewards");
    LiquidityGaugeReward::default().withdraw(value, claim_rewards);
}

#[no_mangle]
fn claim_rewards() {
    let addr: Key = runtime::get_named_arg("addr");
    LiquidityGaugeReward::default().claim_rewards(addr);
}

#[no_mangle]
fn integrate_checkpoint() {
    let ret: U256 = LiquidityGaugeReward::default().integrate_checkpoint();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn integrate_checkpoint_js_client() {
    let ret: U256 = LiquidityGaugeReward::default().integrate_checkpoint();
    js_ret(ret);
}

#[no_mangle]
fn kill_me() {
    LiquidityGaugeReward::default().kill_me();
}

#[no_mangle]
fn commit_transfer_ownership() {
    let addr: Key = runtime::get_named_arg("addr");
    LiquidityGaugeReward::default().commit_transfer_ownership(addr);
}

#[no_mangle]
fn apply_transfer_ownership() {
    LiquidityGaugeReward::default().apply_transfer_ownership();
}

#[no_mangle]
fn toggle_external_rewards_claim() {
    let val: bool = runtime::get_named_arg("val");
    LiquidityGaugeReward::default().toggle_external_rewards_claim(val);
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("lp_addr", Key::cl_type()),
            Parameter::new("minter", Key::cl_type()),
            Parameter::new("reward_contract", Key::cl_type()),
            Parameter::new("rewarded_token", Key::cl_type()),
            Parameter::new("admin", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "user_checkpoint",
        vec![Parameter::new("addr", Key::cl_type())],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "user_checkpoint_js_client",
        vec![Parameter::new("addr", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claimable_tokens",
        vec![Parameter::new("addr", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claimable_tokens_js_client",
        vec![Parameter::new("addr", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claimable_reward",
        vec![Parameter::new("addr", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claimable_reward_js_client",
        vec![Parameter::new("addr", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "kick",
        vec![Parameter::new("addr", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_approve_deposit",
        vec![
            Parameter::new("addr", Key::cl_type()),
            Parameter::new("can_deposit", bool::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "deposit",
        vec![
            Parameter::new("value", U256::cl_type()),
            Parameter::new("addr", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "withdraw",
        vec![
            Parameter::new("value", U256::cl_type()),
            Parameter::new("claim_rewards", bool::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claim_rewards",
        vec![Parameter::new("addr", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "integrate_checkpoint",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "integrate_checkpoint_js_client",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "kill_me",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
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
        "toggle_external_rewards_claim",
        vec![Parameter::new("val", bool::cl_type())],
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

        let lp_addr: Key = runtime::get_named_arg("lp_addr");
        let minter: Key = runtime::get_named_arg("minter");
        let reward_contract: Key = runtime::get_named_arg("reward_contract");
        let rewarded_token: Key = runtime::get_named_arg("rewarded_token");
        let admin: Key = runtime::get_named_arg("admin");
        let constructor_args = runtime_args! {
            "lp_addr" => lp_addr,
            "minter" => minter,
            "reward_contract" => reward_contract,
            "rewarded_token" => rewarded_token,
            "admin" => admin,
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