use crate::{data::*, event::FeeDistributorEvent};
use alloc::vec::Vec;
use alloc::{collections::BTreeMap, string::ToString};
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

#[allow(clippy::too_many_arguments)]
pub trait FEEDISTRIBUTOR<Storage: ContractStorage>: ContractContext<Storage> {
    /// @notice Contract constructor
    /// @param _voting_escrow VotingEscrow contract address
    /// @param _start_time Epoch time for fee distribution to start
    /// @param _token Fee token address (3CRV)
    /// @param _admin Admin address
    /// @param _emergency_return Address to transfer `_token` balance to if this contract is killed
    fn init(
        &self,
        voting_escrow: Key,
        start_time: U256,
        token: Key,
        admin: Key,
        emergency_return: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        TimeCursorOf::init();
        UserEpochOf::init();
        TokensPerWeek::init();
        VeSupply::init();
        let t: U256 = start_time
            .checked_div(WEEK)
            .unwrap_or_revert_with(Error::FeeDistributorDivisionError1)
            .checked_mul(WEEK)
            .unwrap_or_revert_with(Error::FeeDistributorMultiplicationError1);
        set_start_time(t);
        set_last_token_time(t);
        set_time_cursor(t);
        set_token(token);
        set_voting_escrow(voting_escrow);
        set_admin(admin);
        set_emergency_return(emergency_return);
        set_contract_hash(contract_hash);
        set_package_hash(package_hash);
    }

