use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::bytesrepr::Bytes;
use casper_types::{ContractHash, ContractPackageHash, Key, U256};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use casperlabs_contract_utils::{get_key, key_to_str, set_key, Dict};
use common::{errors::*, keys::*, utils::*};

pub const MAX_REWARDS: U256 = U256([8, 0, 0, 0]);
pub const TOKENLESS_PRODUCTION: U256 = U256([40, 0, 0, 0]);
pub const CLAIM_FREQUENCY: U256 = U256([3600000, 0, 0, 0]);
pub const WEEK: U256 = U256([604800000, 0, 0, 0]);

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct ClaimDataStruct {
    pub claimable_amount: U256,
    pub claimed_amount: U256,
}

pub struct ClaimData {
    dict: Dict,
}

impl ClaimData {
    pub fn instance() -> ClaimData {
        ClaimData {
            dict: Dict::instance(CLAIM_DATA_DICT),
        }
    }

    pub fn init() {
        Dict::init(CLAIM_DATA_DICT)
    }

    pub fn get(&self, user: &Key, claiming_address: &Key) -> ClaimDataStruct {
        ClaimDataStruct {
            claimable_amount: self
                .dict
                .get(
                    hash(format!(
                        "{}{}{}{}{}",
                        CLAIM_DATA_DICT,
                        "_claimable_amount_",
                        user.to_formatted_string(),
                        "_",
                        claiming_address.to_formatted_string()
                    ))
                    .as_str(),
                )
                .unwrap_or_default(),

            claimed_amount: self
                .dict
                .get(
                    hash(format!(
                        "{}{}{}{}{}",
                        CLAIM_DATA_DICT,
                        "_claimed_amount_",
                        user.to_formatted_string(),
                        "_",
                        claiming_address.to_formatted_string()
                    ))
                    .as_str(),
                )
                .unwrap_or_default(),
        }
    }

    pub fn set(&self, user: &Key, claiming_address: &Key, value: ClaimDataStruct) {
        self.dict.set(
            hash(format!(
                "{}{}{}{}{}",
                CLAIM_DATA_DICT,
                "_claimable_amount_",
                user.to_formatted_string(),
                "_",
                claiming_address.to_formatted_string()
            ))
            .as_str(),
            value.claimable_amount,
        );

        self.dict.set(
            hash(format!(
                "{}{}{}{}{}",
                CLAIM_DATA_DICT,
                "_claimed_amount_",
                user.to_formatted_string(),
                "_",
                claiming_address.to_formatted_string()
            ))
            .as_str(),
            value.claimed_amount,
        );
    }
}
pub const REWARD_TOKENS: &str = "reward_tokens";
pub struct RewardTokens {
    dict: Dict,
    length: U256,
}

impl RewardTokens {
    pub fn instance() -> RewardTokens {
        RewardTokens {
            dict: Dict::instance(REWARD_TOKENS),
            length: 0.into(),
        }
    }

    pub fn init() {
        Dict::init(REWARD_TOKENS)
    }

    pub fn get(&self, indx: &U256) -> Key {
        self.dict
            .get(indx.to_string().as_str())
            .unwrap_or_else(zero_address)
    }

    pub fn set(&self, indx: &U256, value: Key) {
        self.dict.set(indx.to_string().as_str(), value);
    }

    pub fn push(&mut self, value: U256) {
        self.dict.set(self.length.to_string().as_str(), value);
        self.length = self
            .length
            .checked_add(1.into())
            .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError1);
    }
}

pub const REWARDS_RECEIVER: &str = "reward_reciever";

pub struct RewardsReceiver {
    dict: Dict,
}

impl RewardsReceiver {
    pub fn instance() -> RewardsReceiver {
        RewardsReceiver {
            dict: Dict::instance(REWARDS_RECEIVER),
        }
    }

    pub fn init() {
        Dict::init(REWARDS_RECEIVER)
    }

    pub fn get(&self, owner: &Key) -> Key {
        self.dict
            .get(&key_to_str(owner))
            .unwrap_or_else(zero_address)
    }

    pub fn set(&self, owner: &Key, value: Key) {
        self.dict.set(&key_to_str(owner), value);
    }
}

