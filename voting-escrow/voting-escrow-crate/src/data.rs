use alloc::{
  format,
  string::{String, ToString},
  vec::Vec,
};
use casper_contract::contract_api::runtime::get_blocktime;
use casper_types::{ContractHash, ContractPackageHash, Key, U128, U256};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use casperlabs_contract_utils::{get_key, set_key, Dict};
use common::{keys::*, utils::*};

pub const DEPOSIT_FOR_TYPE: i128 = 0;
pub const CREATE_LOCK_TYPE: i128 = 1;
pub const INCREASE_LOCK_AMOUNT: i128 = 2;
pub const INCREASE_UNLOCK_TIME: i128 = 3;
pub const WEEK: U256 = U256([604800000, 0, 0, 0]); // all future times are rounded by week
pub const MAXTIME: U256 = U256([126144000000, 0, 0, 0]); // 4 years
pub const MULTIPLIER: U256 = U256([1000000000, 0, 0, 0]);

// We cannot really do block numbers per se b/c slope is per time, not per block
// and per block could be fairly bad b/c Ethereum changes blocktimes.
// What we can do is to extrapolate ***At functions
#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct Point {
  pub bias: (bool, U128),
  pub slope: (bool, U128), // - dweight / dt
  pub ts: U256,
  pub blk: U256,
}

impl Point {
  pub fn set_bias(&mut self, value: i128) {
      self.bias = i128_to_tuple(value);
  }

  pub fn bias(&self) -> i128 {
      tuple_to_i128(self.bias)
  }

  pub fn set_slope(&mut self, value: i128) {
      self.slope = i128_to_tuple(value);
  }

  pub fn slope(&self) -> i128 {
      tuple_to_i128(self.slope)
  }
}

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct LockedBalance {
  pub amount: (bool, U128),
  pub end: U256,
}
impl LockedBalance {
  pub fn set_amount(&mut self, value: i128) {
      self.amount = i128_to_tuple(value);
  }

  pub fn amount(&self) -> i128 {
      tuple_to_i128(self.amount)
  }
}

pub const LOCKED: &str = "locked";
#[derive(CLTyped, ToBytes, FromBytes)]
pub struct Locked {
  dict: Dict,
}

impl Locked {
  pub fn instance() -> Locked {
      Locked {
          dict: Dict::instance(LOCKED),
      }
  }

  pub fn init() {
      Dict::init(LOCKED)
  }

  pub fn get(&self, owner: &Key) -> LockedBalance {
      LockedBalance {
          amount: self
              .dict
              .get(
                  hash(format!(
                      "{}{}{}",
                      LOCKED,
                      "_amount_",
                      owner.to_formatted_string()
                  ))
                  .as_str(),
              )
              .unwrap_or_default(),

          end: self
              .dict
              .get(
                  hash(format!(
                      "{}{}{}",
                      LOCKED,
                      "_end_",
                      owner.to_formatted_string()
                  ))
                  .as_str(),
              )
              .unwrap_or_default(),
      }
  }

  pub fn set(&self, owner: &Key, value: LockedBalance) {
      self.dict.set(
          hash(format!(
              "{}{}{}",
              LOCKED,
              "_amount_",
              owner.to_formatted_string()
          ))
          .as_str(),
          value.amount,
      );

      self.dict.set(
          hash(format!(
              "{}{}{}",
              LOCKED,
              "_end_",
              owner.to_formatted_string()
          ))
          .as_str(),
          value.end,
      );
  }
}

pub const USER_POINT_HISTORY: &str = "user_point_history";
#[derive(CLTyped, ToBytes, FromBytes)]
pub struct UserPointHistory {
  dict: Dict,
}

impl UserPointHistory {
  pub fn instance() -> UserPointHistory {
      UserPointHistory {
          dict: Dict::instance(USER_POINT_HISTORY),
      }
  }

  pub fn init() {
      Dict::init(USER_POINT_HISTORY)
  }

