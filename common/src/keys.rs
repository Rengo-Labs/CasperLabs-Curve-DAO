// Session code
pub const SESSION_CODE_WASM: &str = "session-code.wasm";
pub const SESSION_CODE_NAME: &str = "SessionCode";
// Voting Escrow
pub const GET_LAST_USER_SLOPE: &str = "get_last_user_slope";
pub const USER_POINT_HISTORY_TS: &str = "user_point_history_ts";
pub const LOCKED_END: &str = "locked_end";
pub const BALANCE_OF: &str = "balance_of";
pub const BALANCE_OF_AT: &str = "balance_of_at";
pub const TOTAL_SUPPLY: &str = "total_supply";
pub const TOTAL_SUPPLY_AT: &str = "total_supply_at";
// Fee Distributor
pub const VE_FOR_AT: &str = "ve_for_at";
pub const CLAIM: &str = "claim";
pub const CLAIM_MANY: &str = "claim_many";
pub const BURN: &str = "burn";
pub const RECOVER_BALANCE: &str = "recover_balance";
// Result
pub const RESULT_KEY: &str = "result";
pub const FUTURE_EPOCH_TIME_WRITE: &str = "future_epoch_time_write";
pub const START_EPOCH_TIME_WRITE: &str = "start_epoch_time_write";
pub const AVAILABLE_SUPPLY: &str = "available_supply";
pub const MINT_CRV: &str = "mint_crv";
pub const MINTABLE_IN_TIMEFRAME: &str = "mintable_in_timeframe";
// Gauge Controller Wasm Keys
pub const GAUGE_TYPES: &str = "gauge_types";
pub const GAUGE_RELATIVE_WEIGHT: &str = "gauge_relative_weight";
pub const GAUGE_RELATIVE_WEIGHT_WRITE: &str = "gauge_relative_weight_write";
pub const GET_GAUGE_WEIGHT: &str = "get_gauge_weight";
pub const GET_TYPE_WEIGHT: &str = "get_type_weight";
pub const GET_TOTAL_WEIGHT: &str = "get_total_weight";
pub const GET_WEIGHTS_SUM_PER_TYPE: &str = "get_weights_sum_per_type";



