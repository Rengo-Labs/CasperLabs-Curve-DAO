use alloc::string::{String, ToString};
use casper_types::{Key, U256};

pub enum LiquidityGaugeV4Event {
    Deposit {
        provider: Key,
        value: U256,
    },
    Withdraw {
        provider: Key,
        value: U256,
    },
    UpdateLiquidityLimit {
        user: Key,
        original_balance: U256,
        original_supply: U256,
        working_balance: U256,
        working_supply: U256,
    },
    CommitOwnership {
        admin: Key,
    },
    ApplyOwnership {
        admin: Key,
    },
    Transfer {
        from: Key,
        to: Key,
        value: U256,
    },
    Approval {
        owner: Key,
        spender: Key,
        value: U256,
    },
}

impl LiquidityGaugeV4Event {
    pub fn type_name(&self) -> String {
        match self {
            LiquidityGaugeV4Event::Deposit {
                provider: _,
                value: _,
            } => "Deposit",
            LiquidityGaugeV4Event::Withdraw {
                provider: _,
                value: _,
            } => "Withdraw",
            LiquidityGaugeV4Event::UpdateLiquidityLimit {
                user: _,
                original_balance: _,
                original_supply: _,
                working_balance: _,
                working_supply: _,
            } => "UpdateLiquidityLimit",
            LiquidityGaugeV4Event::Transfer {
                from: _,
                to: _,
                value: _,
            } => "Transfer",
            LiquidityGaugeV4Event::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "Approval",
            LiquidityGaugeV4Event::CommitOwnership { admin: _ } => "CommitOwnership",
            LiquidityGaugeV4Event::ApplyOwnership { admin: _ } => "ApplyOwnership",
        }
        .to_string()
    }
}
