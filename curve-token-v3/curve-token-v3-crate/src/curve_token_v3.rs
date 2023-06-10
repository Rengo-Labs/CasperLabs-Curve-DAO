use crate::data;
use crate::{data::*, event::CurveTokenV3Event};
use alloc::string::String;
use alloc::{collections::BTreeMap, string::ToString};
use casper_contract::{
    contract_api::{
        runtime::{self},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use crv20::{self, Address, CURVEERC20};
use curve_casper_erc20::Error as Erc20Error;

use casper_types::{
    runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs, U256,
};
use common::{errors::*, utils::*};

pub trait CURVETOKENV3<Storage: ContractStorage>:
    ContractContext<Storage> + CURVEERC20<Storage>
{
    fn init(&self, contract_hash: ContractHash, package_hash: ContractPackageHash) {
        set_minter(self.get_caller());
        set_hash(contract_hash);
        set_package_hash(package_hash);
        CURVEERC20::init(self, data::get_hash(), data::get_package_hash());
        self.curve_token_v3_emit(&CurveTokenV3Event::Transfer {
            from: zero_address(),
            to: self.get_caller(),
            value: 0.into(),
        });
    }
    /// @notice Get the number of decimals for this token
    //// @return U256 decimal places
    #[inline(always)]
    fn decimals(&self) -> U256 {
        9.into()
    }
    /// @dev Transfer token for a specified address
    /// @param recipient The address to transfer to.
    /// @param amount The amount to be transferred.
    #[inline(always)]
    fn transfer(&self, recipient: Address, amount: U256) -> Result<(), Erc20Error> {
        CURVEERC20::transfer(self, recipient, amount)
    }
    // /// @dev Transfer tokens from one address to another.
    // ///  @param  owner address The address which you want to send tokens from
    // ///  @param recipient address The address which you want to transfer to
    // ///  @param amount U256 the amount of tokens to be transferred
    #[inline(always)]
    fn transfer_from(
        &self,
        owner: Address,
        recipient: Address,
        amount: U256,
    ) -> Result<(), Erc20Error> {
        CURVEERC20::transfer_from(self, owner, recipient, amount)
    }
    // /// @notice Approve the passed address to transfer the specified amount of
    // ///         tokens on behalf of self.get_caller
    // /// @dev Beware that changing an allowance via this method brings the risk
    // ///      that someone may use both the old and new allowance by unfortunate
    // ///      transaction ordering. This may be mitigated with the use of
    // ///      {increase_allowance} and {decrease_allowance}.
    // /// @param spender The address which will transfer the funds
    // /// @param amount The amount of tokens that may be transferred
    #[inline(always)]
    fn approve(&self, spender: Address, amount: U256) -> Result<(), Erc20Error> {
        CURVEERC20::approve(self, spender, amount)
    }
    // /// @notice Increase the allowance granted to `spender` by the caller
    // /// @dev This is alternative to {approve} that can be used as a mitigation for
    // ///      the potential race condition
    // /// @param spender The address which will transfer the funds
    // /// @param added_value The amount of to increase the allowance
    // /// @return ok success
    #[inline(always)]
    fn increase_allowance(&self, spender: Address, amount: U256) -> Result<(), Erc20Error> {
        let res = CURVEERC20::increase_allowance(self, spender, amount);
        self.curve_token_v3_emit(&CurveTokenV3Event::Approval {
            owner: self.get_caller(),
            spender: Key::from(spender),
            value: CURVEERC20::allowance(self, Address::from(self.get_caller()), spender),
        });
        res
    }
    // /// @notice Decrease the allowance granted to `spender` by the caller
    // /// @dev This is alternative to {approve} that can be used as a mitigation for
    // ///      the potential race condition
    // /// @param spender The address which will transfer the funds
    // /// @param amount The amount of to decrease the allowance
    #[inline(always)]
    fn decrease_allowance(&self, spender: Address, amount: U256) -> Result<(), Erc20Error> {
        let res = CURVEERC20::decrease_allowance(self, spender, amount);
        self.curve_token_v3_emit(&CurveTokenV3Event::Approval {
            owner: self.get_caller(),
            spender: Key::from(spender),
            value: CURVEERC20::allowance(self, Address::from(self.get_caller()), spender),
        });
        res
    }
    // /// @dev mint an amount of the token and assigns it to an account.
    // ///      This encapsulates the modification of balances such that the
    // ///      proper events are emitted.
    // /// @param to The account that will receive the created tokens.
    // /// @param amount The amount that will be created.
    #[inline(always)]
    fn mint(&self, to: Address, amount: U256) -> Result<(), Erc20Error> {
        if self.get_caller() != get_minter() {
            runtime::revert(ApiError::from(Error::CurveTokenV3OnlyMinterAllowed));
        }
        let res = CURVEERC20::mint(self, to, amount);
        self.curve_token_v3_emit(&CurveTokenV3Event::Transfer {
            from: zero_address(),
            to: Key::from(to),
            value: amount,
        });
        res
    }
    // /// @dev Burn an amount of the token from a given account.
    // /// @param to The account whose tokens will be burned.
    // /// @param value The amount that will be burned.
    // /// @return bool success
    #[inline(always)]
    fn burn_from(&self, to: Address, value: U256) -> Result<(), Erc20Error> {
        if self.get_caller() != get_minter() {
            runtime::revert(ApiError::from(Error::CurveTokenV3OnlyMinterAllowed2));
        }
        let res = CURVEERC20::burn(self, to, value);
        self.curve_token_v3_emit(&CurveTokenV3Event::Transfer {
            from: Key::from(to),
            to: zero_address(),
            value,
        });
        res
    }

    // /// @dev set minter for a specified address
    // /// @param minter The address to assign minter role.
    #[inline(always)]
    fn set_minter(&self, minter: Key) {
        if self.get_caller() != get_minter() {
            runtime::revert(ApiError::from(Error::CurveTokenV3OnlyMinterCanSet));
        }
        set_minter(minter);
    }
    // /// @dev set name and symbol
    // /// @param name.
    // /// @param symbol.
    #[inline(always)]
    fn set_name(&self, name: String, symbol: String) {
        if self.get_caller()
            != runtime::call_versioned_contract(
                get_minter().into_hash().unwrap_or_revert().into(),
                None,
                "owner",
                runtime_args! {},
            )
        {
            runtime::revert(ApiError::from(Error::CurveTokenV3NotAuthorized));
        }
        CURVEERC20::set_name(self, name);
        CURVEERC20::set_symbol(self, symbol);
    }

    #[inline(always)]
    fn named_keys(
        &self,
        name: String,
        symbol: String,
    ) -> Result<BTreeMap<String, Key>, Erc20Error> {
        CURVEERC20::named_keys(self, name, symbol, 9, 0.into())
    }
    fn curve_token_v3_emit(&self, curve_token_v3_event: &CurveTokenV3Event) {
        let package = get_package_hash();
        match curve_token_v3_event {
            CurveTokenV3Event::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", curve_token_v3_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            CurveTokenV3Event::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", curve_token_v3_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("spender", spender.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
        };
    }
}