//ERC20_CRV
pub const ERC20_CRV_SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const ERC20_CRV_SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const ERC20_CRV_RESULT: &str = "result";
pub const ERC20_CRV_MINING_EPOCH: &str = "mining_epoch";
pub const ERC20_CRV_IS_UPDATED: &str = "is_updated";
pub const ERC20_CRV_START_EPOCH_TIME: &str = "start_epoch_time";
pub const ERC20_CRV_RATE: &str = "rate";
pub const ERC20_CRV_START_EPOCH_SUPPLY: &str = "start_epoch_supply";
pub const ERC20_CRV_INIT_SUPPLY: &str = "init_supply";
pub const ERC20_CRV_ADMIN: &str = "admin";
pub const ERC20_CRV_MINTER: &str = "minter";
//FEE_DRISTRIBUTOR
pub const FEE_DISTRIBUTOR_START_TIME: &str = "start_time";
pub const FEE_DISTRIBUTOR_TIME_CURSOR: &str = "time_cursor";
pub const FEE_DISTRIBUTOR_LAST_TOKEN_TIME: &str = "last_token_time";
pub const FEE_DISTRIBUTOR_VOTING_ESCROW: &str = "voting_escrow";
pub const FEE_DISTRIBUTOR_TOKEN: &str = "token";
pub const FEE_DISTRIBUTOR_TOTAL_RECEIVED: &str = "total_received";
pub const FEE_DISTRIBUTOR_TOKEN_LAST_BALANCE: &str = "token_last_balance";
pub const FEE_DISTRIBUTOR_ADMIN: &str = "admin";
pub const FEE_DISTRIBUTOR_FUTURE_ADMIN: &str = "future_admin";
pub const FEE_DISTRIBUTOR_CAN_CHECKPOINT_TOKEN: &str = "can_checkpoint_token";
pub const FEE_DISTRIBUTOR_EMERGENCY_RETURN: &str = "emergency_return";
pub const FEE_DISTRIBUTOR_IS_KILLED: &str = "is_killed";
pub const FEE_DISTRIBUTOR_LOCK: &str = "lock";
pub const FEE_DISTRIBUTOR_CONTRACT_HASH: &str = "contract_hash";
pub const FEE_DISTRIBUTOR_PACKAGE_HASH: &str = "package_hash";
pub const FEE_DISTRIBUTOR_RESULT: &str = "result";
//MINTER
pub const MINTER_MINTED_DICT: &str = "minted";
pub const MINTER_ALLOWED_TO_MINT_FOR_DICT: &str = "allowed_to_mint_for";
pub const MINTER_NAME: &str = "name";
pub const MINTER_TOKEN: &str = "token";
pub const MINTER_CONTROLLER: &str = "controller";
pub const MINTER_REWARD_COUNT: &str = "reward_count";
pub const MINTER_SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const MINTER_CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";
pub const MINTER_LOCK: &str = "lock";
//REWARD_ONLy GAUGE
pub const REWARD_ONLY_GAUGE_BALANCES_DICT: &str = "balances";
pub const REWARD_ONLY_GAUGE_NONCES_DICT: &str = "nonces";
pub const REWARD_ONLY_GAUGE_ALLOWANCES_DICT: &str = "allowances";
pub const REWARD_ONLY_GAUGE_REWARD_TOKENS_DICT: &str = "reward_tokens";
pub const REWARD_ONLY_GAUGE_REWARD_BALANCES_DICT: &str = "reward_balances";
pub const REWARD_ONLY_GAUGE_REWARDS_RECEIVER_DICT: &str = "reward_receiver";
pub const REWARD_ONLY_GAUGE_REWARD_INTEGRAL_DICT: &str = "reward_integral";
pub const REWARD_ONLY_GAUGE_REWARD_INTEGRAL_FOR_DICT: &str = "reward_integral_for";
pub const REWARD_ONLY_GAUGE_CLAIM_DATA_DICT: &str = "claim_data";
pub const REWARD_ONLY_GAUGE_CLAIM_SIG: &str = "claim_sig";
pub const REWARD_ONLY_GAUGE_NAME: &str = "name";
pub const REWARD_ONLY_GAUGE_SYMBOL: &str = "symbol";
pub const REWARD_ONLY_GAUGE_DECIMALS: &str = "decimals";
pub const REWARD_ONLY_GAUGE_TOTAL_SUPPLY: &str = "total_supply";
pub const REWARD_ONLY_GAUGE_ADMIN: &str = "admin";
pub const REWARD_ONLY_GAUGE_LP_TOKEN: &str = "lp_token";
pub const REWARD_ONLY_GAUGE_FUTURE_ADMIN: &str = "future_admin";
pub const REWARD_ONLY_GAUGE_REWARD_DATA: &str = "reward_data";
pub const REWARD_ONLY_GAUGE_SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const REWARD_ONLY_GAUGE_CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";
pub const REWARD_ONLY_GAUGE_LOCK: &str = "lock";
// VESTING_ESCROW
pub const VESTING_ESCROW_INITIAL_LOCKED_DICT: &str = "initial_locked";
pub const VESTING_ESCROW_TOTAL_CLAIMED_DICT: &str = "total_claimed";
pub const VESTING_ESCROW_DISABLED_AT_DICT: &str = "disabled_at";
pub const VESTING_ESCROW_FUND_ADMINS_DICT: &str = "fund_admins";

pub const VESTING_ESCROW_TOKEN: &str = "token";
pub const VESTING_ESCROW_ADMIN: &str = "admin";
pub const VESTING_ESCROW_FUTURE_ADMIN: &str = "future_admin";
pub const VESTING_ESCROW_SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const VESTING_ESCROW_CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";
pub const VESTING_ESCROW_LOCK: &str = "lock";
pub const VESTING_ESCROW_START_TIME: &str = "start_time";
pub const VESTING_ESCROW_END_TIME: &str = "end_time";
pub const VESTING_ESCROW_INITIAL_LOCKED_SUPPLY: &str = "initial_locked_supply";
pub const VESTING_ESCROW_UNALLOCATED_SUPPLY: &str = "unallocated_supply";
pub const VESTING_ESCROW_CAN_DISABLE: &str = "can_disable";
pub const VESTING_ESCROW_FUND_ADMINS_FUNDS: &str = "fund_admins_enabled";
// VESTING_ESCROW_FACTORY
pub const VESTING_ESCROW_FACTORY_INITIAL_LOCKED_DICT: &str = "initial_locked";
pub const VESTING_ESCROW_FACTORY_TOTAL_CLAIMED_DICT: &str = "total_claimed";
pub const VESTING_ESCROW_FACTORY_DISABLED_AT_DICT: &str = "disabled_at";
pub const VESTING_ESCROW_FACTORY_FUND_ADMINS_DICT: &str = "fund_admins";
pub const VESTING_ESCROW_FACTORY_TARGET: &str = "target";
pub const VESTING_ESCROW_FACTORY_ADMIN: &str = "admin";
pub const VESTING_ESCROW_FACTORY_FUTURE_ADMIN: &str = "future_admin";
pub const VESTING_ESCROW_FACTORY_SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const VESTING_ESCROW_FACTORY_CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";
pub const VESTING_ESCROW_FACTORY_VESTING_ESCROW_SIMPLE_CONTRACT: &str =
    "vesting_escrow_simple_contract";
