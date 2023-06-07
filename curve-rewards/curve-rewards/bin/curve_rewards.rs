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
use curve_rewards_crate::{
    data, get_uni, Address, CURVEERC20, CURVEREWARDS, IREWARDDISTRIBUTIONRECIPIENT, LPTOKENWRAPPER,
    OWNABLE,
};

#[derive(Default)]
struct CurveRewards(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for CurveRewards {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}
impl OWNABLE<OnChainContractStorage> for CurveRewards {}
impl CURVEERC20<OnChainContractStorage> for CurveRewards {}
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

// This function is to return the total token supply
#[no_mangle]
fn total_supply() {
    runtime::ret(CLValue::from_t(CurveRewards::default().total_supply()).unwrap_or_revert());
}

// This function is to return the balance of the passed address
/// # Parameters
/// * `owner` - Address that holds the account address of the user
#[no_mangle]
fn balance_of() {
    let address: Address = runtime::get_named_arg("address");
    let ret: U256 = CurveRewards::default().balance_of(address);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// This function is used for staking the LP tokens according to passed amount
/// # Parameters
/// * `amount` - Amount of tokens to be staked
#[no_mangle]
fn stake_lp() {
    let amount: U256 = runtime::get_named_arg("amount");
    LPTOKENWRAPPER::stake(&mut CurveRewards::default(), amount);
}

// This function is used for withdrawing the LP tokens according to passed amount
/// # Parameters
/// * `amount` - Amount of tokens to be withdrawn
#[no_mangle]
fn withdraw_lp() {
    let amount: U256 = runtime::get_named_arg("amount");
    LPTOKENWRAPPER::withdraw(&mut CurveRewards::default(), amount);
}

// This function is to set the reward_distribution token address
/// # Parameters
/// * `reward_distribution` - reward_distribution deployed contract address for token distribution controlling
#[no_mangle]
fn set_reward_distribution() {
    let reward_distribution: Key = runtime::get_named_arg("reward_distribution");
    IREWARDDISTRIBUTIONRECIPIENT::set_reward_distribution(
        &CurveRewards::default(),
        reward_distribution,
    );
}

// This function is to get the last_time_reward_applicable token address
#[no_mangle]
fn last_time_reward_applicable() {
    let ret: U256 = CurveRewards::default().last_time_reward_applicable();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// This function is to get the reward_per_token token address
#[no_mangle]
fn reward_per_token() {
    let ret: U256 = CurveRewards::default().reward_per_token();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// This function is to get the calculated earnings of the account passed
/// # Parameters
/// * `account` - Address of user to check earnings against of
#[no_mangle]
fn earned() {
    let account: Key = runtime::get_named_arg("account");
    let ret: U256 = CurveRewards::default().earned(account);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// This function is to stake the curve rewards amount
/// # Parameters
/// * `amount` - Amount to be staked
#[no_mangle]
fn stake() {
    let amount: U256 = runtime::get_named_arg("amount");
    CURVEREWARDS::stake(&mut CurveRewards::default(), amount);
}

// This function is to withdraw the curve rewards amount
/// # Parameters
/// * `amount` - Amount to be withdrawn
#[no_mangle]
fn withdraw() {
    let amount: U256 = runtime::get_named_arg("amount");
    CURVEREWARDS::withdraw(&mut CurveRewards::default(), amount);
}

// This function is to get the present reward amount for the caller
#[no_mangle]
fn get_reward() {
    CurveRewards::default().get_reward();
}

// This function is to withdraw from the staking and get the present reward for the caller
#[no_mangle]
fn exit() {
    CurveRewards::default().exit();
}

// This function is to update the reward amount
// can only be called by the only_reward_distribution contract
/// # Parameters
/// * `reward` - Amount for the reward notification
#[no_mangle]
fn notify_reward_amount() {
    let reward: U256 = runtime::get_named_arg("reward");
    CurveRewards::default().notify_reward_amount(reward);
}

// This function is to return the owner of the contract
#[no_mangle]
fn owner() {
    let ret: Key = OWNABLE::owner(&CurveRewards::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// This function is to check if the caller is the owner or not
#[no_mangle]
fn is_owner() {
    let ret: bool = OWNABLE::is_owner(&CurveRewards::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// This function is to revoke the ownership from the current owner
// can be called by the owner only
#[no_mangle]
fn renounce_ownership() {
    OWNABLE::renounce_ownership(&mut CurveRewards::default());
}

// This function is to transfer the ownership from present owner to new one
/// # Parameters
/// * `new_owner` - New owner account address to transfer ownership
#[no_mangle]
fn transfer_ownership() {
    let new_owner: Key = runtime::get_named_arg("new_owner");
    OWNABLE::transfer_ownership(&mut CurveRewards::default(), new_owner);
}

// To get variables values //

#[no_mangle]
fn uni() {
    runtime::ret(CLValue::from_t(get_uni()).unwrap_or_revert());
}
#[no_mangle]
fn snx() {
    runtime::ret(CLValue::from_t(data::get_snx()).unwrap_or_revert());
}
#[no_mangle]
fn duration() {
    runtime::ret(CLValue::from_t(data::DURATION).unwrap_or_revert());
}
#[no_mangle]
fn period_finish() {
    runtime::ret(CLValue::from_t(data::get_period_finish()).unwrap_or_revert());
}
#[no_mangle]
fn reward_rate() {
    runtime::ret(CLValue::from_t(data::get_reward_rate()).unwrap_or_revert());
}
#[no_mangle]
fn last_update_time() {
    runtime::ret(CLValue::from_t(data::get_last_update_time()).unwrap_or_revert());
}
#[no_mangle]
fn reward_per_token_stored() {
    runtime::ret(CLValue::from_t(data::get_reward_per_token_stored()).unwrap_or_revert());
}
#[no_mangle]
fn user_reward_per_token_paid() {
    let account: Key = runtime::get_named_arg("account");
    runtime::ret(
        CLValue::from_t(data::UserRewardPerTokenPaid::instance().get(&account)).unwrap_or_revert(),
    );
}
#[no_mangle]
fn rewards() {
    let account: Key = runtime::get_named_arg("account");
    runtime::ret(CLValue::from_t(data::Rewards::instance().get(&account)).unwrap_or_revert());
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
        "balance_of",
        vec![Parameter::new("address", Address::cl_type())],
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
    entry_points.add_entry_point(EntryPoint::new(
        "renounce_ownership",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_ownership",
        vec![Parameter::new("new_owner", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    //Variables
    entry_points.add_entry_point(EntryPoint::new(
        "uni",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "snx",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "duration",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "period_finish",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_rate",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "last_update_time",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_per_token_stored",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "user_reward_per_token_paid",
        vec![Parameter::new("account", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "rewards",
        vec![Parameter::new("account", Key::cl_type())],
        U256::cl_type(),
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
            CurveRewards::default()
                .named_keys("Curve reward token".into(), "CRT".into(), 9, 0.into())
                .unwrap_or_revert(),
        );
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
