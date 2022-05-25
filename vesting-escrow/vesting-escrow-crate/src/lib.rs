#![no_std]
extern crate alloc;

pub mod data;
pub mod event;
mod vesting_escrow;

pub use contract_utils;
pub use vesting_escrow::{Error, VESTINGESCROW};

use alloc::{collections::BTreeMap, string::String};
use casper_types::U256;
pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;