    #[allow(unused_assignments)]
    fn _checkpoint_token(&self) {
        let token_balance: U256 = runtime::call_versioned_contract(
            get_token().into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "address" => Address::from(Key::from(get_package_hash()))
            },
        );
        let to_distribute: U256 = token_balance
            .checked_sub(get_token_last_balance())
            .unwrap_or_revert_with(Error::FeeDistributorSubtractionError1);
        set_token_last_balance(token_balance);
        let mut t: U256 = get_last_token_time();
        let since_last: U256 = U256::from(u64::from(get_blocktime()))
            .checked_sub(t)
            .unwrap_or_revert_with(Error::FeeDistributorSubtractionError2);
        set_last_token_time(U256::from(u64::from(get_blocktime())));
        let mut this_week: U256 = t
            .checked_div(WEEK)
            .unwrap_or_revert_with(Error::FeeDistributorDivisionError2)
            .checked_mul(WEEK)
            .unwrap_or_revert_with(Error::FeeDistributorMultiplicationError2);
        let mut next_week: U256 = 0.into();
        for _ in 0..20 {
            next_week = this_week
                .checked_add(WEEK)
                .unwrap_or_revert_with(Error::FeeDistributorAdditionError19);
            if U256::from(u64::from(get_blocktime())) < next_week {
                if since_last == 0.into() && U256::from(u64::from(get_blocktime())) == t {
                    TokensPerWeek::instance().set(
                        &this_week,
                        TokensPerWeek::instance()
                            .get(&this_week)
                            .checked_add(to_distribute)
                            .unwrap_or_revert_with(Error::FeeDistributorAdditionError1),
                    );
                } else {
                    TokensPerWeek::instance().set(
                        &this_week,
                        TokensPerWeek::instance()
                            .get(&this_week)
                            .checked_add(to_distribute)
                            .unwrap_or_revert_with(Error::FeeDistributorAdditionError2)
                            .checked_mul(
                                U256::from(u64::from(get_blocktime()))
                                    .checked_sub(t)
                                    .unwrap_or_revert_with(Error::FeeDistributorSubtractionError3),
                            )
                            .unwrap_or_revert_with(Error::FeeDistributorMultiplicationError3)
                            .checked_div(since_last)
                            .unwrap_or_revert_with(Error::FeeDistributorDivisionError3),
                    );
                }
                break;
            } else if since_last == 0.into() && next_week == t {
                TokensPerWeek::instance().set(
                    &this_week,
                    TokensPerWeek::instance()
                        .get(&this_week)
                        .checked_add(to_distribute)
                        .unwrap_or_revert_with(Error::FeeDistributorAdditionError3),
                );
            } else {
                TokensPerWeek::instance().set(
                    &this_week,
                    TokensPerWeek::instance()
                        .get(&this_week)
                        .checked_add(to_distribute)
                        .unwrap_or_revert_with(Error::FeeDistributorAdditionError4)
                        .checked_mul(
                            next_week
                                .checked_sub(t)
                                .unwrap_or_revert_with(Error::FeeDistributorSubtractionError17),
                        )
                        .unwrap_or_revert_with(Error::FeeDistributorMultiplicationError4)
                        .checked_div(since_last)
                        .unwrap_or_revert_with(Error::FeeDistributorDivisionError4),
                );
            }
            t = next_week;
            this_week = next_week;
        }
        FEEDISTRIBUTOR::emit(
            self,
            &FeeDistributorEvent::CheckpointToken {
                time: U256::from(u64::from(get_blocktime())),
                tokens: to_distribute,
            },
        );
    }

    /// @notice Update the token checkpoint
    /// @dev Calculates the total number of tokens to be distributed in a given week.
    ///     During setup for the initial distribution this function is only callable
    ///     by the contract owner. Beyond initial distro, it can be enabled for anyone to call.
    fn checkpoint_token(&self) {
        if !((self.get_caller() == get_admin())
            || (get_can_checkpoint_token()
                && (U256::from(u64::from(get_blocktime()))
                    > get_last_token_time()
                        .checked_add(TOKEN_CHECKPOINT_DEADLINE)
                        .unwrap_or_revert_with(Error::FeeDistributorAdditionError5))))
        {
            runtime::revert(ApiError::from(
                Error::FeeDistributorInvalidTokenCheckpointUpdate,
            ))
        }
        self._checkpoint_token();
    }

    fn _find_timestamp_epoch(&self, ve: Key, timestamp: U256) -> U256 {
        let mut min: U256 = 0.into();
        let mut max: U256 = runtime::call_versioned_contract(
            ve.into_hash().unwrap_or_revert().into(),
            None,
            "epoch",
            runtime_args! {},
        );
        for _ in 0..128 {
            if min >= max {
                break;
            }
            let mid: U256 = (min
                .checked_add(max)
                .unwrap_or_revert_with(Error::FeeDistributorAdditionError6)
                .checked_add(2.into())
                .unwrap_or_revert_with(Error::FeeDistributorAdditionError7))
            .checked_div(2.into())
            .unwrap_or_revert_with(Error::FeeDistributorDivisionError5);
            let pt: Point = runtime::call_versioned_contract(
                ve.into_hash().unwrap_or_revert().into(),
                None,
                "point_history",
                runtime_args! {
                    "epoch" => mid
                },
            );
            if pt.ts <= timestamp {
                min = mid;
            } else {
                max = mid
                    .checked_sub(1.into())
                    .unwrap_or_revert_with(Error::FeeDistributorDivisionError12);
            }
        }
        min
    }

    fn _find_timestamp_user_epoch(
        &self,
        ve: Key,
        user: Key,
        timestamp: U256,
        max_user_epoch: U256,
    ) -> U256 {
        let mut min: U256 = 0.into();
        let mut max: U256 = max_user_epoch;
        for _ in 0..128 {
            if min >= max {
                break;
            }
            let mid: U256 = (min
                .checked_add(max)
                .unwrap_or_revert_with(Error::FeeDistributorAdditionError8)
                .checked_add(2.into())
                .unwrap_or_revert_with(Error::FeeDistributorAdditionError9))
            .checked_div(2.into())
            .unwrap_or_revert_with(Error::FeeDistributorDivisionError6);
            let pt: Point = runtime::call_versioned_contract(
                ve.into_hash().unwrap_or_revert().into(),
                None,
                "user_point_history",
                runtime_args! {
                    "user" => user,
                    "user_epoch" => mid
                },
            );
            if pt.ts <= timestamp {
                min = mid;
            } else {
                max = mid
                    .checked_sub(1.into())
                    .unwrap_or_revert_with(Error::FeeDistributorSubtractionError5);
            }
        }
        min
    }

    /// @notice Get the veCRV balance for `_user` at `_timestamp`
    /// @param _user Address to query balance for
    /// @param _timestamp Epoch time
    /// @return uint256 veCRV balance
    fn ve_for_at(&self, user: Key, timestamp: U256) -> U256 {
        let ve: Key = get_voting_escrow();
        let max_user_epoch: U256 = runtime::call_versioned_contract(
            ve.into_hash().unwrap_or_revert().into(),
            None,
            "user_point_epoch",
            runtime_args! {
                "user" => user
            },
        );
        let epoch: U256 = self._find_timestamp_user_epoch(ve, user, timestamp, max_user_epoch);
        let pt: Point = runtime::call_versioned_contract(
            ve.into_hash().unwrap_or_revert().into(),
            None,
            "user_point_history",
            runtime_args! {
                "user" => user,
                "user_epoch" => epoch
            },
        );
        U256::max(
            (tuple_to_i128(pt.bias)
                .checked_sub(
                    tuple_to_i128(pt.slope)
                        .checked_mul(
                            timestamp
                                .checked_sub(pt.ts)
                                .unwrap_or_revert_with(Error::FeeDistributorSubtractionError7)
                                .to_string()
                                .parse()
                                .unwrap(),
                        )
                        .unwrap_or_revert_with(Error::FeeDistributorMultiplicationError5),
                )
                .unwrap_or_revert_with(Error::FeeDistributorSubtractionError6))
            .into(),
            0.into(),
        )
    }

    fn _checkpoint_total_supply(&self) {
        let ve: Key = get_voting_escrow();
        let mut t: U256 = get_time_cursor();
        let rounded_timestamp: U256 = U256::from(u64::from(get_blocktime()))
            .checked_div(WEEK)
            .unwrap_or_revert_with(Error::FeeDistributorDivisionError7)
            .checked_mul(WEEK)
            .unwrap_or_revert_with(Error::FeeDistributorMultiplicationError6);
        let () = runtime::call_versioned_contract(
            ve.into_hash().unwrap_or_revert().into(),
            None,
            "checkpoint",
            runtime_args! {},
        );
        for _ in 0..20 {
            if t > rounded_timestamp {
                break;
            } else {
                let epoch: U256 = self._find_timestamp_epoch(ve, t);
                let pt: Point = runtime::call_versioned_contract(
                    ve.into_hash().unwrap_or_revert().into(),
                    None,
                    "point_history",
                    runtime_args! {
                        "epoch" => epoch
                    },
                );
                let mut dt: i128 = 0.into();
                if t > pt.ts {
                    // If the point is at 0 epoch, it can actually be earlier than the first deposit
                    // Then make dt 0
                    dt = t
                        .checked_sub(pt.ts)
                        .unwrap_or_revert_with(Error::FeeDistributorSubtractionError8)
                        .to_string()
                        .parse()
                        .unwrap();
                }
                VeSupply::instance().set(
                    &t,
                    i128::max(
                        tuple_to_i128(pt.bias)
                            .checked_sub(
                                tuple_to_i128(pt.slope)
                                    .checked_mul(dt)
                                    .unwrap_or_revert_with(
                                        Error::FeeDistributorMultiplicationError12,
                                    ),
                            )
                            .unwrap_or_revert_with(Error::FeeDistributorSubtractionError9),
                        0.into(),
                    )
                    .into(),
                );
            }
            t = t
                .checked_add(WEEK)
                .unwrap_or_revert_with(Error::FeeDistributorAdditionError10);
        }
        set_time_cursor(t);
    }

    /// @notice Update the veCRV total supply checkpoint
    /// @dev The checkpoint is also updated by the first claimant each
    ///     new epoch week. This function may be called independently
    ///     of a claim, to reduce claiming gas costs.
    fn checkpoint_total_supply(&self) {
        self._checkpoint_total_supply();
    }

    #[allow(unused_assignments)]
    fn _claim(&self, addr: Key, ve: Key, last_token_time: U256) -> U256 {
        // Minimal user_epoch is 0 (if user had no point)
        let mut user_epoch: U256 = 0.into();
        let mut to_distribute: U256 = 0.into();
        let max_user_epoch: U256 = runtime::call_versioned_contract(
            ve.into_hash().unwrap_or_revert().into(),
            None,
            "user_point_epoch",
            runtime_args! {
                "user" => addr
            },
        );
        let start_time: U256 = get_start_time();
        if max_user_epoch == 0.into() {
            // No lock = no fees
            return 0.into();
        }
        let mut week_cursor: U256 = TimeCursorOf::instance().get(&addr);
        if week_cursor == 0.into() {
            // Need to do the initial binary search
            user_epoch = self._find_timestamp_user_epoch(ve, addr, start_time, max_user_epoch);
        } else {
            user_epoch = UserEpochOf::instance().get(&addr);
        }
        if user_epoch == 0.into() {
            user_epoch = 1.into();
        }
        let mut user_point: Point = runtime::call_versioned_contract(
            ve.into_hash().unwrap_or_revert().into(),
            None,
            "user_point_history",
            runtime_args! {
                "user" => addr,
                "user_epoch" => user_epoch
            },
        );
        if week_cursor == 0.into() {
            week_cursor = (user_point
                .ts
                .checked_add(WEEK)
                .unwrap_or_revert_with(Error::FeeDistributorAdditionError11)
                .checked_sub(1.into()))
            .unwrap_or_revert_with(Error::FeeDistributorSubtractionError11)
            .checked_div(WEEK)
            .unwrap_or_revert_with(Error::FeeDistributorDivisionError8)
            .checked_mul(WEEK)
            .unwrap_or_revert_with(Error::FeeDistributorMultiplicationError7);
        }
        if week_cursor >= last_token_time {
            return 0.into();
        }
        if week_cursor < start_time {
            week_cursor = start_time;
        }
        let mut old_user_point: Point = Point::default();
        // Iterate over weeks
        for _ in 0..50 {
            if week_cursor >= last_token_time {
                break;
            }
            if week_cursor >= user_point.ts && user_epoch <= max_user_epoch {
                user_epoch = user_epoch
                    .checked_add(1.into())
                    .unwrap_or_revert_with(Error::FeeDistributorAdditionError12);
                old_user_point = user_point;
                if user_epoch > max_user_epoch {
                    user_point = Point::default();
                } else {
                    user_point = runtime::call_versioned_contract(
                        ve.into_hash().unwrap_or_revert().into(),
                        None,
                        "user_point_history",
                        runtime_args! {
                            "user" => addr,
                            "user_epoch" => user_epoch
                        },
                    );
                }
            } else {
                // Calc
                // + i * 2 is for rounding errors
                let dt: i128 = week_cursor
                    .checked_sub(old_user_point.ts)
                    .unwrap_or_revert_with(Error::FeeDistributorSubtractionError12)
                    .to_string()
                    .parse()
                    .unwrap();
                let balance_of: U256 = U256::max(
                    tuple_to_i128(old_user_point.bias)
                        .checked_sub(
                            dt.checked_mul(tuple_to_i128(old_user_point.slope))
                                .unwrap_or_revert_with(Error::FeeDistributorMultiplicationError8),
                        )
                        .unwrap_or_revert_with(Error::FeeDistributorSubtractionError13)
                        .into(),
                    0.into(),
                );
                if balance_of == 0.into() && user_epoch > max_user_epoch {
                    break;
                }
                if balance_of > 0.into() {
                    to_distribute = to_distribute
                        .checked_add(
                            balance_of
                                .checked_mul(
                                    TokensPerWeek::instance()
                                        .get(&week_cursor)
                                        .checked_div(VeSupply::instance().get(&week_cursor))
                                        .unwrap_or_revert_with(Error::FeeDistributorDivisionError9),
                                )
                                .unwrap_or_revert_with(Error::FeeDistributorMultiplicationError9),
                        )
                        .unwrap_or_revert_with(Error::FeeDistributorAdditionError13);
                }
                week_cursor = week_cursor
                    .checked_add(WEEK)
                    .unwrap_or_revert_with(Error::FeeDistributorAdditionError14);
            }
        }
        user_epoch = U256::min(
            max_user_epoch,
            user_epoch
                .checked_sub(1.into())
                .unwrap_or_revert_with(Error::FeeDistributorSubtractionError14),
        );
        UserEpochOf::instance().set(&addr, user_epoch);
        TimeCursorOf::instance().set(&addr, week_cursor);
        FEEDISTRIBUTOR::emit(
            self,
            &FeeDistributorEvent::Claimed {
                recipient: addr,
                amount: to_distribute,
                claim_epoch: user_epoch,
                max_epoch: max_user_epoch,
            },
        );
        to_distribute
    }

    /// @notice Claim fees for `_addr`
    /// @dev Each call to claim look at a maximum of 50 user veCRV points.
    ///     For accounts with many veCRV related actions, this function
    ///     may need to be called more than once to claim all available
    ///     fees. In the `Claimed` event that fires, if `claim_epoch` is
    ///     less than `max_epoch`, the account may claim again.
    /// @param _addr Address to claim fees for
    /// @return uint256 Amount of fees claimed in the call
    fn claim(&self, addr: Option<Key> /*self.get_caller()*/) -> U256 {
        if get_lock() {
            runtime::revert(ApiError::from(Error::FeeDistributorIsLocked1));
        }
        set_lock(true);
        let _addr: Key = if let Some(..) = addr {
            addr.unwrap()
        } else {
            self.get_caller()
        };
        if get_is_killed() {
            runtime::revert(ApiError::from(Error::FeeDistributorKilled1));
        }
        if U256::from(u64::from(get_blocktime())) >= get_time_cursor() {
            self._checkpoint_total_supply();
        }
        let mut last_token_time: U256 = get_last_token_time();
        if get_can_checkpoint_token()
            && (U256::from(u64::from(get_blocktime()))
                > last_token_time
                    .checked_add(TOKEN_CHECKPOINT_DEADLINE)
                    .unwrap_or_revert_with(Error::FeeDistributorAdditionError15))
        {
            self._checkpoint_token();
            set_last_token_time(U256::from(u64::from(get_blocktime())));
        }
        last_token_time = last_token_time
            .checked_div(WEEK)
            .unwrap_or_revert_with(Error::FeeDistributorDivisionError10)
            .checked_mul(WEEK)
            .unwrap_or_revert_with(Error::FeeDistributorMultiplicationError10);
        let amount: U256 = self._claim(_addr, get_voting_escrow(), last_token_time);
        if amount != 0.into() {
            let token: Key = get_token();
            let () = runtime::call_versioned_contract(
                token.into_hash().unwrap_or_revert().into(),
                None,
                "transfer",
                runtime_args! {
                    "recipient" =>Address::from(_addr),
                    "amount" => amount
                },
            );
            set_token_last_balance(
                get_token_last_balance()
                    .checked_sub(amount)
                    .unwrap_or_revert_with(Error::FeeDistributorSubtractionError15),
            );
        }
        set_lock(false);
        amount
    }

    /// @notice Make multiple fee claims in a single call
    /// @dev Used to claim for many accounts at once, or to make
    ///     multiple claims for the same address when that address
    ///     has significant veCRV history
    /// @param _receivers List of addresses to claim for. Claiming terminates at the first `ZERO_ADDRESS`.
    /// @return bool success
    fn claim_many(&self, receivers: Vec<Key>) -> bool {
        if get_lock() {
            runtime::revert(ApiError::from(Error::FeeDistributorIsLocked2));
        }
        set_lock(true);
        if get_is_killed() {
            runtime::revert(ApiError::from(Error::FeeDistributorKilled2));
        }
        if U256::from(u64::from(get_blocktime())) >= get_time_cursor() {
            self._checkpoint_total_supply();
        }
        let mut last_token_time: U256 = get_last_token_time();
        if get_can_checkpoint_token()
            && (U256::from(u64::from(get_blocktime()))
                > last_token_time
                    .checked_add(TOKEN_CHECKPOINT_DEADLINE)
                    .unwrap_or_revert_with(Error::FeeDistributorAdditionError16))
        {
            self._checkpoint_token();
            last_token_time = U256::from(u64::from(get_blocktime()));
        }
        last_token_time = last_token_time
            .checked_div(WEEK)
            .unwrap_or_revert_with(Error::FeeDistributorDivisionError11)
            .checked_mul(WEEK)
            .unwrap_or_revert_with(Error::FeeDistributorMultiplicationError11);
        let voting_escrow: Key = get_voting_escrow();
        let token: Key = get_token();
        let mut total: U256 = 0.into();
        for addr in receivers {
            if addr == zero_address() {
                break;
            }
            let amount: U256 = self._claim(addr, voting_escrow, last_token_time);
            if amount != 0.into() {
                let () = runtime::call_versioned_contract(
                    token.into_hash().unwrap_or_revert().into(),
                    None,
                    "transfer",
                    runtime_args! {
                        "recipient" => Address::from(addr),
                        "amount" => amount
                    },
                );
                total = total
                    .checked_add(amount)
                    .unwrap_or_revert_with(Error::FeeDistributorAdditionError17);
            }
        }
        if total != 0.into() {
            set_token_last_balance(
                get_token_last_balance()
                    .checked_sub(total)
                    .unwrap_or_revert_with(Error::FeeDistributorSubtractionError16),
            );
        }
        set_lock(false);
        true
    }

    /// @notice Receive 3CRV into the contract and trigger a token checkpoint
    /// @param _coin Address of the coin being received (must be 3CRV)
    /// @return bool success
    fn burn(&self, coin: Key) -> bool {
        if coin != get_token() {
            runtime::revert(ApiError::from(Error::FeeDistributorInvalidCoin1));
        }
        if get_is_killed() {
            runtime::revert(ApiError::from(Error::FeeDistributorKilled3));
        }
        let amount: U256 = runtime::call_versioned_contract(
            coin.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "address" => Address::from(self.get_caller())
            },
        );
        if amount != 0.into() {
            let () = runtime::call_versioned_contract(
                coin.into_hash().unwrap_or_revert().into(),
                None,
                "transfer_from",
                runtime_args! {
                    "owner" => Address::from(self.get_caller()),
                    "recipient" => Address::from(Key::from(get_package_hash())),
                    "amount" => amount
                },
            );
            if get_can_checkpoint_token()
                && (U256::from(u64::from(get_blocktime()))
                    > get_last_token_time()
                        .checked_add(TOKEN_CHECKPOINT_DEADLINE)
                        .unwrap_or_revert_with(Error::FeeDistributorAdditionError18))
            {
                self._checkpoint_token();
            }
        }
        true
    }

    /// @notice Commit transfer of ownership
    /// @param _addr New admin address
    fn commit_admin(&self, addr: Key) {
        if self.get_caller() != get_admin() {
            runtime::revert(ApiError::from(Error::FeeDistributorAccessDenied));
        }
        set_future_admin(addr);
        FEEDISTRIBUTOR::emit(self, &FeeDistributorEvent::CommitAdmin { admin: addr });
    }

    /// @notice Apply transfer of ownership
    fn apply_admin(&self) {
        if self.get_caller() != get_admin() {
            runtime::revert(ApiError::from(Error::FeeDistributorInvalidAdmin1));
        }
        if get_future_admin() == zero_address() {
            runtime::revert(ApiError::from(Error::FeeDistributorZeroFutureAdmin));
        }
        let future_admin: Key = get_future_admin();
        set_admin(future_admin);
        FEEDISTRIBUTOR::emit(
            self,
            &FeeDistributorEvent::ApplyAdmin {
                admin: future_admin,
            },
        );
    }

    /// @notice Toggle permission for checkpointing by any account
    fn toggle_allow_checkpoint_token(&self) {
        if self.get_caller() != get_admin() {
            runtime::revert(ApiError::from(Error::FeeDistributorInvalidAdmin2));
        }
        let flag: bool = !get_can_checkpoint_token();
        set_can_checkpoint_token(flag);
        FEEDISTRIBUTOR::emit(
            self,
            &FeeDistributorEvent::ToggleAllowCheckpointToken { toggle_flag: flag },
        );
    }

    /// @notice Kill the contract
    /// @dev Killing transfers the entire 3CRV balance to the emergency return address
    ///     and blocks the ability to claim or burn. The contract cannot be unkilled.
    fn kill_me(&self) {
        if self.get_caller() != get_admin() {
            runtime::revert(ApiError::from(Error::FeeDistributorInvalidAdmin3));
        }
        set_is_killed(true);
        let token: Key = get_token();
        let balance: U256 = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "address" => Address::from(Key::from(get_package_hash()))
            },
        );
        let () = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => Address::from(get_emergency_return()),
                "amount" => balance
            },
        );
    }

    /// @notice Recover ERC20 tokens from this contract
    /// @dev Tokens are sent to the emergency return address.
    /// @param _coin Token address
    /// @return bool success
    fn recover_balance(&self, coin: Key) -> bool {
        if self.get_caller() != get_admin() {
            runtime::revert(ApiError::from(Error::FeeDistributorInvalidAdmin4));
        }
        if coin != get_token() {
            runtime::revert(ApiError::from(Error::FeeDistributorInvalidCoin2));
        }
        let amount: U256 = runtime::call_versioned_contract(
            coin.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "address" => Address::from(Key::from(get_package_hash()))
            },
        );
        let () = runtime::call_versioned_contract(
            coin.into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => Address::from(get_emergency_return()),
                "amount" => amount
            },
        );
        true
    }

    fn emit(&self, fee_distributor_event: &FeeDistributorEvent) {
        match fee_distributor_event {
            FeeDistributorEvent::CommitAdmin { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", fee_distributor_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            FeeDistributorEvent::ApplyAdmin { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", fee_distributor_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            FeeDistributorEvent::ToggleAllowCheckpointToken { toggle_flag } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", fee_distributor_event.type_name());
                event.insert("toggle_flag", toggle_flag.to_string());
                storage::new_uref(event);
            }
            FeeDistributorEvent::CheckpointToken { time, tokens } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", fee_distributor_event.type_name());
                event.insert("time", time.to_string());
                event.insert("tokens", tokens.to_string());
                storage::new_uref(event);
            }
            FeeDistributorEvent::Claimed {
                recipient,
                amount,
                claim_epoch,
                max_epoch,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", fee_distributor_event.type_name());
                event.insert("recipient", recipient.to_string());
                event.insert("amount", amount.to_string());
                event.insert("claim_epoch", claim_epoch.to_string());
                event.insert("max_epoch", max_epoch.to_string());
                storage::new_uref(event);
            }
        };
    }
}
