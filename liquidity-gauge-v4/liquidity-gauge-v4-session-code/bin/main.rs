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
        // Liquidity gauge v3
        TRANSFER => {
            let recipient: Key = runtime::get_named_arg("recipient");
            let amount: U256 = runtime::get_named_arg("amount");
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                TRANSFER,
                runtime_args! {
                    "recipient"=>recipient,
                    "amount"=>amount
                },
            );
            store(TRANSFER, ret);
        }
        TRANSFER_FROM => {
            let owner: Key = runtime::get_named_arg("owner");
            let recipient: Key = runtime::get_named_arg("recipient");
            let amount: U256 = runtime::get_named_arg("amount");
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                TRANSFER_FROM,
                runtime_args! {
                    "owner"=>owner,
                    "recipient"=>recipient,
                    "amount"=>amount
                },
            );
            store(TRANSFER_FROM, ret);
        }
        CLAIMABLE_TOKENS => {
            let addr: Key = runtime::get_named_arg("addr");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                CLAIMABLE_TOKENS,
                runtime_args! {
                    "addr"=>addr
                },
            );
            store(CLAIMABLE_TOKENS, ret);
        }
        CLAIMABLE_REWARD_WRITE => {
            let addr: Key = runtime::get_named_arg("addr");
            let token: Key = runtime::get_named_arg("token");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                CLAIMABLE_REWARD_WRITE,
                runtime_args! {
                    "addr"=>addr,
                    "token"=>token,
                },
            );
            store(CLAIMABLE_REWARD_WRITE, ret);
        }
        USER_CHECKPOINT => {
            let addr: Key = runtime::get_named_arg("addr");
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                USER_CHECKPOINT,
                runtime_args! {
                    "addr"=>addr
                },
            );
            store(USER_CHECKPOINT, ret);
        }
        INCREASE_ALLOWANCE => {
            let spender: Key = runtime::get_named_arg("spender");
            let amount: U256 = runtime::get_named_arg("amount");
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                INCREASE_ALLOWANCE,
                runtime_args! {
                    "spender"=>spender,
                    "amount"=>amount
                },
            );
            store(INCREASE_ALLOWANCE, ret);
        }
        DECREASE_ALLOWANCE => {
            let spender: Key = runtime::get_named_arg("spender");
            let amount: U256 = runtime::get_named_arg("amount");
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                DECREASE_ALLOWANCE,
                runtime_args! {
                    "spender"=>spender,
                    "amount"=>amount
                },
            );
            store(DECREASE_ALLOWANCE, ret);
        }
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };
}