  pub fn get(&self, user: &Key, user_epoch: &U256) -> Point {
      Point {
          bias: self
              .dict
              .get(
                  hash(format!(
                      "{}{}{}{}{}",
                      USER_POINT_HISTORY,
                      "_bias_",
                      user.to_formatted_string(),
                      "_",
                      user_epoch
                  ))
                  .as_str(),
              )
              .unwrap_or_default(),
          slope: self
              .dict
              .get(
                  hash(format!(
                      "{}{}{}{}{}",
                      USER_POINT_HISTORY,
                      "_slope_",
                      user.to_formatted_string(),
                      "_",
                      user_epoch
                  ))
                  .as_str(),
              )
              .unwrap_or_default(),
          ts: self
              .dict
              .get(
                  hash(format!(
                      "{}{}{}{}{}",
                      USER_POINT_HISTORY,
                      "_ts_",
                      user.to_formatted_string(),
                      "_",
                      user_epoch
                  ))
                  .as_str(),
              )
              .unwrap_or_default(),
          blk: self
              .dict
              .get(
                  hash(format!(
                      "{}{}{}{}{}",
                      USER_POINT_HISTORY,
                      "_blk_",
                      user.to_formatted_string(),
                      "_",
                      user_epoch
                  ))
                  .as_str(),
              )
              .unwrap_or_default(),
      }
  }

  pub fn set(&self, user: &Key, user_epoch: &U256, value: Point) {
      self.dict.set(
          hash(format!(
              "{}{}{}{}{}",
              USER_POINT_HISTORY,
              "_bias_",
              user.to_formatted_string(),
              "_",
              user_epoch
          ))
          .as_str(),
          value.bias,
      );

      self.dict.set(
          hash(format!(
              "{}{}{}{}{}",
              USER_POINT_HISTORY,
              "_slope_",
              user.to_formatted_string(),
              "_",
              user_epoch
          ))
          .as_str(),
          value.slope,
      );

      self.dict.set(
          hash(format!(
              "{}{}{}{}{}",
              USER_POINT_HISTORY,
              "_ts_",
              user.to_formatted_string(),
              "_",
              user_epoch
          ))
          .as_str(),
          value.ts,
      );

      self.dict.set(
          hash(format!(
              "{}{}{}{}{}",
              USER_POINT_HISTORY,
              "_blk_",
              user.to_formatted_string(),
              "_",
              user_epoch
          ))
          .as_str(),
          value.blk,
      );
  }
}

pub const USER_POINT_EPOCH: &str = "user_point_epoch";
#[derive(CLTyped, ToBytes, FromBytes)]
pub struct UserPointEpoch {
  dict: Dict,
}

impl UserPointEpoch {
  pub fn instance() -> UserPointEpoch {
      UserPointEpoch {
          dict: Dict::instance(USER_POINT_EPOCH),
      }
  }

  pub fn init() {
      Dict::init(USER_POINT_EPOCH)
  }

  pub fn get(&self, user: &Key) -> U256 {
      self.dict.get_by_key(user).unwrap_or_default()
  }

  pub fn set(&self, user: &Key, value: U256) {
      self.dict.set_by_key(user, value);
  }
}

pub const SLOPE_CHANGES: &str = "slope_changes";
#[derive(CLTyped, ToBytes, FromBytes)]
pub struct SlopeChanges {
  dict: Dict,
}

impl SlopeChanges {
  pub fn instance() -> SlopeChanges {
      SlopeChanges {
          dict: Dict::instance(SLOPE_CHANGES),
      }
  }

  pub fn init() {
      Dict::init(SLOPE_CHANGES)
  }

  pub fn get(&self, time: &U256) -> i128 {
      let ret: (bool, U128) = self.dict.get(time.to_string().as_str()).unwrap_or_default();
      tuple_to_i128(ret)
  }

  pub fn set(&self, time: &U256, value: i128) {
      self.dict
          .set(time.to_string().as_str(), i128_to_tuple(value));
  }
}

pub const POINT_HISTORY: &str = "point_history";
#[derive(CLTyped, ToBytes, FromBytes)]
pub struct PointHistory {
  dict: Dict,
  length: U256,
}

impl PointHistory {
  pub fn instance() -> PointHistory {
      PointHistory {
          dict: Dict::instance(POINT_HISTORY),
          length: 0.into(),
      }
  }

  pub fn init() {
      Dict::init(POINT_HISTORY)
  }

