use crate::{data::*, event::GaugeProxyEvent};
use alloc::{
    string::String,
    vec::Vec,
    {collections::BTreeMap, string::ToString},
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::errors::Error;

pub trait GAUGEPROXY<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &self,
        ownership_admin: Key,
        emergency_admin: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        set_ownership_admin(ownership_admin);
        set_emergency_admin(emergency_admin);
        set_contract_hash(contract_hash);
        set_package_hash(package_hash);
    }

    #[inline(always)]
    fn commit_set_admins(&self, o_admin: Key, e_admin: Key) {
        if self.get_caller() != get_ownership_admin() {
            runtime::revert(ApiError::from(Error::GaugeProxyAccessDenied1));
        };
        set_future_ownership_admin(o_admin);
        set_future_emergency_admin(e_admin);
        self.emit(&GaugeProxyEvent::CommitAdmins {
            ownership_admin: o_admin,
            emergency_admin: e_admin,
        });
    }

    #[inline(always)]
    fn accept_set_admins(&self) {
        if self.get_caller() != get_future_ownership_admin() {
            runtime::revert(ApiError::from(Error::GaugeProxyAccessDenied2));
        };
        let e_admin: Key = get_future_emergency_admin();
        set_ownership_admin(self.get_caller());
        set_emergency_admin(e_admin);
        self.emit(&GaugeProxyEvent::ApplyAdmins {
            ownership_admin: self.get_caller(),
            emergency_admin: e_admin,
        });
    }

    #[inline(always)]
    fn commit_transfer_ownership(&self, gauge: Key, new_owner: Key) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::GaugeProxyIsLocked1));
        }
        set_lock(true);
        if self.get_caller() != get_ownership_admin() {
            runtime::revert(ApiError::from(Error::GaugeProxyAccessDenied3));
        };
        let () = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "commit_transfer_ownership",
            runtime_args! {
                "addr" => new_owner
            },
        );
        set_lock(false);
    }

    #[inline(always)]
    fn accept_transfer_ownership(&self, gauge: Key) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::GaugeProxyIsLocked2));
        }
        set_lock(true);
        let () = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "accept_transfer_ownership",
            runtime_args! {},
        );
        set_lock(false);
    }

    #[inline(always)]
    fn set_killed(&self, gauge: Key, is_killed: bool) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::GaugeProxyIsLocked3));
        }
        set_lock(true);
        if !(self.get_caller() == get_ownership_admin()
            || self.get_caller() == get_emergency_admin())
        {
            runtime::revert(ApiError::from(Error::GaugeProxyAccessDenied4));
        }
        let () = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "set_killed",
            runtime_args! {
                "is_killed" => is_killed
            },
        );
        set_lock(false);
    }

    #[inline(always)]
    fn set_rewards(&self, gauge: Key, reward_contract: Key, sigs: String, reward_tokens: Vec<Key>) {
        if self.get_caller() != get_ownership_admin() {
            runtime::revert(ApiError::from(Error::GaugeProxyAccessDenied5));
        };
        let () = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "set_rewards",
            runtime_args! {
                "reward_contract" => reward_contract,
                "sigs" => sigs,
                "reward_tokens" => reward_tokens,
            },
        );
    }

    fn emit(&self, gauge_proxy_event: &GaugeProxyEvent) {
        match gauge_proxy_event {
            GaugeProxyEvent::CommitAdmins {
                emergency_admin,
                ownership_admin,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", gauge_proxy_event.type_name());
                event.insert("emergency_admin", emergency_admin.to_string());
                event.insert("ownership_admin", ownership_admin.to_string());
                storage::new_uref(event);
            }
            GaugeProxyEvent::ApplyAdmins {
                emergency_admin,
                ownership_admin,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", gauge_proxy_event.type_name());
                event.insert("emergency_admin", emergency_admin.to_string());
                event.insert("ownership_admin", ownership_admin.to_string());
                storage::new_uref(event);
            }
        };
    }
}
