use core::convert::TryInto;

use crate::{data::*, event::VotingEscrowEvent};
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
};
use casper_contract::{
    contract_api::{
        runtime::{self, get_blocktime},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs, U256,
};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::{errors::*, utils::*};
use crv20::{self, Address};

/// @notice Votes have a weight depending on time, so that users are committed to the future of (whatever they are voting for)
/// @dev Vote weight decays linearly over time. Lock time cannot be more than `MAXTIME` (4 years).
///
/// Voting escrow to have time-weighted votes
/// Votes have a weight depending on time, so that users are committed
/// to the future of (whatever they are voting for).
/// The weight in this implementation is linear, and lock cannot be more than maxtime:
/// w ^
/// 1 +        /
///   |      /
///   |    /
///   |  /
///   |/
/// 0 +--------+------> time
///       maxtime (4 years?)

pub trait VOTINGESCROW<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &self,
        token_addr: Key,
        name: String,
        symbol: String,
        version: String,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        Locked::init();
        UserPointHistory::init();
        UserPointEpoch::init();
        SlopeChanges::init();
        PointHistory::init();

        set_admin(self.get_caller());
        set_token(token_addr);
        PointHistory::instance().set(&U256::from(0), {
            let mut point_history: Point = PointHistory::instance().get(&U256::from(0));
            point_history.blk = block_number().into();
            point_history.ts = U256::from(u64::from(get_blocktime()));
            point_history
        });
        set_controller(self.get_caller());
        set_transfers_enabled(true);
        let decimals: u8 = runtime::call_versioned_contract(
            token_addr.into_hash().unwrap_or_revert().into(),
            None,
            "decimals",
            runtime_args! {},
        );
        let decimals: U256 = decimals.into();
        if decimals > 255.into() {
            runtime::revert(ApiError::from(Error::VotingEscrowInvalidDecimals))
        }
        set_decimals(decimals);
        set_name(name);
        set_symbol(symbol);
        set_version(version);

        set_contract_hash(contract_hash);
        set_package_hash(package_hash);
    }

    #[inline(always)]
    fn only_admin(&self) {
        if self.get_caller() != get_admin() {
            runtime::revert(ApiError::from(Error::VotingEscrowAdminOnly));
        }
    }

    #[inline(always)]
    fn commit_transfer_ownership(&mut self, addr: Key) {
        self.only_admin();
        set_future_admin(addr);
        VOTINGESCROW::emit(self, &VotingEscrowEvent::CommitOwnership { admin: addr });
    }

    #[inline(always)]
    fn apply_transfer_ownership(&mut self) {
        self.only_admin();
        let admin: Key = get_future_admin();
        if admin == zero_address() || admin == account_zero_address() {
            runtime::revert(ApiError::from(Error::VotingEscrowZeroAddress));
        }
        set_admin(admin);
        VOTINGESCROW::emit(self, &VotingEscrowEvent::ApplyOwnership { admin });
    }

    #[inline(always)]
    fn get_last_user_slope(&self, addr: Key) -> i128 {
        let uepoch: U256 = UserPointEpoch::instance().get(&addr);
        UserPointHistory::instance().get(&addr, &uepoch).slope()
    }

    #[inline(always)]
    fn user_point_history_ts(&self, addr: Key, idx: U256) -> U256 {
        UserPointHistory::instance().get(&addr, &idx).ts
    }

    #[inline(always)]
    fn locked_end(&self, addr: Key) -> U256 {
        Locked::instance().get(&addr).end
    }

    /// @notice Record global and per-user data to checkpoint
    /// @param addr User's wallet address. No user checkpoint if 0x0
    /// @param old_locked Pevious locked amount / end lock time for the user
    /// @param new_locked New locked amount / end lock time for the user
    #[inline(always)]
    fn _checkpoint(&self, addr: Key, old_locked: LockedBalance, new_locked: LockedBalance) {
        let mut u_old: Point = Point::default();
        let mut u_new: Point = Point::default();
        let mut old_dslope: i128 = 0.into();
        let mut new_dslope: i128 = 0.into();
        let mut epoch: U256 = get_epoch();
        if addr != zero_address() && addr != account_zero_address() {
            //  Calculate slopes and biases
            //  Kept at zero when they have to
            if (old_locked.end > U256::from(u64::from(get_blocktime())))
                && (old_locked.amount() > 0.into())
            {
                u_old.set_slope(
                    old_locked
                        .amount()
                        .checked_div(MAXTIME.as_u128().try_into().unwrap())
                        .unwrap_or_revert_with(Error::VotingEscrowDivisionError1),
                );
                u_old.set_bias(
                    u_old
                        .slope()
                        .checked_mul(
                            old_locked
                                .end
                                .checked_sub(U256::from(u64::from(get_blocktime())))
                                .unwrap_or_revert_with(Error::VotingEscrowSubtractionError1)
                                .as_u128()
                                .try_into()
                                .unwrap(),
                        )
                        .unwrap_or_revert_with(Error::VotingEscrowMultiplicationError1),
                );
            }
            if (new_locked.end > U256::from(u64::from(get_blocktime())))
                && (new_locked.amount() > 0.into())
            {
                u_new.set_slope(
                    new_locked
                        .amount()
                        .checked_div(MAXTIME.as_u128().try_into().unwrap())
                        .unwrap_or_revert_with(Error::VotingEscrowDivisionError2),
                );
                u_new.set_bias(
                    u_new
                        .slope()
                        .checked_mul(
                            new_locked
                                .end
                                .checked_sub(U256::from(u64::from(get_blocktime())))
                                .unwrap_or_revert_with(Error::VotingEscrowSubtractionError2)
                                .as_u128()
                                .try_into()
                                .unwrap(),
                        )
                        .unwrap_or_revert_with(Error::VotingEscrowMultiplicationError2),
                )
            }
            // Read values of scheduled changes in the slope
            // old_locked.end can be in the past and in the future
            // new_locked.end can ONLY by in the FUTURE unless everything expired: than zeros
            old_dslope = SlopeChanges::instance().get(&old_locked.end);
            if new_locked.end != 0.into() {
                if new_locked.end == old_locked.end {
                    new_dslope = old_dslope
                } else {
                    new_dslope = SlopeChanges::instance().get(&new_locked.end);
                }
            }
        }
        // let mut last_point: Point = Point::default();
        // last_point.ts = U256::from(u64::from(get_blocktime()));
        let mut last_point: Point = Point {
            bias: Default::default(),
            slope: Default::default(),
            ts: U256::from(u64::from(get_blocktime())),
            blk: block_number().into(),
        };
        if epoch > 0.into() {
            last_point = PointHistory::instance().get(&epoch);
        }
        let mut last_checkpoint: U256 = last_point.ts;
        // initial_last_point is used for extrapolation to calculate block number
        // (approximately, for *At methods) and save them
        // as we cannot figure that out exactly from inside the contract
        let initial_last_point: Point = last_point;
        let mut block_slope: U256 = 0.into();
        if U256::from(u64::from(get_blocktime())) > last_point.ts {
            block_slope = MULTIPLIER
                .checked_mul(
                    (U256::from(block_number())
                        .checked_sub(last_point.blk)
                        .unwrap_or_revert_with(Error::VotingEscrowSubtractionError29))
                    .checked_div(
                        U256::from(u64::from(get_blocktime()))
                            .checked_sub(last_point.ts)
                            .unwrap_or_revert_with(Error::VotingEscrowSubtractionError30),
                    )
                    .unwrap_or_revert_with(Error::VotingEscrowDivisionError10),
                )
                .unwrap_or_revert_with(Error::VotingEscrowMultiplicationError15);
        }
        // If last point is already recorded in this block, slope=0
        // But that's ok b/c we know the block in such case
        let mut t_i: U256 = last_checkpoint
            .checked_div(WEEK)
            .unwrap_or_revert_with(Error::VotingEscrowDivisionError3)
            .checked_mul(WEEK)
            .unwrap_or_revert_with(Error::VotingEscrowMultiplicationError3);
        for _ in 0..255 {
            // Hopefully it won't happen that this won't get used in 5 years!
            // If it does, users will be able to withdraw but vote weight will be broken
            t_i = t_i
                .checked_add(WEEK)
                .unwrap_or_revert_with(Error::VotingEscrowAdditionError2);
            let mut d_slope: i128 = 0.into();
            if t_i > U256::from(u64::from(get_blocktime())) {
                t_i = U256::from(u64::from(get_blocktime()));
            } else {
                d_slope = SlopeChanges::instance().get(&t_i);
            }
            last_point.set_bias(
                last_point
                    .bias()
                    .checked_sub(
                        last_point
                            .slope()
                            .checked_mul(
                                t_i.checked_sub(last_checkpoint)
                                    .unwrap_or_revert_with(Error::VotingEscrowSubtractionError3)
                                    .as_u128()
                                    .try_into()
                                    .unwrap(),
                            )
                            .unwrap_or_revert_with(Error::VotingEscrowMultiplicationError4),
                    )
                    .unwrap_or_revert_with(Error::VotingEscrowSubtractionError4),
            );
            last_point.set_slope(
                last_point
                    .slope()
                    .checked_add(d_slope)
                    .unwrap_or_revert_with(Error::VotingEscrowAdditionError3),
            );
            if last_point.bias() < 0.into() {
                // This can happen
                last_point.set_bias(0);
            }
            if last_point.slope() < 0.into() {
                // This cannot happen - just in case
                last_point.set_slope(0);
            }
            last_checkpoint = t_i;
            last_point.ts = t_i;
            last_point.blk = initial_last_point
                .blk
                .checked_add(
                    block_slope
                        .checked_mul(
                            (t_i.checked_sub(initial_last_point.ts)
                                .unwrap_or_revert_with(Error::VotingEscrowSubtractionError31))
                            .checked_div(MULTIPLIER)
                            .unwrap_or_revert_with(Error::VotingEscrowDivisionError11),
                        )
                        .unwrap_or_revert_with(Error::VotingEscrowMultiplicationError16),
                )
                .unwrap_or_revert_with(Error::VotingEscrowAdditionError27);
            epoch = epoch
                .checked_add(1.into())
                .unwrap_or_revert_with(Error::VotingEscrowAdditionError5);
            if t_i == U256::from(u64::from(get_blocktime())) {
                last_point.blk = block_number().into();
                break;
            } else {
                PointHistory::instance().set(&epoch, last_point);
            }
        }
        set_epoch(epoch);
        // Now point_history is filled until t=now
        if addr != zero_address() && addr != account_zero_address() {
            // If last point was in this block, the slope change has been applied already
            // But in such case we have 0 slope(s)
            last_point.set_slope(
                last_point
                    .slope()
                    .checked_add(
                        u_new
                            .slope()
                            .checked_sub(u_old.slope())
                            .unwrap_or_revert_with(Error::VotingEscrowSubtractionError5),
                    )
                    .unwrap_or_revert_with(Error::VotingEscrowAdditionError6),
            );
            last_point.set_bias(
                last_point
                    .bias()
                    .checked_add(
                        u_new
                            .bias()
                            .checked_sub(u_old.bias())
                            .unwrap_or_revert_with(Error::VotingEscrowSubtractionError6),
                    )
                    .unwrap_or_revert_with(Error::VotingEscrowAdditionError7),
            );
            if last_point.slope() < 0.into() {
                last_point.set_slope(0);
            }
            if last_point.bias() < 0.into() {
                last_point.set_bias(0);
            }
        }
        // Record the changed point into history
        PointHistory::instance().set(&epoch, last_point);
        if addr != zero_address() && addr != account_zero_address() {
            // Schedule the slope changes (slope is going down)
            // We subtract new_user_slope from [new_locked.end]
            // and add old_user_slope to [old_locked.end]
            if old_locked.end > U256::from(u64::from(get_blocktime())) {
                // old_dslope was <something> - u_old.slope, so we cancel that
                old_dslope = old_dslope
                    .checked_add(u_old.slope())
                    .unwrap_or_revert_with(Error::VotingEscrowAdditionError8);
                if new_locked.end == old_locked.end {
                    // It was a new deposit, not extension
                    old_dslope = old_dslope
                        .checked_sub(u_new.slope())
                        .unwrap_or_revert_with(Error::VotingEscrowSubtractionError7);
                }
                SlopeChanges::instance().set(&old_locked.end, old_dslope);
            }
            if new_locked.end > U256::from(u64::from(get_blocktime()))
                && new_locked.end > old_locked.end
            {
                new_dslope = new_dslope
                    .checked_sub(u_new.slope())
                    .unwrap_or_revert_with(Error::VotingEscrowSubtractionError8); // old slope disappeared at this point
                SlopeChanges::instance().set(&new_locked.end, new_dslope);
            }
            // else: we recorded it already in old_dslope

            // Now handle user history
            let user_epoch: U256 = UserPointEpoch::instance()
                .get(&addr)
                .checked_add(1.into())
                .unwrap_or_revert_with(Error::VotingEscrowAdditionError9);
            UserPointEpoch::instance().set(&addr, user_epoch);
            u_new.ts = U256::from(u64::from(get_blocktime()));
            u_new.blk = block_number().into();
            UserPointHistory::instance().set(&addr, &user_epoch, u_new);
        }
    }

    /// @notice Deposit and lock tokens for a user
    /// @param _addr User's wallet address
    /// @param _value Amount to deposit
    /// @param unlock_time New time when to unlock the tokens, or 0 if unchanged
    /// @param locked_balance Previous locked amount / timestamp
    #[inline(always)]
    fn _deposit_for(
        &mut self,
        addr: Key,
        value: U256,
        unlock_time: U256,
        locked_balance: LockedBalance,
        _type: i128,
    ) {
        let mut locked: LockedBalance = locked_balance;
        let supply_before: U256 = get_supply();
        set_supply(
            supply_before
                .checked_add(value)
                .unwrap_or_revert_with(Error::VotingEscrowAdditionError10),
        );
        let old_locked: LockedBalance = locked;
        // Adding to existing lock, or if a lock is expired - creating a new one
        locked.set_amount(
            locked
                .amount()
                .checked_add(value.as_u128().try_into().unwrap())
                .unwrap_or_revert_with(Error::VotingEscrowAdditionError11),
        );
        if unlock_time != 0.into() {
            locked.end = unlock_time;
        }
        Locked::instance().set(&addr, locked);
        // Possibilities:
        // Both old_locked.end could be current or expired (>/< block.timestamp)
        // value == 0 (extend lock) or value > 0 (add to lock or extend lock)
        // _locked.end > block.timestamp (always)
        self._checkpoint(addr, old_locked, locked);
        if value != 0.into() {
            let _ret: () = runtime::call_versioned_contract(
                get_token().into_hash().unwrap_or_revert().into(),
                None,
                "transfer_from",
                runtime_args! {
                    "owner" => Address::from(addr),
                    "recipient" => Address::from(Key::from(get_package_hash())),
                    "amount" => value
                },
            );
        }
        VOTINGESCROW::emit(
            self,
            &VotingEscrowEvent::Deposit {
                provider: addr,
                value,
                locktime: locked.end,
                _type,
                ts: U256::from(u64::from(get_blocktime())),
            },
        );
        VOTINGESCROW::emit(
            self,
            &VotingEscrowEvent::Supply {
                prev_supply: supply_before,
                supply: supply_before
                    .checked_add(value)
                    .unwrap_or_revert_with(Error::VotingEscrowAdditionError12),
            },
        );
    }

    #[inline(always)]
    fn checkpoint(&self) {
        self._checkpoint(
            zero_address(),
            LockedBalance::default(),
            LockedBalance::default(),
        )
    }

    #[inline(always)]
    fn deposit_for(&mut self, addr: Key, value: U256) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::VotingEscrowIsLocked1));
        }
        set_lock(true);
        let locked: LockedBalance = Locked::instance().get(&addr);
        if value <= 0.into() {
            runtime::revert(ApiError::from(Error::VotingEscrowNeedNonZeroValue1));
        }
        if locked.amount() <= 0.into() {
            runtime::revert(ApiError::from(Error::VotingEscrowNoExistingLockFound1));
        }
        if locked.end <= U256::from(u64::from(get_blocktime())) {
            runtime::revert(ApiError::from(
                Error::VotingEscrowCannotAddToExpiredLockWithdraw1,
            ));
        }
        self._deposit_for(
            addr,
            value,
            0.into(),
            Locked::instance().get(&addr),
            DEPOSIT_FOR_TYPE,
        );
        set_lock(false);
    }

    #[inline(always)]
    fn create_lock(&mut self, value: U256, unlock_time: U256) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::VotingEscrowIsLocked2));
        }
        set_lock(true);
        let unlock_time: U256 = unlock_time
            .checked_div(WEEK)
            .unwrap_or_revert_with(Error::VotingEscrowDivisionError4)
            .checked_mul(WEEK)
            .unwrap_or_revert_with(Error::VotingEscrowMultiplicationError5); // Locktime is rounded down to weeks
        let locked: LockedBalance = Locked::instance().get(&self.get_caller());
        if value <= 0.into() {
            runtime::revert(ApiError::from(Error::VotingEscrowNeedNonZeroValue2));
        }
        if locked.amount() != 0.into() {
            runtime::revert(ApiError::from(Error::VotingEscrowWithdrawOldTokensFirst));
        }
        if unlock_time <= U256::from(u64::from(get_blocktime())) {
            runtime::revert(ApiError::from(
                Error::VotingEscrowCanOnlyLockUntilTimeInTheFuture,
            ));
        }
        if unlock_time
            > U256::from(u64::from(get_blocktime()))
                .checked_add(MAXTIME)
                .unwrap_or_revert_with(Error::VotingEscrowAdditionError13)
        {
            runtime::revert(ApiError::from(Error::VotingEscrowVotingLockCanBe4YearsMax1));
        }
        self._deposit_for(
            self.get_caller(),
            value,
            unlock_time,
            locked,
            CREATE_LOCK_TYPE,
        );
        set_lock(false);
    }

    #[inline(always)]
    fn increase_amount(&mut self, value: U256) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::VotingEscrowIsLocked3));
        }
        set_lock(true);
        let locked: LockedBalance = Locked::instance().get(&self.get_caller());
        if value <= 0.into() {
            runtime::revert(ApiError::from(Error::VotingEscrowNeedNonZeroValue3));
        }
        if locked.amount() <= 0.into() {
            runtime::revert(ApiError::from(Error::VotingEscrowNoExistingLockFound2));
        }
        if locked.end <= U256::from(u64::from(get_blocktime())) {
            runtime::revert(ApiError::from(
                Error::VotingEscrowCannotAddToExpiredLockWithdraw2,
            ));
        }
        self._deposit_for(
            self.get_caller(),
            value,
            0.into(),
            locked,
            INCREASE_LOCK_AMOUNT,
        );
        set_lock(false);
    }

    #[inline(always)]
    fn increase_unlock_time(&mut self, unlock_time: U256) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::VotingEscrowIsLocked4));
        }
        set_lock(true);
        let locked: LockedBalance = Locked::instance().get(&self.get_caller());
        let unlock_time: U256 = unlock_time
            .checked_div(WEEK)
            .unwrap_or_revert_with(Error::VotingEscrowDivisionError5)
            .checked_mul(WEEK)
            .unwrap_or_revert_with(Error::VotingEscrowMultiplicationError6); // Locktime is rounded down to weeks
        if locked.end <= U256::from(u64::from(get_blocktime())) {
            runtime::revert(ApiError::from(Error::VotingEscrowLockExpired));
        }
        if locked.amount() <= 0.into() {
            runtime::revert(ApiError::from(Error::VotingEscrowNothingIsLocked));
        }
        if unlock_time <= locked.end {
            runtime::revert(ApiError::from(
                Error::VotingEscrowCanOnlyIncreaseLockDuration,
            ));
        }
        if unlock_time
            > U256::from(u64::from(get_blocktime()))
                .checked_add(MAXTIME)
                .unwrap_or_revert_with(Error::VotingEscrowAdditionError14)
        {
            runtime::revert(ApiError::from(Error::VotingEscrowVotingLockCanBe4YearsMax2));
        }
        self._deposit_for(
            self.get_caller(),
            0.into(),
            unlock_time,
            locked,
            INCREASE_UNLOCK_TIME,
        );
        set_lock(false);
    }

    #[inline(always)]
    fn withdraw(&mut self) {
        let mut locked: LockedBalance = Locked::instance().get(&self.get_caller());
        if U256::from(u64::from(get_blocktime())) < locked.end {
            runtime::revert(ApiError::from(Error::VotingEscrowTheLockDidntExpire));
        }
        let value: U256 = locked.amount().try_into().unwrap();
        let old_locked: LockedBalance = locked;
        locked.end = 0.into();
        locked.set_amount(0);
        Locked::instance().set(&self.get_caller(), locked);
        let supply_before: U256 = get_supply();
        set_supply(
            supply_before
                .checked_sub(value)
                .unwrap_or_revert_with(Error::VotingEscrowSubtractionError9),
        );
        // old_locked can have either expired <= timestamp or zero end
        // _locked has only 0 end
        // Both can have >= 0 amount
        self._checkpoint(self.get_caller(), old_locked, locked);
        let _ret: () = runtime::call_versioned_contract(
            get_token().into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" =>Address::from(self.get_caller()),
                "amount" => value
            },
        );

        VOTINGESCROW::emit(
            self,
            &VotingEscrowEvent::Withdraw {
                provider: self.get_caller(),
                value,
                ts: U256::from(u64::from(get_blocktime())),
            },
        );
        VOTINGESCROW::emit(
            self,
            &VotingEscrowEvent::Supply {
                prev_supply: supply_before,
                supply: supply_before
                    .checked_sub(value)
                    .unwrap_or_revert_with(Error::VotingEscrowSubtractionError10),
            },
        );
    }

    /// The following ERC20/minime-compatible methods are not real balanceOf and supply!
    /// They measure the weights for the purpose of voting, so they don't represent real coins.
    /// @notice Binary search to estimate timestamp for block number
    /// @param _block Block to find
    /// @param max_epoch Don't go beyond this epoch
    /// @return Approximate timestamp for block
    #[inline(always)]
    fn _find_block_epoch(&self, block: U256, max_epoch: U256) -> U256 {
        // Binary search
        let mut min: U256 = 0.into();
        let mut max: U256 = max_epoch;
        for _ in 0..128 {
            // Will be always enough for 128-bit numbers
            if min >= max {
                break;
            }
            let mid: U256 = min
                .checked_add(max)
                .unwrap_or_revert_with(Error::VotingEscrowAdditionError15)
                .checked_add(1.into())
                .unwrap_or_revert_with(Error::VotingEscrowAdditionError16)
                .checked_div(2.into())
                .unwrap_or_revert_with(Error::VotingEscrowAdditionError17);
            if PointHistory::instance().get(&mid).blk <= block {
                min = mid;
            } else {
                max = mid
                    .checked_sub(1.into())
                    .unwrap_or_revert_with(Error::VotingEscrowSubtractionError11);
            }
        }
        min
    }

    #[inline(always)]
    fn balance_of(&self, addr: Key, t: Option<U256>) -> U256 {
        let t: U256 = match t {
            Some(val) => val,
            None => {
                let blocktime: u64 = runtime::get_blocktime().into();
                blocktime.into()
            }
        };

        let epoch: U256 = UserPointEpoch::instance().get(&addr);
        if epoch == 0.into() {
            0.into()
        } else {
            let mut last_point: Point = UserPointHistory::instance().get(&addr, &epoch);
            last_point.set_bias(
                last_point
                    .bias()
                    .checked_sub(
                        last_point
                            .slope()
                            .checked_mul(
                                t.checked_sub(last_point.ts)
                                    .unwrap_or_revert_with(Error::VotingEscrowSubtractionError12)
                                    .as_u128()
                                    .try_into()
                                    .unwrap(),
                            )
                            .unwrap_or_revert_with(Error::VotingEscrowMultiplicationError7),
                    )
                    .unwrap_or_revert_with(Error::VotingEscrowSubtractionError13),
            );
            if last_point.bias() < 0.into() {
                last_point.set_bias(0);
            }
            last_point.bias().try_into().unwrap()
        }
    }

    #[allow(unused_assignments)]
    #[inline(always)]
    fn balance_of_at(&self, addr: Key, block: U256) -> U256 {
        if block > block_number().into() {
            runtime::revert(ApiError::from(Error::VotingEscrowInvalidBlockNumber1));
        }
        // Binary search
        let mut min: U256 = 0.into();
        let mut max: U256 = UserPointEpoch::instance().get(&addr);
        for _ in 0..128 {
            // Will be always enough for 128-bit numbers
            if min >= max {
                break;
            }
            let mid: U256 = min
                .checked_add(max)
                .unwrap_or_revert_with(Error::VotingEscrowAdditionError18)
                .checked_add(1.into())
                .unwrap_or_revert_with(Error::VotingEscrowAdditionError19)
                .checked_div(2.into())
                .unwrap_or_revert_with(Error::VotingEscrowAdditionError20);
            if UserPointHistory::instance().get(&addr, &mid).blk <= block {
                min = mid;
            } else {
                max = mid
                    .checked_sub(1.into())
                    .unwrap_or_revert_with(Error::VotingEscrowSubtractionError14);
            }
        }
        let mut upoint: Point = UserPointHistory::instance().get(&addr, &min);
        let max_epoch: U256 = get_epoch();
        let epoch: U256 = self._find_block_epoch(block, max_epoch);
        let point_0: Point = PointHistory::instance().get(&epoch);
        let mut d_block: U256 = 0.into();
        let mut d_t: U256 = 0.into();
        if epoch < max_epoch {
            let point_1: Point = PointHistory::instance().get(
                &epoch
                    .checked_add(1.into())
                    .unwrap_or_revert_with(Error::VotingEscrowAdditionError21),
            );
            d_block = point_1
                .blk
                .checked_sub(point_0.blk)
                .unwrap_or_revert_with(Error::VotingEscrowSubtractionError32);
            d_t = point_1
                .ts
                .checked_sub(point_0.ts)
                .unwrap_or_revert_with(Error::VotingEscrowSubtractionError15);
        } else {
            d_block = U256::from(block_number())
                .checked_sub(point_0.blk)
                .unwrap_or_revert_with(Error::VotingEscrowSubtractionError33);
            d_t = U256::from(u64::from(get_blocktime()))
                .checked_sub(point_0.ts)
                .unwrap_or_revert_with(Error::VotingEscrowSubtractionError16);
        }
        let mut block_time: U256 = point_0.ts;
        if d_block != 0.into() {
            block_time = block_time
                .checked_add(
                    d_t.checked_mul(
                        block
                            .checked_sub(point_0.blk)
                            .unwrap_or_revert_with(Error::VotingEscrowSubtractionError17),
                    )
                    .unwrap_or_revert_with(Error::VotingEscrowMultiplicationError8)
                    .checked_div(d_block)
                    .unwrap_or_revert_with(Error::VotingEscrowDivisionError6),
                )
                .unwrap_or_revert_with(Error::VotingEscrowAdditionError22);
        }
        upoint.set_bias(
            upoint
                .bias()
                .checked_sub(
                    upoint
                        .slope()
                        .checked_mul(convert(block_time, upoint.ts))
                        .unwrap_or_revert_with(Error::VotingEscrowMultiplicationError9),
                )
                .unwrap_or_revert_with(Error::VotingEscrowSubtractionError18),
        );
        if upoint.bias() >= 0.into() {
            upoint.bias().try_into().unwrap()
        } else {
            0.into()
        }
    }

    /// @notice Calculate total voting power at some point in the past
    /// @param point The point (bias/slope) to start search from
    /// @param t Time to calculate the total voting power at
    /// @return Total voting power at that time
    #[inline(always)]
    fn _supply_at(&self, point: Point, t: U256) -> U256 {
        let mut last_point: Point = point;
        let mut t_i: U256 = last_point
            .ts
            .checked_div(WEEK)
            .unwrap_or_revert_with(Error::VotingEscrowDivisionError7)
            .checked_mul(WEEK)
            .unwrap_or_revert_with(Error::VotingEscrowSubtractionError19);
        for _ in 0..255 {
            t_i = t_i
                .checked_add(WEEK)
                .unwrap_or_revert_with(Error::VotingEscrowAdditionError23);
            let mut d_slope: i128 = 0.into();
            if t_i > t {
                t_i = t;
            } else {
                d_slope = SlopeChanges::instance().get(&t_i);
            }
            last_point.set_bias(
                last_point
                    .bias()
                    .checked_sub(
                        last_point
                            .slope()
                            .checked_mul(convert(t_i, last_point.ts))
                            .unwrap_or_revert_with(Error::VotingEscrowMultiplicationError10),
                    )
                    .unwrap_or_revert_with(Error::VotingEscrowSubtractionError20),
            );
            if t_i == t {
                break;
            }
            last_point.set_slope(
                last_point
                    .slope()
                    .checked_add(d_slope)
                    .unwrap_or_revert_with(Error::VotingEscrowAdditionError24),
            );
            last_point.ts = t_i;
        }
        if last_point.bias() < 0.into() {
            last_point.set_bias(0);
        }
        last_point.bias().try_into().unwrap()
    }

    #[inline(always)]
    fn total_supply(&self, t: Option<U256>) -> U256 {
        let t: U256 = match t {
            Some(val) => val,
            None => {
                let blocktime: u64 = runtime::get_blocktime().into();
                blocktime.into()
            }
        };

        let epoch: U256 = get_epoch();
        let last_point: Point = PointHistory::instance().get(&epoch);
        self._supply_at(last_point, t)
    }

    #[inline(always)]
    fn total_supply_at(&self, block: U256) -> U256 {
        if block > block_number().into() {
            runtime::revert(ApiError::from(Error::VotingEscrowInvalidBlockNumber2));
        }
        let epoch: U256 = get_epoch();
        let target_epoch: U256 = self._find_block_epoch(block, epoch);
        let point: Point = PointHistory::instance().get(&target_epoch);
        let mut dt: U256 = 0.into();
        if target_epoch < epoch {
            let point_next: Point = PointHistory::instance().get(
                &target_epoch
                    .checked_add(1.into())
                    .unwrap_or_revert_with(Error::VotingEscrowAdditionError25),
            );
            if point.blk != point_next.blk {
                dt = block
                    .checked_sub(point.blk)
                    .unwrap_or_revert_with(Error::VotingEscrowSubtractionError21)
                    .checked_mul(
                        point_next
                            .ts
                            .checked_sub(point.ts)
                            .unwrap_or_revert_with(Error::VotingEscrowSubtractionError22),
                    )
                    .unwrap_or_revert_with(Error::VotingEscrowMultiplicationError11)
                    .checked_div(
                        point_next
                            .blk
                            .checked_sub(point.blk)
                            .unwrap_or_revert_with(Error::VotingEscrowSubtractionError23),
                    )
                    .unwrap_or_revert_with(Error::VotingEscrowDivisionError8);
            }
        } else if point.blk != block_number().into() {
            dt = (block
                .checked_sub(point.blk)
                .unwrap_or_revert_with(Error::VotingEscrowSubtractionError34))
            .checked_mul(
                U256::from(u64::from(get_blocktime()))
                    .checked_sub(point.ts)
                    .unwrap_or_revert_with(Error::VotingEscrowSubtractionError24),
            )
            .unwrap_or_revert_with(Error::VotingEscrowMultiplicationError12)
            .checked_div(
                U256::from(block_number())
                    .checked_sub(point.blk)
                    .unwrap_or_revert_with(Error::VotingEscrowSubtractionError35),
            )
            .unwrap_or_revert_with(Error::VotingEscrowDivisionError12);
        }
        // Now dt contains info on how far are we beyond point
        self._supply_at(
            point,
            point
                .ts
                .checked_add(dt)
                .unwrap_or_revert_with(Error::VotingEscrowAdditionError26),
        )
    }

    #[inline(always)]
    fn change_controller(&self, new_controller: Key) {
        if self.get_caller() != get_controller() {
            runtime::revert(ApiError::from(Error::VotingEscrowNotController));
        }
        set_controller(new_controller);
    }

    fn emit(&self, voting_escrow_event: &VotingEscrowEvent) {
        match voting_escrow_event {
            VotingEscrowEvent::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", voting_escrow_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            VotingEscrowEvent::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", voting_escrow_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            VotingEscrowEvent::Deposit {
                provider,
                value,
                locktime,
                _type,
                ts,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", voting_escrow_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                event.insert("locktime", locktime.to_string());
                event.insert("_type", _type.to_string());
                event.insert("ts", ts.to_string());
                storage::new_uref(event);
            }
            VotingEscrowEvent::Withdraw {
                provider,
                value,
                ts,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", voting_escrow_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                event.insert("ts", ts.to_string());
                storage::new_uref(event);
            }
            VotingEscrowEvent::Supply {
                prev_supply,
                supply,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", voting_escrow_event.type_name());
                event.insert("prev_supply", prev_supply.to_string());
                event.insert("supply", supply.to_string());
                storage::new_uref(event);
            }
        };
    }
}
