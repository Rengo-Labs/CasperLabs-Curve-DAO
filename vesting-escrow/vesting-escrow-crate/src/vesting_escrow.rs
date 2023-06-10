use crate::alloc::string::ToString;
use crate::data::{self, get_package_hash, DisabledAt, FundAdmins, InitialLocked, TotalClaimed};
use alloc::collections::BTreeMap;
use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::{errors::*, utils::*};

pub enum VESTINGESCROWEvent {
    Fund { recipient: Key, amount: U256 },
    Claim { recipient: Key, claimed: U256 },
    CommitOwnership { admin: Key },
    ApplyOwnership { admin: Key },
    ToggleDisable { recipient: Key, disabled: bool },
}

impl VESTINGESCROWEvent {
    pub fn type_name(&self) -> String {
        match self {
            VESTINGESCROWEvent::Fund {
                recipient: _,
                amount: _,
            } => "Fund",
            VESTINGESCROWEvent::Claim {
                recipient: _,
                claimed: _,
            } => "Claim",
            VESTINGESCROWEvent::CommitOwnership { admin: _ } => "CommitOwnership",
            VESTINGESCROWEvent::ApplyOwnership { admin: _ } => "ApplyOwnership",
            VESTINGESCROWEvent::ToggleDisable {
                recipient: _,
                disabled: _,
            } => "ToggleDisable",
        }
        .to_string()
    }
}
#[allow(clippy::too_many_arguments)]
pub trait VESTINGESCROW<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        _token: Key,
        _start_time: U256,
        _end_time: U256,
        _can_disable: bool,
        _fund_admins: Vec<String>,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        data::set_token(_token);
        data::set_admin(self.get_caller());
        data::set_start_time(_start_time);
        data::set_end_time(_end_time);
        data::set_can_disable(_can_disable);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        data::set_lock(0);
        InitialLocked::init();
        TotalClaimed::init();
        DisabledAt::init();
        FundAdmins::init();
        let mut _fund_admins_enabled: bool = false;
        let mut fund_admins: Vec<Key> = Vec::new();
        for fund_admin in &_fund_admins {
            fund_admins.push(Key::from_formatted_str(fund_admin).unwrap());
        }
        for fund_admin in &fund_admins {
            if *fund_admin != account_zero_address() {
                FundAdmins::instance().set(fund_admin, true);
                if !_fund_admins_enabled {
                    _fund_admins_enabled = true;
                    data::set_fund_admins_enabled(true);
                }
            }
        }
    }

    #[inline(always)]
    fn initial_locked(&mut self, owner: Key) -> U256 {
        InitialLocked::instance().get(&owner)
    }
    #[inline(always)]
    fn total_claimed(&mut self, owner: Key) -> U256 {
        TotalClaimed::instance().get(&owner)
    }
    #[inline(always)]
    fn disabled_at(&mut self, owner: Key) -> U256 {
        DisabledAt::instance().get(&owner)
    }
    #[inline(always)]
    fn fund_admins(&mut self, owner: Key) -> bool {
        FundAdmins::instance().get(&owner)
    }

    #[inline(always)]
    fn commit_transfer_ownership(&mut self, addr: Key) -> bool {
        if self.get_caller() != self.admin() {
            //Vesting Escrow Only Admin
            runtime::revert(Error::VestingEscrowOnlyAdmin1);
        }
        data::set_future_admin(addr);
        self.emit(&VESTINGESCROWEvent::CommitOwnership { admin: addr });
        true
    }

    #[inline(always)]
    fn apply_transfer_ownership(&mut self) -> bool {
        if self.get_caller() != self.admin() {
            //Vesting Escrow Only Admin
            runtime::revert(Error::VestingEscrowOnlyAdmin2);
        }
        let _admin = self.future_admin();
        if _admin == zero_address() {
            //Vesting Escrow Admin Not Set
            runtime::revert(Error::VestingEscrowAdminNotSet);
        }
        data::set_admin(_admin);
        self.emit(&VESTINGESCROWEvent::ApplyOwnership { admin: _admin });
        true
    }
    #[inline(always)]
    fn disable_fund_admins(&mut self) {
        if self.get_caller() != self.admin() {
            //Vesting Escrow Only Admin
            runtime::revert(Error::VestingEscrowOnlyAdmin3);
        }
        data::set_fund_admins_enabled(false);
    }
    #[inline(always)]
    fn disable_can_disable(&mut self) {
        if self.get_caller() != self.admin() {
            //Vesting Escrow Only Admin
            runtime::revert(Error::VestingEscrowOnlyAdmin4);
        }
        data::set_can_disable(false);
    }
    #[inline(always)]
    fn toggle_disable(&mut self, _recipient: Key) {
        if self.get_caller() != self.admin() {
            //Vesting Escrow Only Admin
            runtime::revert(Error::VestingEscrowOnlyAdmin5);
        }
        if !self.can_disable() {
            //Vesting Escrow Cannot Disable
            runtime::revert(Error::VestingEscrowCannotDisable);
        }
        let is_disabled: bool = self.disabled_at(_recipient) == 0.into();
        if is_disabled {
            DisabledAt::instance().set(&_recipient, U256::from(u64::from(runtime::get_blocktime())))
        } else {
            DisabledAt::instance().set(&_recipient, 0.into());
        }
        self.emit(&VESTINGESCROWEvent::ToggleDisable {
            recipient: _recipient,
            disabled: is_disabled,
        });
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
    fn can_disable(&mut self) -> bool {
        data::can_disable()
    }
    #[inline(always)]
    fn fund_admins_enabled(&mut self) -> bool {
        data::fund_admins_enabled()
    }
    #[inline(always)]
    fn unallocated_supply(&mut self) -> U256 {
        data::unallocated_supply()
    }
    #[inline(always)]
    fn initial_locked_supply(&mut self) -> U256 {
        data::initial_locked_supply()
    }
    #[inline(always)]
    fn end_time(&mut self) -> U256 {
        data::end_time()
    }
    #[inline(always)]
    fn start_time(&mut self) -> U256 {
        data::start_time()
    }
    #[inline(always)]
    fn token(&mut self) -> Key {
        data::token()
    }

    #[inline(always)]
    fn _total_vested_of(&mut self, _recipient: Key, _time: Option<U256>) -> U256 {
        let time: U256 = if let Some(..) = _time {
            _time.unwrap()
        } else {
            U256::from(u64::from(runtime::get_blocktime()))
        };
        let start: U256 = self.start_time();
        let end: U256 = self.end_time();
        let locked: U256 = self.initial_locked(_recipient);
        if time < start {
            return 0.into();
        }

        if locked
            * (time
                .checked_sub(start)
                .unwrap_or_revert_with(Error::VestingEscrowUnderFlow1))
            / (end
                .checked_sub(start)
                .unwrap_or_revert_with(Error::VestingEscrowUnderFlow2))
            < locked
        {
            locked
                * (time
                    .checked_sub(start)
                    .unwrap_or_revert_with(Error::VestingEscrowUnderFlow3))
                / (end
                    .checked_sub(start)
                    .unwrap_or_revert_with(Error::VestingEscrowUnderFlow4))
        } else {
            locked
        }
    }
    #[inline(always)]
    fn _total_vested(&mut self) -> U256 {
        let start: U256 = self.start_time();
        let end: U256 = self.end_time();
        let locked: U256 = self.initial_locked_supply();
        if U256::from(u64::from(runtime::get_blocktime())) < start {
            return U256::from(0);
        }
        if locked
            * (U256::from(u64::from(runtime::get_blocktime()))
                .checked_sub(start)
                .unwrap_or_revert_with(Error::VestingEscrowUnderFlow5))
            / (end
                .checked_sub(start)
                .unwrap_or_revert_with(Error::VestingEscrowUnderFlow6))
            < locked
        {
            locked
                * (U256::from(u64::from(runtime::get_blocktime()))
                    .checked_sub(start)
                    .unwrap_or_revert_with(Error::VestingEscrowUnderFlow7))
                / (end
                    .checked_sub(start)
                    .unwrap_or_revert_with(Error::VestingEscrowUnderFlow8))
        } else {
            locked
        }
    }
    #[inline(always)]
    fn vested_supply(&mut self) -> U256 {
        self._total_vested()
    }
    #[inline(always)]
    fn locked_supply(&mut self) -> U256 {
        self.initial_locked_supply()
            .checked_sub(self._total_vested())
            .unwrap_or_revert_with(Error::VestingEscrowUnderFlow9)
    }
    #[inline(always)]
    fn vested_of(&mut self, _recipient: Key) -> U256 {
        self._total_vested_of(_recipient, None)
    }

    #[inline(always)]
    fn balance_of(&mut self, _recipient: Key) -> U256 {
        self._total_vested_of(_recipient, None)
            .checked_sub(self.total_claimed(_recipient))
            .unwrap_or_revert_with(Error::VestingEscrowUnderFlow10)
    }
    #[inline(always)]
    fn locked_of(&mut self, _recipient: Key) -> U256 {
        self.initial_locked(_recipient)
            .checked_sub(self._total_vested_of(_recipient, None))
            .unwrap_or_revert_with(Error::VestingEscrowUnderFlow11)
    }
    #[inline(always)]
    fn add_tokens(&mut self, _amount: U256) {
        if self.get_caller() != self.admin() {
            //Vesting Escrow Only Admin
            runtime::revert(Error::VestingEscrowOnlyAdmin6);
        }

        let token_hash_add_array = match self.token() {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token_package_hash = ContractPackageHash::new(token_hash_add_array);
        let () = runtime::call_versioned_contract(
            token_package_hash,
            None,
            "transfer_from",
            runtime_args! {"owner" => self.get_caller(),"recipient" =>  Key::from(data::get_package_hash()),"amount" => _amount},
        );
        let unallocated_supply = self.unallocated_supply();
        let res = unallocated_supply
            .checked_add(_amount)
            .unwrap_or_revert_with(Error::VestingEscrowOverFlow1);
        data::set_unallocated_supply(res);
    }

    #[inline(always)]
    fn fund(&mut self, _recipients: Vec<String>, _amounts: Vec<U256>) {
        let lock = data::get_lock();
        if lock != 0 {
            //Reward Only Gauge: Locked
            runtime::revert(Error::VestingEscrowLocked1);
        }
        data::set_lock(1);
        let mut recipients: Vec<Key> = Vec::new();
        for recipient in &_recipients {
            recipients.push(Key::from_formatted_str(recipient).unwrap());
        }
        if self.get_caller() != self.admin() {
            if !self.fund_admins(self.get_caller()) {
                //Vesting Escrow Only Admin7
                runtime::revert(Error::VestingEscrowOnlyAdmin7);
            }
            if !self.fund_admins_enabled() {
                runtime::revert(Error::VestingEscrowFundAdminsDisabled);
            }
        }
        let mut _total_amount: U256 = 0.into();
        for i in 0..(recipients.len()) {
            let amount = _amounts[i];
            let recipient = recipients[i];
            if recipient == zero_address() || recipient == account_zero_address() {
                break;
            }
            _total_amount = _total_amount
                .checked_add(amount)
                .unwrap_or_revert_with(Error::VestingEscrowOverFlow2);
            let initial_locked = self.initial_locked(recipient);
            InitialLocked::instance().set(
                &recipient,
                initial_locked
                    .checked_add(amount)
                    .unwrap_or_revert_with(Error::VestingEscrowOverFlow3),
            );
            self.emit(&VESTINGESCROWEvent::Fund { recipient, amount });
        }
        let initial_locked_supply = self.initial_locked_supply();
        data::set_initial_locked_supply(
            initial_locked_supply
                .checked_add(_total_amount)
                .unwrap_or_revert_with(Error::VestingEscrowOverFlow4),
        );
        let unallocated_supply = self.unallocated_supply();
        data::set_unallocated_supply(
            unallocated_supply
                .checked_sub(_total_amount)
                .unwrap_or_revert_with(Error::VestingEscrowUnderFlow12),
        );
        data::set_lock(0);
    }
    #[inline(always)]
    fn claim(&mut self, _addr: Option<Key>) {
        let lock = data::get_lock();
        if lock != 0 {
            //Reward Only Gauge: Locked
            runtime::revert(Error::VestingEscrowLocked2);
        }
        data::set_lock(1);
        let addr: Key = if let Some(..) = _addr {
            _addr.unwrap()
        } else {
            self.get_caller()
        };
        let mut t = self.disabled_at(addr);
        if t == 0.into() {
            t = U256::from(u64::from(runtime::get_blocktime()));
        }
        let claimable = self
            ._total_vested_of(addr, Some(t))
            .checked_sub(self.total_claimed(addr))
            .unwrap_or_revert_with(Error::VestingEscrowUnderFlow13);
        let total_claimed = self.total_claimed(addr);
        let res = total_claimed
            .checked_add(claimable)
            .unwrap_or_revert_with(Error::VestingEscrowOverFlow5);
        TotalClaimed::instance().set(&addr, res);
        self.emit(&VESTINGESCROWEvent::Claim {
            recipient: addr,
            claimed: claimable,
        });
        data::set_lock(0);
    }

    fn emit(&mut self, vesting_escrow_event: &VESTINGESCROWEvent) {
        match vesting_escrow_event {
            VESTINGESCROWEvent::Fund { recipient, amount } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", vesting_escrow_event.type_name());
                event.insert("recipient", recipient.to_string());
                event.insert("amount", amount.to_string());
                storage::new_uref(event);
            }
            VESTINGESCROWEvent::Claim { recipient, claimed } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", vesting_escrow_event.type_name());
                event.insert("recipient", recipient.to_string());
                event.insert("claimed", claimed.to_string());
                storage::new_uref(event);
            }
            VESTINGESCROWEvent::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", vesting_escrow_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            VESTINGESCROWEvent::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", vesting_escrow_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            VESTINGESCROWEvent::ToggleDisable {
                recipient,
                disabled,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", vesting_escrow_event.type_name());
                event.insert("recipient", recipient.to_string());
                event.insert("disabled", disabled.to_string());
                storage::new_uref(event);
            }
        };
    }

    #[inline(always)]
    fn get_package_hash(&mut self) -> ContractPackageHash {
        data::get_package_hash()
    }
}