  pub fn get(&self, epoch: &U256) -> Point {
      Point {
          bias: self
              .dict
              .get(hash(format!("{}{}{}", POINT_HISTORY, "_bias_", epoch)).as_str())
              .unwrap_or_default(),
          slope: self
              .dict
              .get(hash(format!("{}{}{}", POINT_HISTORY, "_slope_", epoch)).as_str())
              .unwrap_or_default(),
          ts: self
              .dict
              .get(hash(format!("{}{}{}", POINT_HISTORY, "_ts_", epoch)).as_str())
              .unwrap_or_default(),
          blk: self
              .dict
              .get(hash(format!("{}{}{}", POINT_HISTORY, "_blk_", epoch)).as_str())
              .unwrap_or_default(),
      }
  }

  pub fn set(&self, epoch: &U256, value: Point) {
      self.dict.set(
          hash(format!("{}{}{}", POINT_HISTORY, "_bias_", epoch)).as_str(),
          value.bias,
      );

      self.dict.set(
          hash(format!("{}{}{}", POINT_HISTORY, "_slope_", epoch)).as_str(),
          value.slope,
      );

      self.dict.set(
          hash(format!("{}{}{}", POINT_HISTORY, "_ts_", epoch)).as_str(),
          value.ts,
      );

      self.dict.set(
          hash(format!("{}{}{}", POINT_HISTORY, "_blk_", epoch)).as_str(),
          value.blk,
      );
  }
}

pub fn get_token() -> Key {
  get_key(TOKEN).unwrap_or_else(zero_address)
}

pub fn set_token(token: Key) {
  set_key(TOKEN, token);
}

pub fn get_supply() -> U256 {
  get_key(SUPPLY).unwrap_or_default()
}

pub fn set_supply(supply: U256) {
  set_key(SUPPLY, supply);
}

pub fn get_admin() -> Key {
  get_key(ADMIN).unwrap_or_else(zero_address)
}

pub fn set_admin(admin: Key) {
  set_key(ADMIN, admin);
}

pub fn get_future_admin() -> Key {
  get_key(FUTURE_ADMIN).unwrap_or_else(zero_address)
}

pub fn set_future_admin(future_admin: Key) {
  set_key(FUTURE_ADMIN, future_admin);
}

pub fn get_controller() -> Key {
  get_key(CONTROLLER).unwrap_or_else(zero_address)
}

pub fn set_controller(controller: Key) {
  set_key(CONTROLLER, controller);
}

pub fn get_transfers_enabled() -> bool {
  get_key(TRANSFERS_ENABLED).unwrap_or_default()
}

pub fn set_transfers_enabled(transfers_enabled: bool) {
  set_key(TRANSFERS_ENABLED, transfers_enabled);
}

pub fn get_name() -> String {
  get_key(NAME).unwrap_or_default()
}

pub fn set_name(name: String) {
  set_key(NAME, name);
}

pub fn get_symbol() -> String {
  get_key(SYMBOL).unwrap_or_default()
}

pub fn set_symbol(symbol: String) {
  set_key(SYMBOL, symbol);
}

pub fn get_version() -> String {
  get_key(VERSION).unwrap_or_default()
}

pub fn set_version(version: String) {
  set_key(VERSION, version);
}

pub fn get_decimals() -> U256 {
  get_key(DECIMALS).unwrap_or_default()
}

pub fn set_decimals(decimals: U256) {
  set_key(DECIMALS, decimals);
}

pub fn get_epoch() -> U256 {
  get_key(EPOCH).unwrap_or_default()
}

pub fn set_epoch(epoch: U256) {
  set_key(EPOCH, epoch);
}

pub fn get_lock() -> bool {
  get_key(LOCK).unwrap_or_default()
}

pub fn set_lock(lock: bool) {
  set_key(LOCK, lock);
}

pub fn get_contract_hash() -> ContractHash {
  get_key(SELF_CONTRACT_HASH).unwrap_or_default()
}

pub fn set_contract_hash(contract_hash: ContractHash) {
  set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
  get_key(SELF_CONTRACT_PACKAGE_HASH).unwrap_or_default()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
  set_key(SELF_CONTRACT_PACKAGE_HASH, package_hash);
}

pub fn convert(a: U256, b: U256) -> i128 {
  if a > b {
      (a - b).to_string().as_str().parse::<i128>().unwrap()
  } else {
      -(b - a).to_string().as_str().parse::<i128>().unwrap()
  }
}

pub fn block_number() -> u64 {
  const AVG_BLOCK_TIME_IN_MS: u64 = 45000;
  u64::from(get_blocktime()) / AVG_BLOCK_TIME_IN_MS
}