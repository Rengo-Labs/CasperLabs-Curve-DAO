use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{contract_api::runtime::get_call_stack, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{system::CallStackElement, ContractPackageHash, Key, U256};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use casperlabs_contract_utils::*;
use common::{
    errors::*,
    keys::*,
    utils::{key_to_str, *},
};

pub const WEEK: U256 = U256([604800000, 0, 0, 0]); // all future times are rounded by week
pub const WEIGHT_VOTE_DELAY: U256 = U256([86400000, 0, 0, 0]);
pub const MULTIPLIER: U256 = U256([1000000000, 0, 0, 0]);

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct Point {
    pub bias: U256,
    pub slope: U256, // - dweight / dt
}

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct VotedSlope {
    pub slope: U256,
    pub power: U256,
    pub end: U256,
}

pub struct GaugeTypeNames {
    dict: Dict,
}

impl GaugeTypeNames {
    #[inline(always)]
    pub fn instance() -> GaugeTypeNames {
        GaugeTypeNames {
            dict: Dict::instance(GAUGE_TYPE_NAMES_DICT),
        }
    }
    #[inline(always)]
    pub fn init() {
        Dict::init(GAUGE_TYPE_NAMES_DICT)
    }
    #[inline(always)]
    pub fn get(&self, owner: &i128) -> String {
        self.dict.get(&owner.to_string()).unwrap_or_default()
    }
    #[inline(always)]
    pub fn set(&self, owner: &i128, value: String) {
        self.dict.set(&owner.to_string(), value);
    }
}

pub struct GaugeTypes_ {
    dict: Dict,
}

impl GaugeTypes_ {
    #[inline(always)]
    pub fn instance() -> GaugeTypes_ {
        GaugeTypes_ {
            dict: Dict::instance(GAUGE_TYPES_DICT),
        }
    }
    #[inline(always)]
    pub fn init() {
        Dict::init(GAUGE_TYPES_DICT)
    }
    #[inline(always)]
    pub fn get(&self, owner: &Key) -> i128 {
        tuple_to_i128(self.dict.get(&key_to_str(owner)).unwrap_or_default())
    }
    #[inline(always)]
    pub fn set(&self, owner: &Key, value: i128) {
        self.dict.set(&key_to_str(owner), i128_to_tuple(value));
    }
}

pub struct VoteUserSlopes {
    dict: Dict,
}

impl VoteUserSlopes {
    pub fn instance() -> VoteUserSlopes {
        VoteUserSlopes {
            dict: Dict::instance(VOTE_USER_SLOPES_DICT),
        }
    }

    pub fn init() {
        Dict::init(VOTE_USER_SLOPES_DICT)
    }

    pub fn get(&self, owner: &Key, spender: &Key) -> VotedSlope {
        VotedSlope {
            slope: self
                .dict
                .get(
                    hash(format!(
                        "{}{}{}{}{}",
                        VOTE_USER_SLOPES_DICT,
                        "_slope_",
                        owner.to_formatted_string(),
                        "_",
                        spender.to_formatted_string()
                    ))
                    .as_str(),
                )
                .unwrap_or_default(),

            power: self
                .dict
                .get(
                    hash(format!(
                        "{}{}{}{}{}",
                        VOTE_USER_SLOPES_DICT,
                        "_power_",
                        owner.to_formatted_string(),
                        "_",
                        spender.to_formatted_string()
                    ))
                    .as_str(),
                )
                .unwrap_or_default(),

            end: self
                .dict
                .get(
                    hash(format!(
                        "{}{}{}{}{}",
                        VOTE_USER_SLOPES_DICT,
                        "_end_",
                        owner.to_formatted_string(),
                        "_",
                        spender.to_formatted_string()
                    ))
                    .as_str(),
                )
                .unwrap_or_default(),
        }
    }

    pub fn set(&self, owner: &Key, spender: &Key, value: VotedSlope) {
        self.dict.set(
            hash(format!(
                "{}{}{}{}{}",
                VOTE_USER_SLOPES_DICT,
                "_slope_",
                owner.to_formatted_string(),
                "_",
                spender.to_formatted_string()
            ))
            .as_str(),
            value.slope,
        );

        self.dict.set(
            hash(format!(
                "{}{}{}{}{}",
                VOTE_USER_SLOPES_DICT,
                "_power_",
                owner.to_formatted_string(),
                "_",
                spender.to_formatted_string()
            ))
            .as_str(),
            value.power,
        );

        self.dict.set(
            hash(format!(
                "{}{}{}{}{}",
                VOTE_USER_SLOPES_DICT,
                "_end_",
                owner.to_formatted_string(),
                "_",
                spender.to_formatted_string()
            ))
            .as_str(),
            value.end,
        );
    }
}

pub struct VoteUserPower {
    dict: Dict,
}

impl VoteUserPower {
    pub fn instance() -> VoteUserPower {
        VoteUserPower {
            dict: Dict::instance(VOTE_USER_POWER_DICT),
        }
    }

    pub fn init() {
        Dict::init(VOTE_USER_POWER_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set(&key_to_str(owner), value);
    }
}

pub struct LastUserVote {
    dict: Dict,
}

impl LastUserVote {
    pub fn instance() -> LastUserVote {
        LastUserVote {
            dict: Dict::instance(LAST_USER_VOTE_DICT),
        }
    }

    pub fn init() {
        Dict::init(LAST_USER_VOTE_DICT)
    }

    pub fn get(&self, owner: &Key, spender: &Key) -> U256 {
        self.dict.get_by_keys((owner, spender)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, spender: &Key, value: U256) {
        self.dict.set_by_keys((owner, spender), value);
    }
}

pub struct PointsWeight {
    dict: Dict,
}

impl PointsWeight {
    #[inline(always)]
    pub fn instance() -> PointsWeight {
        PointsWeight {
            dict: Dict::instance(POINTS_WEIGHT_DICT),
        }
    }
    #[inline(always)]
    pub fn init() {
        Dict::init(POINTS_WEIGHT_DICT)
    }
    #[inline(always)]
    pub fn get(&self, owner: &Key, recipient: &U256) -> Point {
        Point {
            bias: self
                .dict
                .get(
                    hash(format!(
                        "{}{}{}{}{}",
                        POINTS_WEIGHT_DICT,
                        "_bias_",
                        owner.to_formatted_string(),
                        "_",
                        recipient
                    ))
                    .as_str(),
                )
                .unwrap_or_default(),

            slope: self
                .dict
                .get(
                    hash(format!(
                        "{}{}{}{}{}",
                        POINTS_WEIGHT_DICT,
                        "_slope_",
                        owner.to_formatted_string(),
                        "_",
                        recipient
                    ))
                    .as_str(),
                )
                .unwrap_or_default(),
        }
    }
    #[inline(always)]
    pub fn set(&self, owner: &Key, recipient: &U256, value: Point) {
        self.dict.set(
            hash(format!(
                "{}{}{}{}{}",
                POINTS_WEIGHT_DICT,
                "_bias_",
                owner.to_formatted_string(),
                "_",
                recipient
            ))
            .as_str(),
            value.bias,
        );

        self.dict.set(
            hash(format!(
                "{}{}{}{}{}",
                POINTS_WEIGHT_DICT,
                "_slope_",
                owner.to_formatted_string(),
                "_",
                recipient
            ))
            .as_str(),
            value.slope,
        );
    }
}

pub struct ChangesWeight {
    dict: Dict,
}

impl ChangesWeight {
    #[inline(always)]
    pub fn instance() -> ChangesWeight {
        ChangesWeight {
            dict: Dict::instance(CHANGES_WEIGHT_DICT),
        }
    }
    #[inline(always)]
    pub fn init() {
        Dict::init(CHANGES_WEIGHT_DICT)
    }
    #[inline(always)]
    pub fn get(&self, owner: &Key, recipient: &U256) -> U256 {
        let key_: String = key_and_value_to_str(owner, recipient);
        self.dict.get(key_.as_str()).unwrap_or_default()
    }
    #[inline(always)]
    pub fn set(&self, owner: &Key, recipient: &U256, value: U256) {
        let key_: String = key_and_value_to_str(owner, recipient);
        self.dict.set(key_.as_str(), value);
    }
}

pub struct TimeWeight {
    dict: Dict,
}

impl TimeWeight {
    #[inline(always)]
    pub fn instance() -> TimeWeight {
        TimeWeight {
            dict: Dict::instance(TIME_WEIGHT_DICT),
        }
    }
    #[inline(always)]
    pub fn init() {
        Dict::init(TIME_WEIGHT_DICT)
    }
    #[inline(always)]
    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }
    #[inline(always)]
    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set(&key_to_str(owner), value);
    }
}

pub struct Gauges {
    dict: Dict,
    length: U256,
}

impl Gauges {
    #[inline(always)]
    pub fn instance() -> Gauges {
        Gauges {
            dict: Dict::instance(GAUGES_DICT),
            length: 0.into(),
        }
    }
    #[inline(always)]
    pub fn init() {
        Dict::init(GAUGES_DICT)
    }
    #[inline(always)]
    pub fn get(&self, indx: &U256) -> Key {
        match self.dict.get(indx.to_string().as_str()) {
            Some(owner) => owner,
            None => Key::from_formatted_str(
                "hash-0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
        }
    }
    #[inline(always)]
    pub fn set(&self, indx: &U256, value: Key) {
        self.dict.set(indx.to_string().as_str(), value);
    }
    #[inline(always)]
    pub fn push(&mut self, value: Key) {
        self.dict.set(self.length.to_string().as_str(), value);
        self.length = self
            .length
            .checked_add(1.into())
            .unwrap_or_revert_with(Error::GaugeControllerOverFlow28);
    }
}

pub struct TimeSum {
    dict: Dict,
    length: U256,
}

impl TimeSum {
    #[inline(always)]
    pub fn instance() -> TimeSum {
        TimeSum {
            dict: Dict::instance(TIME_SUM_DICT),
            length: 0.into(),
        }
    }
    #[inline(always)]
    pub fn init() {
        Dict::init(TIME_SUM_DICT)
    }
    #[inline(always)]
    pub fn get(&self, indx: &U256) -> U256 {
        self.dict.get(indx.to_string().as_str()).unwrap_or_default()
    }
    #[inline(always)]
    pub fn set(&self, indx: &U256, value: U256) {
        self.dict.set(indx.to_string().as_str(), value);
    }
    #[inline(always)]
    pub fn push(&mut self, value: U256) {
        self.dict.set(self.length.to_string().as_str(), value);
        self.length = self
            .length
            .checked_add(1.into())
            .unwrap_or_revert_with(Error::GaugeControllerOverFlow29);
    }
}

pub struct PointsSum {
    dict: Dict,
}

impl PointsSum {
    #[inline(always)]
    pub fn instance() -> PointsSum {
        PointsSum {
            dict: Dict::instance(POINTS_SUM_DICT),
        }
    }

    #[inline(always)]
    pub fn init() {
        Dict::init(POINTS_SUM_DICT)
    }

    #[inline(always)]
    pub fn get(&self, owner: &i128, recipient: &U256) -> Point {
        Point {
            bias: self
                .dict
                .get(
                    hash(format!(
                        "{}{}{}{}{}",
                        POINTS_SUM_DICT, "_bias_", owner, "_", recipient
                    ))
                    .as_str(),
                )
                .unwrap_or_default(),

            slope: self
                .dict
                .get(
                    hash(format!(
                        "{}{}{}{}{}",
                        POINTS_SUM_DICT, "_slope_", owner, "_", recipient
                    ))
                    .as_str(),
                )
                .unwrap_or_default(),
        }
    }

    #[inline(always)]
    pub fn set(&self, owner: &i128, recipient: &U256, value: Point) {
        self.dict.set(
            hash(format!(
                "{}{}{}{}{}",
                POINTS_SUM_DICT, "_bias_", owner, "_", recipient
            ))
            .as_str(),
            value.bias,
        );

        self.dict.set(
            hash(format!(
                "{}{}{}{}{}",
                POINTS_SUM_DICT, "_slope_", owner, "_", recipient
            ))
            .as_str(),
            value.slope,
        );
    }
}

pub struct ChangesSum {
    dict: Dict,
}

impl ChangesSum {
    pub fn instance() -> ChangesSum {
        ChangesSum {
            dict: Dict::instance(CHANGES_SUM_DICT),
        }
    }

    pub fn init() {
        Dict::init(CHANGES_SUM_DICT)
    }

    pub fn get(&self, owner: &i128, recipient: &U256) -> U256 {
        let key_: String = values_to_str(&owner.to_string(), recipient);
        self.dict.get(key_.as_str()).unwrap_or_default()
    }

    pub fn set(&self, owner: &i128, recipient: &U256, value: U256) {
        let key_: String = values_to_str(&owner.to_string(), recipient);
        self.dict.set(key_.as_str(), value);
    }
}

pub struct PointsTotal {
    dict: Dict,
}

impl PointsTotal {
    #[inline(always)]
    pub fn instance() -> PointsTotal {
        PointsTotal {
            dict: Dict::instance(POINTS_TOTAL_DICT),
        }
    }
    #[inline(always)]
    pub fn init() {
        Dict::init(POINTS_TOTAL_DICT)
    }
    #[inline(always)]
    pub fn get(&self, owner: &U256) -> U256 {
        self.dict.get(&owner.to_string()).unwrap_or_default()
    }
    #[inline(always)]
    pub fn set(&self, owner: &U256, value: U256) {
        self.dict.set(&owner.to_string(), value);
    }
}

pub struct PointsTypeWeight {
    dict: Dict,
}

impl PointsTypeWeight {
    #[inline(always)]
    pub fn instance() -> PointsTypeWeight {
        PointsTypeWeight {
            dict: Dict::instance(POINTS_TYPE_WEIGHT_DICT),
        }
    }
    #[inline(always)]
    pub fn init() {
        Dict::init(POINTS_TYPE_WEIGHT_DICT)
    }
    #[inline(always)]
    pub fn get(&self, owner: &i128, recipient: &U256) -> U256 {
        let key_: String = values_to_str(&owner.to_string(), recipient);
        self.dict.get(key_.as_str()).unwrap_or_default()
    }
    #[inline(always)]
    pub fn set(&self, owner: &i128, recipient: &U256, value: U256) {
        let key_: String = values_to_str(&owner.to_string(), recipient);
        self.dict.set(key_.as_str(), value);
    }
}

pub struct TimeTypeWeight {
    dict: Dict,
    length: U256,
}

impl TimeTypeWeight {
    #[inline(always)]
    pub fn instance() -> TimeTypeWeight {
        TimeTypeWeight {
            dict: Dict::instance(TIME_TYPE_WEIGHT_DICT),
            length: 0.into(),
        }
    }
    #[inline(always)]
    pub fn init() {
        Dict::init(TIME_TYPE_WEIGHT_DICT)
    }
    #[inline(always)]
    pub fn get(&self, indx: &U256) -> U256 {
        self.dict.get(indx.to_string().as_str()).unwrap_or_default()
    }
    #[inline(always)]
    pub fn set(&self, indx: &U256, value: U256) {
        self.dict.set(indx.to_string().as_str(), value);
    }
    #[inline(always)]
    pub fn push(&mut self, value: U256) {
        self.dict.set(self.length.to_string().as_str(), value);
        self.length = self
            .length
            .checked_add(1.into())
            .unwrap_or_revert_with(Error::GaugeControllerOverFlow30);
    }
}

pub fn time_total() -> U256 {
    get_key(TIME_TOTAL).unwrap_or_default()
}

pub fn set_time_total(time_total: U256) {
    set_key(TIME_TOTAL, time_total);
}

pub fn admin() -> Key {
    get_key(ADMIN).unwrap_or_else(zero_address)
}

pub fn set_admin(admin: Key) {
    set_key(ADMIN, admin);
}

pub fn future_admin() -> Key {
    get_key(FUTURE_ADMIN).unwrap_or_else(zero_address)
}

pub fn set_future_admin(future_admin: Key) {
    set_key(FUTURE_ADMIN, future_admin);
}

pub fn token() -> Key {
    get_key(TOKEN).unwrap_or_else(zero_address)
}

pub fn set_token(token: Key) {
    set_key(TOKEN, token);
}

pub fn n_gauge_types() -> i128 {
    tuple_to_i128(get_key(N_GAUGE_TYPES).unwrap_or_default())
}

pub fn set_n_gauge_types(n_gauge_types: i128) {
    set_key(N_GAUGE_TYPES, i128_to_tuple(n_gauge_types));
}
pub fn n_gauges() -> i128 {
    tuple_to_i128(get_key(N_GAUGES).unwrap_or_default())
}

pub fn set_n_gauges(n_gauges: i128) {
    set_key(N_GAUGES, i128_to_tuple(n_gauges));
}

pub fn voting_escrow() -> Key {
    get_key(VOTING_ESCROW).unwrap_or_else(zero_address)
}

pub fn set_voting_escrow(voting_escrow: Key) {
    set_key(VOTING_ESCROW, voting_escrow);
}

pub fn reward_count() -> U256 {
    get_key(REWARD_COUNT).unwrap_or_default()
}

pub fn set_reward_count(reward_count: U256) {
    set_key(REWARD_COUNT, reward_count);
}

pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(SELF_CONTRACT_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(SELF_CONTRACT_PACKAGE_HASH).unwrap_or_revert()
}

pub fn contract_package_hash() -> ContractPackageHash {
    let call_stacks = get_call_stack();
    let last_entry = call_stacks.last().unwrap_or_revert();
    let package_hash: Option<ContractPackageHash> = match last_entry {
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash: _,
        } => Some(*contract_package_hash),
        _ => None,
    };
    package_hash.unwrap_or_revert()
}