// VESTING_ESCROW_SIMPLE
pub const VESTING_ESCROW_SIMPLE_SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const VESTING_ESCROW_SIMPLE_SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const VESTING_ESCROW_SIMPLE_RESULT: &str = "result";
pub const VESTING_ESCROW_SIMPLE_TOKEN: &str = "token";
pub const VESTING_ESCROW_SIMPLE_START_TIME: &str = "start_time";
pub const VESTING_ESCROW_SIMPLE_END_TIME: &str = "end_time";
pub const VESTING_ESCROW_SIMPLE_INITIAL_LOCKED_DICT_SUPPLY: &str = "initial_locked_supply";
pub const VESTING_ESCROW_SIMPLE_CAN_DISABLE: &str = "can_disable";
pub const VESTING_ESCROW_SIMPLE_ADMIN: &str = "admin";
pub const VESTING_ESCROW_SIMPLE_FUTURE_ADMIN: &str = "future_admin";
pub const VESTING_ESCROW_SIMPLE_LOCK: &str = "lock";
pub const VESTING_ESCROW_SIMPLE_INITIAL_LOCKED_DICT: &str = "initial_locked";
pub const VESTING_ESCROW_SIMPLE_TOTAL_CLAIMED_DICT: &str = "total_claimed";
pub const VESTING_ESCROW_SIMPLE_DISABLED_AT_DICT: &str = "disabled_at";

// VOTING_ESCROW
pub const VOTING_ESCROW_TOKEN: &str = "token";
pub const VOTING_ESCROW_SUPPLY: &str = "supply";
pub const VOTING_ESCROW_ADMIN: &str = "admin";
pub const VOTING_ESCROW_FUTURE_ADMIN: &str = "future_admin";
pub const VOTING_ESCROW_CONTROLLER: &str = "controller";
pub const VOTING_ESCROW_TRANSFERS_ENABLED: &str = "transfers_enabled";
pub const VOTING_ESCROW_NAME: &str = "name";
pub const VOTING_ESCROW_SYMBOL: &str = "symbol";
pub const VOTING_ESCROW_VERSION: &str = "version";
pub const VOTING_ESCROW_DECIMALS: &str = "decimals";
pub const VOTING_ESCROW_EPOCH: &str = "epoch";
pub const VOTING_ESCROW_LOCK: &str = "lock";
pub const VOTING_ESCROW_CONTRACT_HASH: &str = "contract_hash";
pub const VOTING_ESCROW_PACKAGE_HASH: &str = "package_hash";
pub const VOTING_ESCROW_RESULT: &str = "result";

