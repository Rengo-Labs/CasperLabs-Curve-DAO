use crate::alloc::string::ToString;
use crate::data::{self, AllowedToMintFor, Minted};
use alloc::collections::BTreeMap;
use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::runtime;
use casper_contract::contract_api::storage;
use casper_types::{
    runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U128, U256,
};
use common::errors::*;
use contract_utils::{ContractContext, ContractStorage};
pub enum MINTEREvent {
    Minted {
        recipient: Key,
        gauge: Key,
        minted: U256,
    },
}

impl MINTEREvent {
    pub fn type_name(&self) -> String {
        match self {
            MINTEREvent::Minted {
                recipient: _,
                gauge: _,
                minted: _,
            } => "minted",
        }
        .to_string()
    }
}

pub trait MINTER<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        token: Key,
        controller: Key,
        contract_hash: Key,
        package_hash: ContractPackageHash,
        lock: u64,
    ) {
        data::set_token(token);
        data::set_controller(controller);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        data::set_lock(lock);
        Minted::init();
        AllowedToMintFor::init();
    }

    fn _mint_for(&mut self, gauge_addr: Key, _for: Key) {
        let controller: Key = self.controller();
        let controller_hash_add_array = match controller {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let controller_package_hash = ContractPackageHash::new(controller_hash_add_array);
        let ret: U128 = runtime::call_versioned_contract(
            controller_package_hash,
            None,
            "gauge_types",
            runtime_args! {"addr" => gauge_addr},
        );

        if ret <= 0.into() {
            //dev: gauge is not added
            runtime::revert(Error::MinterIsNotAdded);
        }

        let gauge_addr_hash_add_array = match gauge_addr {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let gauge_addr_package_hash = ContractPackageHash::new(gauge_addr_hash_add_array);
        let _ret: bool = runtime::call_versioned_contract(
            gauge_addr_package_hash,
            None,
            "user_checkpoint",
            runtime_args! {"addr" => _for},
        );
        let total_mint: U256 = runtime::call_versioned_contract(
            gauge_addr_package_hash,
            None,
            "integrate_fraction",
            runtime_args! {"key" => _for},
        );
        let minted = self.minted(_for, gauge_addr);
        let to_mint: U256 = total_mint - minted;
        if to_mint != U256::from(0) {
            let token = self.token();
            let token_hash_add_array = match token {
                Key::Hash(package) => package,
                _ => runtime::revert(ApiError::UnexpectedKeyVariant),
            };
            let token_package_hash = ContractPackageHash::new(token_hash_add_array);
            let _result: () = runtime::call_versioned_contract(
                token_package_hash,
                None,
                "mint",
                runtime_args! {"to" => _for,"amount" => to_mint},
            );
            Minted::instance().set(&_for, &gauge_addr, total_mint);
            self.emit(&MINTEREvent::Minted {
                recipient: _for,
                gauge: gauge_addr,
                minted: total_mint,
            });
        }
    }
    fn mint(&mut self, gauge_addr: Key) {
        let lock = data::get_lock();
        if lock != 0 {
            //Minter: Locked
            runtime::revert(Error::MinterLocked1);
        }
        data::set_lock(1);
        self._mint_for(gauge_addr, self.get_caller());
        data::set_lock(0);
    }
    fn mint_many(&mut self, gauge_addrs: Vec<Key>) {
        let lock = data::get_lock();
        if lock != 0 {
            //Minter: Locked
            runtime::revert(Error::MinterLocked2);
        }
        data::set_lock(1);
        for i in 0..(gauge_addrs.len()) {
            self._mint_for(gauge_addrs[i], self.get_caller())
        }
        data::set_lock(0);
    }
    fn mint_for(&mut self, gauge_addr: Key, _for: Key) {
        let lock = data::get_lock();
        if lock != 0 {
            //Minter: Locked
            runtime::revert(Error::MinterLocked3);
        }
        data::set_lock(1);
        let is_allowed = self.allowed_to_mint_for(self.get_caller(), _for);
        if is_allowed == true {
            self._mint_for(gauge_addr, _for)
        }
        data::set_lock(0);
    }

    fn toggle_approve_mint(&mut self, minting_user: Key) {
        let is_allowed = self.allowed_to_mint_for(minting_user, self.get_caller());
        AllowedToMintFor::instance().set(&minting_user, &self.get_caller(), !is_allowed);
    }

    fn allowed_to_mint_for(&mut self, owner: Key, spender: Key) -> bool {
        AllowedToMintFor::instance().get(&owner, &spender)
    }
    fn minted(&mut self, owner: Key, spender: Key) -> U256 {
        Minted::instance().get(&owner, &spender)
    }

    fn token(&mut self) -> Key {
        data::token()
    }
    fn controller(&mut self) -> Key {
        data::controller()
    }

    fn emit(&mut self, minter_event: &MINTEREvent) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match minter_event {
            MINTEREvent::Minted {
                recipient,
                gauge,
                minted,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", minter_event.type_name());
                event.insert("recipient", recipient.to_string());
                event.insert("gauge", gauge.to_string());
                event.insert("minted", minted.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }

    fn get_package_hash(&mut self) -> ContractPackageHash {
        data::get_package_hash()
    }
}
