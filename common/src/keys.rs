// Common Keys
pub const SESSION_CODE_WASM: &str = "session-code.wasm";
pub const FD_SESSION_CODE_WASM: &str = "fee-distributor-session-code.wasm";
pub const SESSION_CODE_NAME: &str = "session_code";
pub const TEST_SESSION_CODE_WASM: &str = "test-session-code.wasm";
pub const TEST_SESSION_CODE_NAME: &str = "test_session_code";
pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_CONTRACT_PACKAGE_HASH: &str = "self_contract_package_hash";
pub const VESTING_ESCROW_SIMPLE_CONTRACT_HASH: &str = "vesting_escrow_simple_contract_hash";
pub const VESTING_ESCROW_SIMPLE_CONTRACT_PACKAGE_HASH: &str =
    "vesting_escrow_simple_contract_package_hash";
pub const EVENT_TYPE: &str = "event_type";
pub const GAUGES: &str = "gauges";
pub const RESULT: &str = "result";
pub const ADMIN: &str = "admin";
pub const FUTURE_ADMIN: &str = "future_admin";
pub const LOCK: &str = "lock";
pub const BALANCE_OF: &str = "balance_of";
pub const TOTAL_SUPPLY: &str = "total_supply";
pub const INIT_SUPPLY: &str = "init_supply";
pub const MINTER: &str = "minter";
pub const START_TIME: &str = "start_time";
pub const VOTING_ESCROW: &str = "voting_escrow";
pub const TOKEN: &str = "token";
pub const IS_KILLED: &str = "is_killed";
pub const NAME: &str = "name";
pub const CONTROLLER: &str = "controller";
pub const REWARD_COUNT: &str = "reward_count";
pub const SYMBOL: &str = "symbol";
pub const DECIMALS: &str = "decimals";
pub const LP_TOKEN: &str = "lp_token";
pub const CRV_TOKEN: &str = "crv_token";
pub const REWARDED_TOKEN: &str = "rewarded_token";
pub const REWARD_INTEGRAL: &str = "reward_integral";
pub const REWARD_INTEGRAL_FOR: &str = "reward_integral_for";
pub const APPROVED_TO_DEPOSIT: &str = "approved_to_deposit";
pub const INITIAL_LOCKED_DICT: &str = "initial_locked";
pub const TOTAL_CLAIMED_DICT: &str = "total_claimed";
pub const DISABLED_AT_DICT: &str = "disabled_at";
pub const FUND_ADMINS_DICT: &str = "fund_admins";
pub const CAN_DISABLE: &str = "can_disable";
pub const FUND_ADMINS_ENABLED: &str = "fund_admins_enabled";
pub const END_TIME: &str = "end_time";
// Voting Escrow
pub const GET_LAST_USER_SLOPE: &str = "get_last_user_slope";
pub const USER_POINT_HISTORY_TS: &str = "user_point_history_ts";
pub const LOCKED_END: &str = "locked_end";
pub const BALANCE_OF_AT: &str = "balance_of_at";
pub const VE_BALANCE_OF: &str = "ve_balance_of";
pub const TOTAL_SUPPLY_AT: &str = "total_supply_at";
pub const VE_TOTAL_SUPPLY: &str = "ve_total_supply";
// Fee Distributor
pub const VE_FOR_AT: &str = "ve_for_at";
pub const CLAIM: &str = "claim";
pub const CLAIM_MANY: &str = "claim_many";
pub const BURN: &str = "burn";
pub const RECOVER_BALANCE: &str = "recover_balance";
pub const FUTURE_EPOCH_TIME_WRITE: &str = "future_epoch_time_write";
pub const START_EPOCH_TIME_WRITE: &str = "start_epoch_time_write";
pub const AVAILABLE_SUPPLY: &str = "available_supply";
pub const MINT: &str = "mint";
pub const MINTABLE_IN_TIMEFRAME: &str = "mintable_in_timeframe";
// ERC-20 CRV
pub const ADMIN_WHITELIST: &str = "admin_whitelist";
pub const ALLOWANCE_CRV: &str = "allowance_crv";
pub const MINING_EPOCH: &str = "mining_epoch";
pub const IS_UPDATED: &str = "is_updated";
pub const START_EPOCH_TIME: &str = "start_epoch_time";
pub const RATE: &str = "rate";
pub const START_EPOCH_SUPPLY: &str = "start_epoch_supply";
// Fee Distributor
pub const TIME_CURSOR: &str = "time_cursor";
pub const LAST_TOKEN_TIME: &str = "last_token_time";
pub const TOTAL_RECEIVED: &str = "total_received";
pub const TOKEN_LAST_BALANCE: &str = "token_last_balance";
pub const CAN_CHECKPOINT_TOKEN: &str = "can_checkpoint_token";
pub const EMERGENCY_RETURN: &str = "emergency_return";
// Minter
pub const MINTED_DICT: &str = "minted";
pub const ALLOWED_TO_MINT_FOR_DICT: &str = "allowed_to_mint_for";
// Reward Only Gauge
pub const BALANCES_DICT: &str = "balances";
pub const NONCES_DICT: &str = "nonces";
pub const ALLOWANCES_DICT: &str = "allowances";
pub const REWARD_TOKENS_DICT: &str = "reward_tokens";
pub const REWARD_BALANCES_DICT: &str = "reward_balances";
pub const REWARDS_RECEIVER_DICT: &str = "reward_receiver";
pub const REWARD_INTEGRAL_DICT: &str = "reward_integral";
pub const REWARD_INTEGRAL_FOR_DICT: &str = "reward_integral_for";
pub const CLAIM_DATA_DICT: &str = "claim_data";
pub const CLAIM_SIG: &str = "claim_sig";
pub const REWARD_DATA_DICT: &str = "reward_data";
pub const METHOD: &str = "method";
pub const PARAMS: &str = "params";
// Vesting Escrow
pub const INITIAL_LOCKED_SUPPLY: &str = "initial_locked_supply";
pub const UNALLOCATED_SUPPLY: &str = "unallocated_supply";
pub const FUND_ADMINS_FUNDS: &str = "fund_admins_enabled";
// Vesting Escrow Factory
pub const TARGET: &str = "target";
pub const VESTING_ESCROW_SIMPLE_CONTRACT: &str = "vesting_escrow_simple_contract";
// Vesting Escrow Simple
pub const INITIAL_LOCKED_DICT_SUPPLY: &str = "initial_locked_supply";
// Voting Escrow
pub const SUPPLY: &str = "supply";
pub const TRANSFERS_ENABLED: &str = "transfers_enabled";
pub const VERSION: &str = "version";
pub const EPOCH: &str = "epoch";
// Curve Token V3
pub const CURVE_TOKEN_V3_SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const CURVE_TOKEN_V3_SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const CURVE_TOKEN_V3_RESULT: &str = "result";
pub const CURVE_TOKEN_V3_NAME: &str = "name";
pub const CURVE_TOKEN_V3_SYMBOL: &str = "symbol";
pub const CURVE_TOKEN_V3_DECIMALS: &str = "decimals";
pub const CURVE_TOKEN_V3_BALANCE_OF_DICT: &str = "balance_of";
pub const CURVE_TOKEN_V3_ALLOWANCES_DICT: &str = "allowances";
pub const CURVE_TOKEN_V3_TOTAL_SUPPLY: &str = "total_supply";
pub const CURVE_TOKEN_V3_CURVE: &str = "curve";
pub const CURVE_TOKEN_V3_MINTER: &str = "minter";
pub const BURN_FROM: &str = "burn_from";
// Gauge Controller
pub const GAUGE_TYPE_NAMES_DICT: &str = "gauge_type_names";
pub const GAUGE_TYPES_DICT: &str = "gauge_types_";
pub const VOTE_USER_SLOPES_DICT: &str = "vote_user_slopes";
pub const VOTE_USER_POWER_DICT: &str = "vote_user_power";
pub const LAST_USER_VOTE_DICT: &str = "last_user_vote";
pub const POINTS_WEIGHT_DICT: &str = "points_weight";
pub const CHANGES_WEIGHT_DICT: &str = "changes_weight";
pub const TIME_WEIGHT_DICT: &str = "time_weight";
pub const GAUGES_DICT: &str = "gauges";
pub const TIME_SUM_DICT: &str = "time_sum";
pub const POINTS_SUM_DICT: &str = "points_sum";
pub const CHANGES_SUM_DICT: &str = "changes_sum";
pub const POINTS_TOTAL_DICT: &str = "points_total";
pub const POINTS_TYPE_WEIGHT_DICT: &str = "points_type_weight";
pub const TIME_TYPE_WEIGHT_DICT: &str = "time_type_weight";
pub const OWNER: &str = "owner";
pub const TIME_TOTAL: &str = "time_total";
pub const N_GAUGE_TYPES: &str = "n_gauge_types";
pub const N_GAUGES: &str = "n_gauges";
pub const LAST_USER_VOTE: &str = "last_user_vote";
// Gauge Proxy
pub const OWNERSHIP_ADMIN: &str = "ownership_admin";
pub const EMERGENCY_ADMIN: &str = "emergency_admin";
pub const FUTURE_OWNERSHIP_ADMIN: &str = "future_ownership_admin";
pub const FUTURE_EMERGENCY_ADMIN: &str = "future_emergency_admin";
// Liquidity Gauge Reward
pub const PERIOD: &str = "period";
pub const FUTURE_EPOCH_TIME: &str = "future_epoch_time";
pub const WORKING_SUPPLY: &str = "working_supply";
pub const INFLATION_RATE: &str = "inflation_rate";
pub const REWARD_CONTRACT: &str = "reward_contract";
pub const IS_CLAIMING_REWARDS: &str = "is_claiming_rewards";
pub const WORKING_BALANCES: &str = "working_balances";
pub const PERIOD_TIMESTAMP: &str = "period_timestamp";
pub const INTEGRATE_INV_SUPPLY: &str = "integrate_inv_supply";
pub const INTEGRATE_INV_SUPPLY_OF: &str = "integrate_inv_supply_of";
pub const INTEGRATE_CHECKPOINT_OF: &str = "integrate_checkpoint_of";
pub const INTEGRATE_FRACTION: &str = "integrate_fraction";
pub const REWARDS_FOR: &str = "rewards_for";
pub const CLAIMED_REWARDS_FOR: &str = "claimed_rewards_for";
// Liquidity Gauge Reward Wrapper
pub const GAUGE: &str = "gauge";
pub const CRV_INTEGRAL: &str = "crv_integral";
pub const ALLOWANCES: &str = "allownances";
pub const CLAIMABLE_CRV: &str = "claimable_crv";
pub const CRV_INTEGRAL_FOR: &str = "crv_integral_for";
pub const CLAIMABLE_REWARDS: &str = "claimable_rewards";
pub const USER_CHECKPOINT: &str = "user_checkpoint";
pub const CLAIMABLE_TOKENS: &str = "claimable_tokens";
pub const CLAIMABLE_REWARD: &str = "claimable_reward";
pub const CLAIMABLE_V3_REWARD: &str = "claimable_reward_v3";
pub const U8_DECIMALS: &str = "decimals_u8";
pub const ALLOWANCE: &str = "allowance";
// Gauge Controller Wasm Keys
pub const GAUGE_TYPES: &str = "gauge_types";
pub const GAUGE_RELATIVE_WEIGHT: &str = "gauge_relative_weight";
pub const GAUGE_RELATIVE_WEIGHT_WRITE: &str = "gauge_relative_weight_write";
pub const GET_GAUGE_WEIGHT: &str = "get_gauge_weight";
pub const GET_TYPE_WEIGHT: &str = "get_type_weight";
pub const GET_TOTAL_WEIGHT: &str = "get_total_weight";
pub const GET_WEIGHTS_SUM_PER_TYPE: &str = "get_weights_sum_per_type";
//IRewardDistributionRecipient
pub const REWARDDISTRIBUTION: &str = "reward_distribution";
//LpTokenWrapper
pub const UNI: &str = "uni";
pub const BALANCES: &str = "balances";
//CruveRewards
pub const SNX: &str = "snx";
pub const PERIOD_FINISH: &str = "period_finish";
pub const REWARD_RATE: &str = "reward_rate";
pub const LAST_UPDATE_TIME: &str = "last_update_time";
pub const REWARD_PER_TOKEN_STORED: &str = "reward_per_token_stored";
pub const USER_REWARD_PER_TOKEN_PAID_DICT: &str = "user_reward_per_token_paid";
pub const REWARDS_DICT: &str = "rewards";
pub const LAST_TIME_REWARD_APPLICABLE: &str = "last_time_reward_applicable";
pub const REWARD_PER_TOKEN: &str = "reward_per_token";
pub const EARNED: &str = "earned";
pub const IS_OWNER: &str = "is_owner";
//Liqudity Guage v3 keys
pub const MYVEC: &str = "myvec";
pub const INTEGRATE_CHECKPOINT: &str = "integrate_checkpoint";
pub const REWARD_SIGS: &str = "reward_sigs";
//VESTING ESCROW
pub const APPLY_TRANSFER_OWNERSHIP: &str = "apply_transfer_ownership";
pub const COMMIT_TRANSFER_OWNERSHIP: &str = "commit_transfer_ownership";
pub const VESTED_SUPPLY: &str = "vested_supply";
pub const LOCKED_SUPPLY: &str = "locked_supply";
pub const VESTED_OF: &str = "vested_of";
pub const LOCKED_OF: &str = "locked_of";
// REWARD_ONLY_GAUGE
pub const TRANSFER: &str = "transfer";
pub const TRANSFER_FROM: &str = "transfer_from";
pub const INCREASE_ALLOWANCE: &str = "increase_allowance";
pub const DECREASE_ALLOWANCE: &str = "decrease_allowance";
pub const LAST_CLAIM: &str = "last_claim";
pub const CLAIMED_REWARD: &str = "claimed_reward";
pub const CLAIMABLE_REWARD_WRITE: &str = "claimable_reward_write";