const WORKING_BALANCES: &str = "working_balances";
pub struct WorkingBalances {
    dict: Dict,
}

impl WorkingBalances {
    pub fn instance() -> WorkingBalances {
        WorkingBalances {
            dict: Dict::instance(WORKING_BALANCES),
        }
    }

    pub fn init() {
        Dict::init(WORKING_BALANCES)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

const PERIOD_TIMESTAMP: &str = "period_timestamp";
pub struct PeriodTimestamp {
    dict: Dict,
}

impl PeriodTimestamp {
    pub fn instance() -> PeriodTimestamp {
        PeriodTimestamp {
            dict: Dict::instance(PERIOD_TIMESTAMP),
        }
    }

    pub fn init() {
        Dict::init(PERIOD_TIMESTAMP)
    }

    pub fn get(&self, key: &U256) -> U256 {
        self.dict.get(key.to_string().as_str()).unwrap_or_default()
    }

    pub fn set(&self, key: &U256, value: U256) {
        self.dict.set(key.to_string().as_str(), value);
    }
}

const INTEGRATE_INV_SUPPLY: &str = "integrate_inv_supply";
pub struct IntegrateInvSupply {
    dict: Dict,
}

impl IntegrateInvSupply {
    pub fn instance() -> IntegrateInvSupply {
        IntegrateInvSupply {
            dict: Dict::instance(INTEGRATE_INV_SUPPLY),
        }
    }

    pub fn init() {
        Dict::init(INTEGRATE_INV_SUPPLY)
    }

    pub fn get(&self, key: &U256) -> U256 {
        self.dict.get(key.to_string().as_str()).unwrap_or_default()
    }

    pub fn set(&self, key: &U256, value: U256) {
        self.dict.set(key.to_string().as_str(), value);
    }
}

const INTEGRATE_INV_SUPPLY_OF: &str = "integrate_inv_supply_of";
pub struct IntegrateInvSupplyOf {
    dict: Dict,
}

impl IntegrateInvSupplyOf {
    pub fn instance() -> IntegrateInvSupplyOf {
        IntegrateInvSupplyOf {
            dict: Dict::instance(INTEGRATE_INV_SUPPLY_OF),
        }
    }

    pub fn init() {
        Dict::init(INTEGRATE_INV_SUPPLY_OF)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

const INTEGRATE_CHECKPOINT_OF: &str = "integrate_checkpoint_of";
pub struct IntegrateCheckpointOf {
    dict: Dict,
}

impl IntegrateCheckpointOf {
    pub fn instance() -> IntegrateCheckpointOf {
        IntegrateCheckpointOf {
            dict: Dict::instance(INTEGRATE_CHECKPOINT_OF),
        }
    }

    pub fn init() {
        Dict::init(INTEGRATE_CHECKPOINT_OF)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

const INTEGRATE_FRACTION: &str = "integrate_fraction";
pub struct IntegrateFraction {
    dict: Dict,
}

impl IntegrateFraction {
    pub fn instance() -> IntegrateFraction {
        IntegrateFraction {
            dict: Dict::instance(INTEGRATE_FRACTION),
        }
    }

    pub fn init() {
        Dict::init(INTEGRATE_FRACTION)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}
const REWARD_INTEGRAL: &str = "reward_integral";
pub struct RewardIntegral {
    dict: Dict,
}

impl RewardIntegral {
    pub fn instance() -> RewardIntegral {
        RewardIntegral {
            dict: Dict::instance(REWARD_INTEGRAL),
        }
    }

    pub fn init() {
        Dict::init(REWARD_INTEGRAL)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}
const REWARD_INTEGRAL_FOR: &str = "reward_integral_for";
pub struct RewardIntegralFor {
    dict: Dict,
}

impl RewardIntegralFor {
    pub fn instance() -> RewardIntegralFor {
        RewardIntegralFor {
            dict: Dict::instance(REWARD_INTEGRAL_FOR),
        }
    }

    pub fn init() {
        Dict::init(REWARD_INTEGRAL_FOR)
    }

    pub fn get(&self, key1: &Key, key2: &Key) -> U256 {
        self.dict.get_by_keys((key1, key2)).unwrap_or_default()
    }

    pub fn set(&self, key1: &Key, key2: &Key, value: U256) {
        self.dict.set_by_keys((key1, key2), value);
    }
}
pub fn reward_sigs() -> Bytes {
    get_key(REWARD_SIGS).unwrap_or_revert()
}

pub fn set_reward_sigs(reward_sigs: Bytes) {
    set_key(REWARD_SIGS, reward_sigs);
}
pub fn myvec() -> Vec<Key> {
    get_key(MYVEC).unwrap_or_revert()
}
pub fn set_myvec(myvec: Vec<Key>) {
    set_key(MYVEC, myvec);
}

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes)]
pub struct RewardDataStruct {
    pub token: Key,
    pub distributor: Key,
    pub period_finish: u64,
    pub rate: U256,
    pub last_update: u64,
    pub integral: U256
}

pub struct RewardData {
    dict: Dict,
}

impl RewardData {
    pub fn instance() -> RewardData {
        RewardData {
            dict: Dict::instance(REWARD_DATA_DICT),
        }
    }

    pub fn init() {
        Dict::init(REWARD_DATA_DICT)
    }

    pub fn get(&self, reward_token_address: &Key) -> RewardDataStruct {
      RewardDataStruct {
          /*token: self
              .dict
                .get(
                    hash(format!(
                        "{}{}{}",
                        REWARD_DATA_DICT,
                        "_token_",
                        reward_token_address.to_formatted_string(),
                    ))
                    .as_str(),
                )
                .unwrap_or_default(),*/

          distributor: self
              .dict
                .get(
                    hash(format!(
                        "{}{}{}",
                        REWARD_DATA_DICT,
                        "_distributor_",
                        reward_token_address.to_formatted_string(),
                    ))
                    .as_str(),
                )
                .unwrap_or_default(),

          period_finish: self
              .dict
                .get(
                    hash(format!(
                        "{}{}{}",
                        REWARD_DATA_DICT,
                        "_period_finish_",
                        reward_token_address.to_formatted_string(),
                    ))
                    .as_str(),
                )
                .unwrap_or_default(),

          period_finish: self
              .dict
                .get(
                    hash(format!(
                        "{}{}{}",
                        REWARD_DATA_DICT,
                        "_period_finish_",
                        reward_token_address.to_formatted_string(),
                    ))
                    .as_str(),
                )
                .unwrap_or_default(),

          rate: self
                .dict
                  .get(
                      hash(format!(
                          "{}{}{}",
                          REWARD_DATA_DICT,
                          "_rate_",
                          reward_token_address.to_formatted_string(),
                      ))
                      .as_str(),
                  )
                  .unwrap_or_default(),

          last_update: self
                .dict
                  .get(
                      hash(format!(
                          "{}{}{}",
                          REWARD_DATA_DICT,
                          "_last_update_",
                          reward_token_address.to_formatted_string(),
                      ))
                      .as_str(),
                  )
                  .unwrap_or_default(),

          integral: self
                .dict
                  .get(
                      hash(format!(
                          "{}{}{}",
                          REWARD_DATA_DICT,
                          "_integral_",
                          reward_token_address.to_formatted_string(),
                      ))
                      .as_str(),
                  )
                  .unwrap_or_default(),
        }
    }

    pub fn set(&self, reward_token_address: &Key, value: RewardDataStruct) {
        /*self.dict.set(
            hash(format!(
                "{}{}{}",
                REWARD_DATA_DICT,
                "_token_",
                reward_token_address.to_formatted_string(),
            ))
            .as_str(),
            value.token,
        );*/
    
        self.dict.set(
            hash(format!(
                "{}{}{}",
                REWARD_DATA_DICT,
                "_distributor_",
                reward_token_address.to_formatted_string(),
            ))
            .as_str(),
            value.distributor,
        );
    
        self.dict.set(
            hash(format!(
                "{}{}{}",
                REWARD_DATA_DICT,
                "_period_finish_",
                reward_token_address.to_formatted_string(),
            ))
            .as_str(),
            value.period_finish,
        );
    
        self.dict.set(
            hash(format!(
                "{}{}{}",
                REWARD_DATA_DICT,
                "_rate_",
                reward_token_address.to_formatted_string(),
            ))
            .as_str(),
            value.rate,
        );
    
        self.dict.set(
            hash(format!(
                "{}{}{}",
                REWARD_DATA_DICT,
                "_last_update_",
                reward_token_address.to_formatted_string(),
            ))
            .as_str(),
            value.last_update,
        );
    
        self.dict.set(
            hash(format!(
                "{}{}{}",
                REWARD_DATA_DICT,
                "_integral_",
                reward_token_address.to_formatted_string(),
            ))
            .as_str(),
            value.integral,
        );
    }
}

pub fn set_minter(minter: Key) {
    set_key(MINTER, minter);
}

pub fn get_minter() -> Key {
    get_key(MINTER).unwrap_or_else(zero_address)
}

pub fn set_crv_token(crv_token: Key) {
    set_key(CRV_TOKEN, crv_token);
}

pub fn get_crv_token() -> Key {
    get_key(CRV_TOKEN).unwrap_or_else(zero_address)
}

pub fn set_lp_token(lp_token: Key) {
    set_key(LP_TOKEN, lp_token);
}

pub fn get_lp_token() -> Key {
    get_key(LP_TOKEN).unwrap_or_else(zero_address)
}

pub fn set_reward_count(reward_count: U256) {
    set_key(REWARD_COUNT, reward_count);
}

pub fn get_reward_count() -> U256 {
    get_key(REWARD_COUNT).unwrap_or_else()
}

pub fn set_controller(controller: Key) {
    set_key(CONTROLLER, controller);
}

pub fn get_controller() -> Key {
    get_key(CONTROLLER).unwrap_or_else(zero_address)
}

pub fn set_voting_escrow(voting_escrow: Key) {
    set_key(VOTING_ESCROW, voting_escrow);
}

pub fn get_voting_escrow() -> Key {
    get_key(VOTING_ESCROW).unwrap_or_else(zero_address)
}

pub fn set_future_epoch_time(future_epoch_time: U256) {
    set_key(FUTURE_EPOCH_TIME, future_epoch_time);
}

pub fn get_future_epoch_time() -> U256 {
    get_key(FUTURE_EPOCH_TIME).unwrap_or_default()
}
pub fn set_working_supply(working_supply: U256) {
    set_key(WORKING_SUPPLY, working_supply);
}

pub fn get_working_supply() -> U256 {
    get_key(WORKING_SUPPLY).unwrap_or_default()
}

pub fn set_period(period: i128) {
    set_key(PERIOD, i128_to_tuple(period));
}

pub fn get_period() -> i128 {
    tuple_to_i128(get_key(PERIOD).unwrap_or_default())
}

pub fn set_inflation_rate(inflation_rate: U256) {
    set_key(INFLATION_RATE, inflation_rate);
}

pub fn get_inflation_rate() -> U256 {
    get_key(INFLATION_RATE).unwrap_or_default()
}

pub fn set_admin(admin: Key) {
    set_key(ADMIN, admin);
}

pub fn get_admin() -> Key {
    get_key(ADMIN).unwrap_or_else(zero_address)
}

pub fn set_future_admin(future_admin: Key) {
    set_key(FUTURE_ADMIN, future_admin);
}

pub fn get_future_admin() -> Key {
    get_key(FUTURE_ADMIN).unwrap_or_else(zero_address)
}

pub fn set_is_killed(is_killed: bool) {
    set_key(IS_KILLED, is_killed);
}

pub fn get_is_killed() -> bool {
    get_key(IS_KILLED).unwrap_or_default()
}

pub fn set_contract_hash(contract_hash: ContractHash) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_contract_hash() -> ContractHash {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(SELF_CONTRACT_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(SELF_CONTRACT_PACKAGE_HASH).unwrap_or_revert()
}

pub fn set_lock(lock: bool) {
    set_key(LOCK, lock);
}

pub fn get_lock() -> bool {
    get_key(LOCK).unwrap_or_revert()
}
