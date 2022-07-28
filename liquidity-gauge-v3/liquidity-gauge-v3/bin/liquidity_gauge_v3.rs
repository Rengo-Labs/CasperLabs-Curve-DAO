#![no_main]
#![no_std]
extern crate alloc;
use crate::vec::Vec;
use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::Bytes, runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash,
    EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs,
    URef, U128, U256,
};

use casperlabs_contract_utils::{ContractContext, OnChainContractStorage};
use liquidity_gauge_v3_crate::{self, data, LIQUIDITYTGAUGEV3};

#[derive(Default)]
struct LiquidityGaugeV3(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for LiquidityGaugeV3 {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl LIQUIDITYTGAUGEV3<OnChainContractStorage> for LiquidityGaugeV3 {}

impl LiquidityGaugeV3 {
    fn constructor(
        &mut self,
        lp_addr: Key,
        minter: Key,
        admin: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        LIQUIDITYTGAUGEV3::init(self, lp_addr, minter, admin, contract_hash, package_hash);
    }
}
#[no_mangle]
fn constructor() {
    let lp_addr: Key = runtime::get_named_arg("lp_addr");
    let minter: Key = runtime::get_named_arg("minter");
    let admin: Key = runtime::get_named_arg("admin");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    LiquidityGaugeV3::default().constructor(lp_addr, minter, admin, contract_hash, package_hash);
}
/// @notice Get the number of decimals for this token
/// @dev Implemented as a view method to reduce gas costs
/// @return uint256 decimal places
#[no_mangle]
fn decimals() {
    let ret: u8 = LiquidityGaugeV3::default().decimals();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn integrate_checkpoint() {
    let ret: U256 = LiquidityGaugeV3::default().integrate_checkpoint();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

///"""
///    @notice Record a checkpoint for `addr`
///    @param addr User address
///    @return bool success
///"""
#[no_mangle]
fn user_checkpoint() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: bool = LiquidityGaugeV3::default().user_checkpoint(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
///    @notice Get the number of claimable tokens per user
///    @return uint256 number of claimable tokens per user
#[no_mangle]
fn claimable_tokens() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: U256 = LiquidityGaugeV3::default().claimable_tokens(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
/// @notice Address of the reward contract providing non-CRV incentives for this gauge
/// @dev Returns `ZERO_ADDRESS` if there is no reward contract active
#[no_mangle]
fn reward_contract() {
    let ret = LiquidityGaugeV3::default().reward_contract();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
/// @notice Epoch timestamp of the last call to claim from `reward_contract`
/// @dev Rewards are claimed at most once per hour in order to reduce gas costs
#[no_mangle]
fn last_claim() {
    let ret = LiquidityGaugeV3::default().last_claim();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
/// @notice Get the number of already-claimed reward tokens for a user
/// @param _addr Account to get reward amount for
/// @param _token Token to get reward amount for
/// @return uint256 Total amount of `_token` already claimed by `_addr`
#[no_mangle]
fn claimed_reward() {
    let addr: Key = runtime::get_named_arg("addr");
    let token: Key = runtime::get_named_arg("token");
    let ret = LiquidityGaugeV3::default().claimed_reward(addr, token);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
/// @notice Get the number of claimable reward tokens for a user
/// @dev This call does not consider pending claimable amount in `reward_contract`.
///      Off-chain callers should instead use `claimable_rewards_write` as a
///      view method.
/// @param _addr Account to get reward amount for
/// @param _token Token to get reward amount for
/// @return uint256 Claimable reward token amount

#[no_mangle]
fn claimable_reward() {
    let addr: Key = runtime::get_named_arg("addr");
    let token: Key = runtime::get_named_arg("token");

    let ret: U256 = LiquidityGaugeV3::default().claimable_reward(addr, token);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
/// @notice Get the number of claimable reward tokens for a user
/// @dev This function should be manually changed to "view" in the ABI
///      Calling it via a transaction will claim available reward tokens
/// @param _addr Account to get reward amount for
/// @param _token Token to get reward amount for
/// @return uint256 Claimable reward token amount

#[no_mangle]
fn claimable_reward_write() {
    let addr: Key = runtime::get_named_arg("addr");
    let token: Key = runtime::get_named_arg("token");
    let ret: U256 = LiquidityGaugeV3::default().claimable_reward_write(addr, token);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Set the default reward receiver for the caller.
/// @dev When set to ZERO_ADDRESS, rewards are sent to the caller
/// @param _receiver Receiver address for any rewards claimed via `claim_rewards`

#[no_mangle]
fn set_rewards_receiver() {
    let receiver: Key = runtime::get_named_arg("receiver");
    LiquidityGaugeV3::default().set_rewards_receiver(receiver);
}
///    @notice Kick `addr` for abusing their boost
///    @dev Only if either they had another voting event, or their voting escrow lock expired
///    @param addr Address to kick
#[no_mangle]
fn kick() {
    let addr: Key = runtime::get_named_arg("addr");
    LiquidityGaugeV3::default().kick(addr);
}
///    @notice Claim available reward tokens for `addr`
///    @param addr Address to claim for
///    @param receiver Address to transfer rewards to - if set to
///                     ZERO_ADDRESS, uses the default reward receiver
///                     for the caller

#[no_mangle]
fn claim_rewards() {
    let addr: Option<Key> = runtime::get_named_arg("addr");
    let receiver: Option<Key> = runtime::get_named_arg("receiver");
    LiquidityGaugeV3::default().claim_rewards(addr, receiver);
}
/// @notice Deposit `_value` LP tokens
/// @dev Depositting also claims pending reward tokens
/// @param _value Number of tokens to deposit
/// @param _addr Address to deposit for

#[no_mangle]
fn deposit() {
    let value: U256 = runtime::get_named_arg("value");
    let addr: Option<Key> = runtime::get_named_arg("addr");
    let claim_rewards: Option<bool> = runtime::get_named_arg("claim_rewards");
    LiquidityGaugeV3::default().deposit(value, addr, claim_rewards);
}

/// @notice Withdraw `value` LP tokens
/// @dev Withdrawing also claims pending reward tokens
/// @param _value Number of tokens to withdraw

#[no_mangle]
fn withdraw() {
    let value: U256 = runtime::get_named_arg("value");
    let claim_rewards: Option<bool> = runtime::get_named_arg("claim_rewards");

    LiquidityGaugeV3::default().withdraw(value, claim_rewards);
}

/// @notice Transfer token for a specified address
/// @dev Transferring claims pending reward tokens for the sender and receiver
/// @param _to The address to transfer to.
/// @param _value The amount to be transferred.

#[no_mangle]
fn transfer() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret: Result<(), u32> = LiquidityGaugeV3::default().transfer(recipient, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Transfer tokens from one address to another.
/// @dev Transferring claims pending reward tokens for the sender and receiver
/// @param _from address The address which you want to send tokens from
/// @param _to address The address which you want to transfer to
/// @param _value uint256 the amount of tokens to be transferred

#[no_mangle]
fn transfer_from() {
    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret: Result<(), u32> = LiquidityGaugeV3::default().transfer_from(owner, recipient, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Approve the passed address to transfer the specified amount of
///            tokens on behalf of self.get_caller
///    @dev Beware that changing an allowance via this method brings the risk
///         that someone may use both the old and new allowance by unfortunate
///         transaction ordering. This may be mitigated with the use of
///         {increase_allowance} and {decrease_allowance}.
///    @param spender The address which will transfer the funds
///    @param amount The amount of tokens that may be transferred
#[no_mangle]
fn approve() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    LiquidityGaugeV3::default().approve(spender, amount);
}
///@notice Increase the allowance granted to `spender` by the caller
///    @dev This is alternative to {approve} that can be used as a mitigation for
///         the potential race condition
///    @param spender The address which will transfer the funds
///    @param added_value The amount of to increase the allowance
///   @return ok success
#[no_mangle]
fn increase_allowance() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret = LiquidityGaugeV3::default().increase_allowance(spender, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

///@notice Decrease the allowance granted to `spender` by the caller
///    @dev This is alternative to {approve} that can be used as a mitigation for
///         the potential race condition
///    @param spender The address which will transfer the funds
///    @param amount The amount of to decrease the allowance
///    @return ok success
#[no_mangle]
fn decrease_allowance() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret = LiquidityGaugeV3::default().decrease_allowance(spender, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Set the active reward contract
/// @dev A reward contract cannot be set while this contract has no deposits
/// @param _reward_contract Reward contract address. Set to ZERO_ADDRESS to
///                         disable staking.
/// @param _claim_sig Four byte selectors for staking, withdrawing and claiming,
///             left padded with zero bytes. If the reward contract can
///             be claimed from but does not require staking, the staking
///             and withdraw selectors should be set to 0x00
/// @param _reward_tokens List of claimable reward tokens. New reward tokens
///                     may be added but they cannot be removed. When calling
///                     this function to unset or modify a reward contract,
///                     this array must begin with the already-set reward
///                     token addresses.

#[no_mangle]
fn set_rewards() {
    let reward_contract: Key = runtime::get_named_arg("reward_contract");
    let claim_sig: Bytes = runtime::get_named_arg("claim_sig");
    let reward_tokens: Vec<String> = runtime::get_named_arg("reward_tokens");

    LiquidityGaugeV3::default().set_rewards(reward_contract, claim_sig, reward_tokens);
}
///    @notice Set the killed status for this contract
///    @dev When killed, the gauge always yields a rate of 0 and so cannot mint CRV
///    @param _is_killed Killed status to set
#[no_mangle]
fn set_killed() {
    let is_killed: bool = runtime::get_named_arg("is_killed");
    LiquidityGaugeV3::default().set_killed(is_killed);
}

/// @notice Transfer ownership of GaugeController to `addr`
/// @param addr Address to have ownership transferred to

#[no_mangle]
fn commit_transfer_ownership() {
    let addr: Key = runtime::get_named_arg("addr");
    LiquidityGaugeV3::default().commit_transfer_ownership(addr);
}

/// @notice Accept a pending ownership transfer

#[no_mangle]
fn accept_transfer_ownership() {
    LiquidityGaugeV3::default().accept_transfer_ownership();
}

// public Variables
#[no_mangle]
fn minter() {
    runtime::ret(CLValue::from_t(data::get_minter()).unwrap_or_revert());
}
#[no_mangle]
fn crv_token() {
    runtime::ret(CLValue::from_t(data::get_crv_token()).unwrap_or_revert());
}
#[no_mangle]
fn lp_token() {
    runtime::ret(CLValue::from_t(data::get_lp_token()).unwrap_or_revert());
}
#[no_mangle]
fn controller() {
    runtime::ret(CLValue::from_t(data::get_controller()).unwrap_or_revert());
}
#[no_mangle]
fn voting_escrow() {
    runtime::ret(CLValue::from_t(data::get_voting_escrow()).unwrap_or_revert());
}
#[no_mangle]
fn future_epoch_time() {
    runtime::ret(CLValue::from_t(data::get_future_epoch_time()).unwrap_or_revert());
}
#[no_mangle]
fn balance_of() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = LiquidityGaugeV3::default().balance_of(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn total_supply() {
    let ret: U256 = LiquidityGaugeV3::default().total_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn allowance() {
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    runtime::ret(
        CLValue::from_t(data::Allowance::instance().get(&owner, &spender)).unwrap_or_revert(),
    );
}
#[no_mangle]
fn name() {
    runtime::ret(CLValue::from_t(data::get_name()).unwrap_or_revert());
}
#[no_mangle]
fn symbol() {
    runtime::ret(CLValue::from_t(data::get_symbol()).unwrap_or_revert());
}
#[no_mangle]
fn working_balances() {
    let owner: Key = runtime::get_named_arg("owner");
    runtime::ret(CLValue::from_t(data::WorkingBalances::instance().get(&owner)).unwrap_or_revert());
}
#[no_mangle]
fn working_supply() {
    runtime::ret(CLValue::from_t(data::get_working_supply()).unwrap_or_revert());
}
#[no_mangle]
fn period() {
    runtime::ret(CLValue::from_t(data::get_period()).unwrap_or_revert());
}
#[no_mangle]
fn period_timestamp() {
    let owner: U256 = runtime::get_named_arg("owner");
    runtime::ret(CLValue::from_t(data::PeriodTimestamp::instance().get(&owner)).unwrap_or_revert());
}
#[no_mangle]
fn integrate_inv_supply() {
    let owner: U256 = runtime::get_named_arg("owner");
    runtime::ret(
        CLValue::from_t(data::IntegrateInvSupply::instance().get(&owner)).unwrap_or_revert(),
    );
}
#[no_mangle]
fn integrate_inv_supply_of() {
    let owner: Key = runtime::get_named_arg("owner");
    runtime::ret(
        CLValue::from_t(data::IntegrateInvSupplyOf::instance().get(&owner)).unwrap_or_revert(),
    );
}
#[no_mangle]
fn integrate_checkpoint_of() {
    let owner: Key = runtime::get_named_arg("owner");
    runtime::ret(
        CLValue::from_t(data::IntegrateCheckpointOf::instance().get(&owner)).unwrap_or_revert(),
    );
}
#[no_mangle]
fn integrate_fraction() {
    let owner: Key = runtime::get_named_arg("owner");
    runtime::ret(
        CLValue::from_t(data::IntegrateFraction::instance().get(&owner)).unwrap_or_revert(),
    );
}
#[no_mangle]
fn inflation_rate() {
    runtime::ret(CLValue::from_t(data::get_inflation_rate()).unwrap_or_revert());
}
#[no_mangle]
fn reward_tokens() {
    let owner: U256 = runtime::get_named_arg("owner");
    runtime::ret(CLValue::from_t(data::RewardTokens::instance().get(&owner)).unwrap_or_revert());
}
#[no_mangle]
fn rewards_receiver() {
    let owner: Key = runtime::get_named_arg("owner");
    runtime::ret(CLValue::from_t(data::RewardsReceiver::instance().get(&owner)).unwrap_or_revert());
}
#[no_mangle]
fn reward_integral() {
    let owner: Key = runtime::get_named_arg("owner");
    runtime::ret(CLValue::from_t(data::RewardIntegral::instance().get(&owner)).unwrap_or_revert());
}
#[no_mangle]
fn reward_integral_for() {
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    runtime::ret(
        CLValue::from_t(data::RewardIntegralFor::instance().get(&owner, &spender))
            .unwrap_or_revert(),
    );
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
fn is_killed() {
    runtime::ret(CLValue::from_t(data::get_is_killed()).unwrap_or_revert());
}
fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("lp_addr", Key::cl_type()),
            Parameter::new("minter", Key::cl_type()),
            Parameter::new("admin", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "decimals",
        vec![],
        u8::cl_type(),
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
        "user_checkpoint",
        vec![Parameter::new("addr", Key::cl_type())],
        bool::cl_type(),
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
        "reward_contract",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "last_claim",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claimed_reward",
        vec![
            Parameter::new("addr", Key::cl_type()),
            Parameter::new("token", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claimable_reward",
        vec![
            Parameter::new("addr", Key::cl_type()),
            Parameter::new("token", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claimable_reward_write",
        vec![
            Parameter::new("addr", Key::cl_type()),
            Parameter::new("token", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_rewards_receiver",
        vec![Parameter::new("receiver", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claim_rewards",
        vec![
            Parameter::new("addr", CLType::Option(Box::new(CLType::Key))),
            Parameter::new("receiver", CLType::Option(Box::new(CLType::Key))),
        ],
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
        "deposit",
        vec![
            Parameter::new("value", U256::cl_type()),
            Parameter::new("addr", CLType::Option(Box::new(CLType::Key))),
            Parameter::new("claim_rewards", CLType::Option(Box::new(bool::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "withdraw",
        vec![
            Parameter::new("value", U256::cl_type()),
            Parameter::new("claim_rewards", CLType::Option(Box::new(bool::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "increase_allowance",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "decrease_allowance",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_rewards",
        vec![
            Parameter::new("reward_contract", Key::cl_type()),
            Parameter::new("claim_sig", Bytes::cl_type()),
            Parameter::new("reward_tokens", CLType::List(Box::new(String::cl_type()))),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_killed",
        vec![Parameter::new("is_killed", bool::cl_type())],
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
        "accept_transfer_ownership",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    //entry points of public variables
    entry_points.add_entry_point(EntryPoint::new(
        "minter",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "crv_token",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "lp_token",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "controller",
        vec![],
        Key::cl_type(),
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
        "future_epoch_time",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
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
        "allowance",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "name",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "symbol",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "working_balances",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "working_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "period",
        vec![],
        U128::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "period_timestamp",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "integrate_inv_supply",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "integrate_inv_supply_of",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "integrate_checkpoint_of",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "integrate_fraction",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "inflation_rate",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_tokens",
        vec![Parameter::new("owner", Key::cl_type())],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "rewards_receiver",
        vec![Parameter::new("owner", Key::cl_type())],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_integral",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_integral_for",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
        ],
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
    entry_points.add_entry_point(EntryPoint::new(
        "is_killed",
        vec![],
        bool::cl_type(),
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
        // Build new package with initial a first version of the contract.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        let (contract_hash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());
        // Variables
        let lp_addr: Key = runtime::get_named_arg("lp_addr");
        let minter: Key = runtime::get_named_arg("minter");
        let admin: Key = runtime::get_named_arg("admin");
        // Call the constructor entry point
        let constructor_args = runtime_args! {
            "lp_addr" => lp_addr,
            "minter" => minter,
            "admin" => admin,
            "contract_hash" => contract_hash,
            "package_hash" => package_hash,
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
