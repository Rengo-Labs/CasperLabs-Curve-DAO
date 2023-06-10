use crate::data::*;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use casper_types::{ContractHash, ContractPackageHash};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use crv20::data::get_package_hash;
use crv20::{Address, CURVEERC20};

pub trait LPTOKENWRAPPER<Storage: ContractStorage>:
    ContractContext<Storage> + CURVEERC20<Storage>
{
    fn init(&self, uni: Key, contract_hash: ContractHash, package_hash: ContractPackageHash) {
        CURVEERC20::init(self, contract_hash, package_hash);
        set_uni(uni);
    }
    #[inline(always)]
    fn stake(&mut self, amount: U256) {
        self.mint(Address::from(self.get_caller()), amount)
            .unwrap_or_revert();
        runtime::call_versioned_contract::<()>(
            get_uni().into_hash().unwrap_or_revert().into(),
            None,
            "transfer_from",
            runtime_args! {
                "owner" => Address::from(self.get_caller()),
                "recipient" => Address::Contract(get_package_hash()),
                "amount" => amount
            },
        );
    }
    #[inline(always)]
    fn withdraw(&mut self, amount: U256) {
        self.burn(Address::from(self.get_caller()), amount)
            .unwrap_or_revert();
        runtime::call_versioned_contract::<()>(
            get_uni().into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => Address::from(self.get_caller()),
                "amount" => amount
            },
        );
    }
}
