use casper_contract::{contract_api::runtime::get_call_stack, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{system::CallStackElement, ContractPackageHash, Key, U256};
use casperlabs_contract_utils::{get_key, key_to_str, set_key, Dict};
use common::keys::*;

pub struct FundAdmins {
    dict: Dict,
}

impl FundAdmins {
    pub fn instance() -> FundAdmins {
        FundAdmins {
            dict: Dict::instance(FUND_ADMINS_DICT),
        }
    }

    pub fn init() {
        Dict::init(FUND_ADMINS_DICT)
    }

    pub fn get(&self, owner: &Key) -> bool {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: bool) {
        self.dict.set(&key_to_str(owner), value);
    }
}

pub struct DisabledAt {
    dict: Dict,
}

impl DisabledAt {
    pub fn instance() -> DisabledAt {
        DisabledAt {
            dict: Dict::instance(DISABLED_AT_DICT),
        }
    }

    pub fn init() {
        Dict::init(DISABLED_AT_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set(&key_to_str(owner), value);
    }
}

pub struct TotalClaimed {
    dict: Dict,
}

impl TotalClaimed {
    pub fn instance() -> TotalClaimed {
        TotalClaimed {
            dict: Dict::instance(TOTAL_CLAIMED_DICT),
        }
    }

    pub fn init() {
        Dict::init(TOTAL_CLAIMED_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set(&key_to_str(owner), value);
    }
}

pub struct InitialLocked {
    dict: Dict,
}

impl InitialLocked {
    pub fn instance() -> InitialLocked {
        InitialLocked {
            dict: Dict::instance(INITIAL_LOCKED_DICT),
        }
    }

    pub fn init() {
        Dict::init(INITIAL_LOCKED_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set(&key_to_str(owner), value);
    }
}

pub fn set_lock(lock: u64) {
    set_key(LOCK, lock);
}

pub fn get_lock() -> u64 {
    get_key(LOCK).unwrap_or_revert()
}

pub fn admin() -> Key {
    get_key(ADMIN).unwrap_or_revert()
}

pub fn set_admin(admin: Key) {
    set_key(ADMIN, admin);
}

pub fn token() -> Key {
    get_key(TOKEN).unwrap_or_revert()
}

pub fn set_token(value: Key) {
    set_key(TOKEN, value);
}

pub fn future_admin() -> Key {
    get_key(FUTURE_ADMIN).unwrap_or_revert()
}

pub fn set_future_admin(future_admin: Key) {
    set_key(FUTURE_ADMIN, future_admin);
}

pub fn start_time() -> U256 {
    get_key(START_TIME).unwrap_or_default()
}

pub fn set_start_time(value: U256) {
    set_key(START_TIME, value);
}

pub fn end_time() -> U256 {
    get_key(END_TIME).unwrap_or_default()
}

pub fn set_end_time(value: U256) {
    set_key(END_TIME, value);
}

pub fn initial_locked_supply() -> U256 {
    get_key(INITIAL_LOCKED_SUPPLY).unwrap_or_default()
}

pub fn set_initial_locked_supply(value: U256) {
    set_key(INITIAL_LOCKED_SUPPLY, value);
}

pub fn unallocated_supply() -> U256 {
    get_key(UNALLOCATED_SUPPLY).unwrap_or_default()
}

pub fn set_unallocated_supply(value: U256) {
    set_key(UNALLOCATED_SUPPLY, value);
}

pub fn can_disable() -> bool {
    get_key(CAN_DISABLE).unwrap_or_default()
}

pub fn set_can_disable(value: bool) {
    set_key(CAN_DISABLE, value);
}

pub fn fund_admins_enabled() -> bool {
    get_key(FUND_ADMINS_ENABLED).unwrap_or_default()
}

pub fn set_fund_admins_enabled(value: bool) {
    set_key(FUND_ADMINS_ENABLED, value);
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