//CURVE_TOKEN_V1
pub const CURVE_TOKEN_V1_SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const CURVE_TOKEN_V1_SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const CURVE_TOKEN_V1_RESULT: &str = "result";
pub const CURVE_TOKEN_V1_MINTER: &str = "minter";
pub const CURVE_TOKEN_V1_INIT_SUPPLY: &str = "init_supply";
// CURVE_TOKEN_V2
pub const CURVE_TOKEN_V2_SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const CURVE_TOKEN_V2_SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const CURVE_TOKEN_V2_RESULT: &str = "result";
pub const CURVE_TOKEN_V2_MINTER: &str = "minter";
pub const CURVE_TOKEN_V2_INIT_SUPPLY: &str = "init_supply";
// CURVE_TOKEN_V3
pub const CURVE_TOKEN_V3_SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const CURVE_TOKEN_V3_SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const CURVE_TOKEN_V3_RESULT: &str = "result";
pub const CURVE_TOKEN_V3_MINTER: &str = "minter";
pub const CURVE_TOKEN_V3_CURVE: &str = "curve";
//GAUGE_CONTROLLER
pub const GAUGE_CONTROLLER_GAUGE_TYPE_NAMES_DICT: &str = "gauge_type_names";
pub const GAUGE_CONTROLLER_GAUGE_TYPES_DICT: &str = "gauge_types_";
pub const GAUGE_CONTROLLER_VOTE_USER_SLOPES_DICT: &str = "vote_user_slopes";
pub const GAUGE_CONTROLLER_VOTE_USER_POWER_DICT: &str = "vote_user_power";
pub const GAUGE_CONTROLLER_LAST_USER_VOTE_DICT: &str = "last_user_vote";
pub const GAUGE_CONTROLLER_POINTS_WEIGHT_DICT: &str = "points_weight";
pub const GAUGE_CONTROLLER_CHANGES_WEIGHT_DICT: &str = "changes_weight";
pub const GAUGE_CONTROLLER_TIME_WEIGHT_DICT: &str = "time_weight";
pub const GAUGE_CONTROLLER_GAUGES_DICT: &str = "gauges";
pub const GAUGE_CONTROLLER_TIME_SUM_DICT: &str = "time_sum";
pub const GAUGE_CONTROLLER_POINTS_SUM_DICT: &str = "points_sum";
pub const GAUGE_CONTROLLER_CHANGES_SUM_DICT: &str = "changes_sum";
pub const GAUGE_CONTROLLER_POINTS_TOTAL_DICT: &str = "points_total";
pub const GAUGE_CONTROLLER_POINTS_TYPE_WEIGHT_DICT: &str = "points_type_weight";
pub const GAUGE_CONTROLLER_TIME_TYPE_WEIGHT_DICT: &str = "time_type_weight";

