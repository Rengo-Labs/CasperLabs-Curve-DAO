use crate::alloc::string::ToString;
use crate::data::{self, get_package_hash, MIN_VESTING_DURATION};
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::{errors::*, utils::*};
use vesting_escrow_simple_crate::entry_points::get_entry_points;

pub enum VESTINGESCROWFACTORYEvent {
    CommitOwnership { admin: Key },
    ApplyOwnership { admin: Key },
}

impl VESTINGESCROWFACTORYEvent {
    pub fn type_name(&self) -> String {
        match self {
            VESTINGESCROWFACTORYEvent::CommitOwnership { admin: _ } => "CommitOwnership",
            VESTINGESCROWFACTORYEvent::ApplyOwnership { admin: _ } => "ApplyOwnership",
        }
        .to_string()
    }
}

pub trait VESTINGESCROWFACTORY<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        _target: Key,
        _admin: Key,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        data::set_target(_target);
        data::set_admin(_admin);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
    }

    #[inline(always)]
    fn deploy_vesting_contract(
        &mut self,
        _token: Key,
        _recipient: Key,
        _amount: U256,
        _can_disable: bool,
        _vesting_duration: U256,
        _vesting_start: Option<U256>,
        // _vesting_escrow_simple_contract: Key,
    ) -> Key {
        // data::set_vesting_escrow_simple_contract(_vesting_escrow_simple_contract);
        let vesting_start: U256 = if let Some(..) = _vesting_start {
            _vesting_start.unwrap()
        } else {
            U256::from(u64::from(runtime::get_blocktime()))
        };

        if self.get_caller() != self.admin() {
            //Vesting Escrow Only Admin
            runtime::revert(Error::VestingEscrowFactoryOnlyAdmin3);
        } else if vesting_start < U256::from(u64::from(runtime::get_blocktime())) {
            //Vesting Escrow Start Time Too Soon
            runtime::revert(Error::VestingEscrowFactoryStartTimeTooSoon);
        } else if _vesting_duration < MIN_VESTING_DURATION {
            //Vesting Escrow Duration Too Soon
            runtime::revert(Error::VestingEscrowFactoryDurationTooShort);
        } else {
            // let _contract: Key = _vesting_escrow_simple_contract;
            let name: String = "VESTINGESCROWSIMPLE".to_string();
            let (package_hash, _) = storage::create_contract_package_at_hash();
            let (contract_hash, _) =
                storage::add_contract_version(package_hash, get_entry_points(), Default::default());
            runtime::put_key(&format!("{}_contract", name), contract_hash.into());
            let end_time = vesting_start
                .checked_add(_vesting_duration)
                .unwrap_or_revert_with(Error::VestingEscrowFactoryOverFlow1);

            let token_hash_add_array = match _token {
                Key::Hash(package) => package,
                _ => runtime::revert(ApiError::UnexpectedKeyVariant),
            };

            let token_package_hash = ContractPackageHash::new(token_hash_add_array);
            let () = runtime::call_versioned_contract(
                token_package_hash,
                None,
                "approve",
                runtime_args! {"spender" =>  Key::from(package_hash),"amount" => _amount},
            );
            let _: bool = runtime::call_versioned_contract(
                package_hash,
                None,
                "initialize",
                runtime_args! {
                    "admin" => self.admin(),
                    "token" =>  _token,
                    "recipient" =>  _recipient,
                    "amount" => _amount,
                    "start_time" => vesting_start,
                    "end_time" => end_time,
                    "can_disable" => _can_disable,
                    "contract_hash" => contract_hash,
                    "package_hash"=> package_hash
                },
            );
            data::set_vesting_escrow_simple_contract_hash(Key::from(contract_hash));
            data::set_vesting_escrow_simple_package_hash(package_hash);
            Key::from(package_hash)
        }
    }

    #[inline(always)]
    fn commit_transfer_ownership(&mut self, addr: Key) -> bool {
        if self.get_caller() != self.admin() {
            //Vesting Escrow Factroy Only Admin
            runtime::revert(Error::VestingEscrowFactoryOnlyAdmin1);
        }
        data::set_future_admin(addr);
        self.emit(&VESTINGESCROWFACTORYEvent::CommitOwnership { admin: addr });
        true
    }

    #[inline(always)]
    fn apply_transfer_ownership(&mut self) -> bool {
        if self.get_caller() != self.admin() {
            //Vesting Escrow Only Admin
            runtime::revert(Error::VestingEscrowFactoryOnlyAdmin2);
        }
        let _admin = self.future_admin();
        if _admin == zero_address() {
            //Vesting Escrow Admin Not Set
            runtime::revert(Error::VestingEscrowFactoryAdminNotSet);
        }
        data::set_admin(_admin);
        self.emit(&VESTINGESCROWFACTORYEvent::ApplyOwnership { admin: _admin });
        true
    }

    #[inline(always)]
    fn admin(&mut self) -> Key {
        data::admin()
    }

    #[inline(always)]
    fn future_admin(&mut self) -> Key {
        data::future_admin()
    }

    #[inline(always)]
    fn target(&mut self) -> Key {
        data::target()
    }

    #[inline(always)]
    fn vesting_escrow_simple_contract(&mut self) -> Key {
        data::vesting_escrow_simple_contract()
    }

    fn emit(&mut self, vesting_escrow_factory_event: &VESTINGESCROWFACTORYEvent) {
        match vesting_escrow_factory_event {
            VESTINGESCROWFACTORYEvent::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", vesting_escrow_factory_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            VESTINGESCROWFACTORYEvent::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", vesting_escrow_factory_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
        };
    }

    #[inline(always)]
    fn get_package_hash(&mut self) -> ContractPackageHash {
        data::get_package_hash()
    }
}
