#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::string::String;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, ApiError, CLTyped, Key, RuntimeArgs, URef, U256,
};
use common::keys::*;

// Key is the same a destination
fn store<T: CLTyped + ToBytes>(key: &str, value: T) {
    // Store `value` under a new unforgeable reference.
    let value_ref: URef = storage::new_uref(value);

    // Wrap the unforgeable reference in a value of type `Key`.
    let value_key: Key = value_ref.into();

    // Store this key under the name "special_value" in context-local storage.
    runtime::put_key(key, value_key);
}

#[no_mangle]
pub extern "C" fn call() {
    let entrypoint: String = runtime::get_named_arg("entrypoint");
    let package_hash: Key = runtime::get_named_arg("package_hash");

    match entrypoint.as_str() {
        USER_CHECKPOINT => {
            let addr: Key = runtime::get_named_arg("addr");
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                USER_CHECKPOINT,
                runtime_args! {
                    "addr" => addr,
                },
            );
            store(USER_CHECKPOINT, ret);
        }
        CLAIMABLE_TOKENS => {
            let addr: Key = runtime::get_named_arg("addr");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                CLAIMABLE_TOKENS,
                runtime_args! {
                    "addr" => addr,
                },
            );
            store(CLAIMABLE_TOKENS, ret);
        }
        ALLOWANCE => {
            let owner: Key = runtime::get_named_arg("owner");
            let spender: Key = runtime::get_named_arg("spender");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                ALLOWANCE,
                runtime_args! {
                    "owner" => owner,
                    "spender" => spender,
                },
            );
            store(ALLOWANCE, ret);
        }
        BALANCE_OF => {
            let owner: Key = runtime::get_named_arg("owner");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                BALANCE_OF,
                runtime_args! {
                    "owner" => owner,
                },
            );
            store(BALANCE_OF, ret);
        }
        APPROVED_TO_DEPOSIT => {
            let key0: Key = runtime::get_named_arg("key0");
            let key1: Key = runtime::get_named_arg("key1");
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                APPROVED_TO_DEPOSIT,
                runtime_args! {
                    "key0" => key0,
                    "key1" => key1,
                },
            );
            store(APPROVED_TO_DEPOSIT, ret);
        }
        ADMIN => {
            let ret: Key = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                ADMIN,
                runtime_args! {},
            );
            store(ADMIN, ret);
        }
        FUTURE_ADMIN => {
            let ret: Key = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                FUTURE_ADMIN,
                runtime_args! {},
            );
            store(FUTURE_ADMIN, ret);
        }
        IS_KILLED => {
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                IS_KILLED,
                runtime_args! {},
            );
            store(IS_KILLED, ret);
        }
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };
}