pub const GAUGE_CONTROLLER_OWNER: &str = "owner";
pub const GAUGE_CONTROLLER_ADMIN: &str = "admin";
pub const GAUGE_CONTROLLER_FUTURE_ADMIN: &str = "future_admin";
pub const GAUGE_CONTROLLER_TIME_TOTAL: &str = "time_total";
pub const GAUGE_CONTROLLER_TOKEN: &str = "token";
pub const GAUGE_CONTROLLER_VOTING_ESCROW: &str = "voting_escrow";
pub const GAUGE_CONTROLLER_REWARD_COUNT: &str = "reward_count";
pub const GAUGE_CONTROLLER_SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const GAUGE_CONTROLLER_CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";
pub const GAUGE_CONTROLLER_N_GAUGE_TYPES: &str = "n_gauge_types";
pub const GAUGE_CONTROLLER_N_GAUGES: &str = "n_gauges";
pub const GAUGE_CONTROLLER_LAST_USER_VOTE: &str = "last_user_vote";
//LIQUIDITY_GAUGE_REWARD
pub const LIQUIDITY_GAUGE_REWARD_PERIOD: &str = "period";
pub const LIQUIDITY_GAUGE_REWARD_MINTER: &str = "minter";
pub const LIQUIDITY_GAUGE_REWARD_CRV_TOKEN: &str = "crv_token";
pub const LIQUIDITY_GAUGE_REWARD_LP_TOKEN: &str = "lp_token";
pub const LIQUIDITY_GAUGE_REWARD_CONTROLLER: &str = "controller";
pub const LIQUIDITY_GAUGE_REWARD_VOTING_ESCROW: &str = "voting_escrow";
pub const LIQUIDITY_GAUGE_REWARD_TOTAL_SUPPLY: &str = "total_supply";
pub const LIQUIDITY_GAUGE_REWARD_FUTURE_EPOCH_TIME: &str = "future_epoch_time";
pub const LIQUIDITY_GAUGE_REWARD_WORKING_SUPPLY: &str = "working_supply";
pub const LIQUIDITY_GAUGE_REWARD_INFLATION_RATE: &str = "inflation_rate";
pub const LIQUIDITY_GAUGE_REWARD_REWARD_CONTRACT: &str = "reward_contract";
pub const LIQUIDITY_GAUGE_REWARD_REWARDED_TOKEN: &str = "rewarded_token";
pub const LIQUIDITY_GAUGE_REWARD_REWARD_INTEGRAL: &str = "reward_integral";
pub const LIQUIDITY_GAUGE_REWARD_ADMIN: &str = "admin";
pub const LIQUIDITY_GAUGE_REWARD_FUTURE_ADMIN: &str = "future_admin";
pub const LIQUIDITY_GAUGE_REWARD_IS_KILLED: &str = "is_killed";
pub const LIQUIDITY_GAUGE_REWARD_IS_CLAIMING_REWARDS: &str = "is_claiming_rewards";
pub const LIQUIDITY_GAUGE_REWARD_LOCK: &str = "lock";
pub const LIQUIDITY_GAUGE_REWARD_CONTRACT_HASH: &str = "contract_hash";
pub const LIQUIDITY_GAUGE_REWARD_PACKAGE_HASH: &str = "package_hash";
pub const LIQUIDITY_GAUGE_REWARD_RESULT: &str = "result";
pub const LIQUIDITY_GAUGE_REWARD_APPROVED_TO_DEPOSIT: &str = "approved_to_deposit";
pub const LIQUIDITY_GAUGE_REWARD_BALANCE_OF: &str = "balance_of";
pub const LIQUIDITY_GAUGE_REWARD_WORKING_BALANCES: &str = "working_balances";
pub const LIQUIDITY_GAUGE_REWARD_PERIOD_TIMESTAMP: &str = "period_timestamp";
pub const LIQUIDITY_GAUGE_REWARD_INTEGRATE_INV_SUPPLY: &str = "integrate_inv_supply";
pub const LIQUIDITY_GAUGE_REWARD_INTEGRATE_INV_SUPPLY_OF: &str = "integrate_inv_supply_of";
pub const LIQUIDITY_GAUGE_REWARD_INTEGRATE_CHECKPOINT_OF: &str = "integrate_checkpoint_of";
pub const LIQUIDITY_GAUGE_REWARD_INTEGRATE_FRACTION: &str = "integrate_fraction";
pub const LIQUIDITY_GAUGE_REWARD_REWARD_INTEGRAL_FOR: &str = "reward_integral_for";
pub const LIQUIDITY_GAUGE_REWARD_REWARDS_FOR: &str = "rewards_for";
pub const LIQUIDITY_GAUGE_REWARD_CLAIMED_REWARDS_FOR: &str = "claimed_rewards_for";
//LIQUIDITY_GAUGE_REWARD_WRAPPER
pub const REWARD_WRAPPER_MINTER: &str = "minter";
pub const REWARD_WRAPPER_CRV_TOKEN: &str = "crv_token";
pub const REWARD_WRAPPER_REWARDED_TOKEN: &str = "rewarded_token";
pub const REWARD_WRAPPER_LP_TOKEN: &str = "lp_token";
pub const REWARD_WRAPPER_GAUGE: &str = "gauge";
pub const REWARD_WRAPPER_TOTAL_SUPPLY: &str = "total_supply";
pub const REWARD_WRAPPER_NAME: &str = "name";
pub const REWARD_WRAPPER_SYMBOL: &str = "symbol";
pub const REWARD_WRAPPER_DECIMALS: &str = "decimals";
pub const REWARD_WRAPPER_REWARD_INTEGRAL: &str = "reward_integral";
pub const REWARD_WRAPPER_CRV_INTEGRAL: &str = "crv_integral";
pub const REWARD_WRAPPER_ADMIN: &str = "admin";
pub const REWARD_WRAPPER_FUTURE_ADMIN: &str = "future_admin";
pub const REWARD_WRAPPER_IS_KILLED: &str = "is_killed";
pub const REWARD_WRAPPER_LOCK: &str = "lock";
pub const REWARD_WRAPPER_CONTRACT_HASH: &str = "contract_hash";
pub const REWARD_WRAPPER_PACKAGE_HASH: &str = "package_hash";
pub const REWARD_WRAPPER_RESULT: &str = "result";
pub const REWARD_WRAPPER_ALLOWANCES: &str = "allownances";
pub const REWARD_WRAPPER_APPROVED_TO_DEPOSIT: &str = "approved_to_deposit";
pub const REWARD_WRAPPER_BALANCE_OF: &str = "balance_of";
pub const REWARD_WRAPPER_CLAIMABLE_CRV: &str = "claimable_crv";
pub const REWARD_WRAPPER_REWARD_INTEGRAL_FOR: &str = "reward_integral_for";
pub const REWARD_WRAPPER_CRV_INTEGRAL_FOR: &str = "crv_integral_for";
pub const REWARD_WRAPPER_CLAIMABLE_REWARDS: &str = "claimable_rewards";
