# CasperLabs-Curve-DAO

Implementation of following contracts for the CasperLabs platform.

1.  `Curve Rewards`
2.  `Minter`
3.  `Curve Token v3`
4.  `ERC20 CRV`
5.  `Fee Distributor`
6.  `Gauge Controller`
7.  `Gauge Proxy`
8.  `Liquidity Gauge Reward`
9.  `Liquidity Gauge Reward Wrapper`
10. `Liquidity Gauge Wrapper`
11. `Lp Token Wrapper`
12. `Reward Only Gauge`
13. `Vesting Escrow`
14. `Vesting Escrow Factory`
15. `Vesting Escrow Simple`
16. `Voting Escrow`
17. `Liquidity Gauge V3`
18. `Ownable`
19. `I Reward Distribution Recipient`

## Error Code List

https://docs.google.com/spreadsheets/d/1Rzh1LERQyGiGpHB3djlT1Tk0LNQ18q_eLBWFDPm2bNc/edit#gid=4667616

## Table of contents

- [Interacting with the contract](#interacting-with-the-contract)
  - [Install the prerequisites](#install-the-prerequisites)
  - [Creating Keys](#creating-keys)
  - [Usage](#usage)
    - [Install](#install)
    - [Build Individual Smart Contract](#build-individual-smart-contract)
    - [Build All Smart Contracts](#build-all-smart-contracts)
    - [Individual Test Cases](#individual-test-cases)
    - [All Test Cases](#all-test-cases)
  - [Known contract hashes](#known-contract-hashes)
  - [Deploying Reward Only Gauge contract manually](#deploying-reward-only-gauge-contract-manually)
    - [Entry Point methods](#RewardOnlyGauge-entry-point-methods)
      - [decimals](#RewardOnlyGauge-decimals)
      - [reward_contract](#RewardOnlyGauge-reward-contract)
      - [last_claim](#RewardOnlyGauge-last-claim)
      - [claimable_reward](#RewardOnlyGauge-claimable-reward)
      - [claimable_reward_write](#RewardOnlyGauge-claimable-reward-write)
      - [set_rewards_receiver](#RewardOnlyGauge-set-rewards-receiver)
      - [claim_rewards](#RewardOnlyGauge-claim-rewards)
      - [deposit](#RewardOnlyGauge-deposit)
      - [withdraw](#RewardOnlyGauge-withdraw)
      - [transfer](#RewardOnlyGauge-transfer)
      - [transfer_from](#RewardOnlyGauge-transfer-from)
      - [approve](#RewardOnlyGauge-approve)
      - [increase_allowance](#RewardOnlyGauge-increase-allowance)
      - [decrease_allowance](#RewardOnlyGauge-decrease_allowance)
      - [set_rewards](#RewardOnlyGauge-set-rewards)
      - [commit_transfer_ownership](#RewardOnlyGauge-commit-transfer-ownership)
      - [accept_transfer_ownership](#RewardOnlyGauge-accept-transfer-ownership)
      - [decimals](#RewardOnlyGauge-decimals)
      - [future_admin](#RewardOnlyGauge-future-admin)
      - [admin](#RewardOnlyGauge-admin)
      - [reward_integral_for](#RewardOnlyGauge-reward-integral-for)
      - [reward_integral](#RewardOnlyGauge-reward-integral)
      - [claim_sig](#RewardOnlyGauge-claim-sig)
      - [rewards_receiver](#RewardOnlyGauge-rewards-receiver)
      - [reward_balances](#RewardOnlyGauge-reward-balances)
      - [reward_tokens](#RewardOnlyGauge-reward-tokens)
      - [lp_token](#RewardOnlyGauge-lp-token)
      - [balance_of](#RewardOnlyGauge-balance-of)
      - [total_supply](#RewardOnlyGauge-total-supply)
      - [allowance](#RewardOnlyGauge-allowance)
      - [name](#RewardOnlyGauge-name)
      - [symbol](#RewardOnlyGauge-symbol)
  - [Deploying Gauge Controller contract manually](#deploying-gauge-controller-contract-manually)
    - [Entry Point methods](#GaugeController-entry-point-methods)
      - [commit_transfer_ownership](#GaugeController-commit-transfer-ownership)
      - [apply_transfer_ownership](#GaugeController-apply-transfer-ownership)
      - [gauge_types](#GaugeController-gauge-types)
      - [add_gauge](#GaugeController-add-gauge)
      - [checkpoint](#GaugeController-checkpoint)
      - [checkpoint_gauge](#GaugeController-checkpoint-gauge)
      - [gauge_relative_weight](#GaugeController-gauge-relative-weight)
      - [gauge_relative_weight_write](#GaugeController-gauge-relative-weight-write)
      - [add_type](#GaugeController-add-type)
      - [change_type_weight](#GaugeController-change-type-weight)
      - [change_gauge_weight](#GaugeController-change-gauge-weight)
      - [vote_for_gauge_weights](#GaugeController-vote-for-gauge-weights)
      - [get_gauge_weight](#GaugeController-get-gauge-weight)
      - [get_type_weight](#GaugeController-get-type-weight)
      - [get_total_weight](#GaugeController-get-total-weight)
      - [get_weights_sum_per_type](#GaugeController-get-weights-sum-per-type)
      - [future_admin](#GaugeController-future-admin)
      - [admin](#GaugeController-admin)
      - [token](#GaugeController-token)
      - [voting_escrow](#GaugeController-voting-escrow)
      - [n_gauge_types](#GaugeController-n-gauge-types)
      - [n_gauges](#GaugeController-n-gauges)
      - [gauge_type_names](#GaugeController-gauge-type-names)
      - [gauges](#GaugeController-gauges)
      - [vote_user_slopes](#GaugeController-vote-user-slopes)
      - [vote_user_power](#GaugeController-vote-user-power)
      - [last_user_vote](#GaugeController-last-user-vote)
      - [points_weight](#GaugeController-points-weight)
      - [time-weight](#GaugeController-time-weight)
      - [points_sum](#GaugeController-points-sum)
      - [time_sum](#GaugeController-time-sum)
      - [points_total](#GaugeController-points-total)
      - [time_total](#GaugeController-time-total)
      - [points_type_weight](#GaugeController-points-type-weight)
      - [time_type_weight](#GaugeController-time-type-weight)
  - [Deploying Minter contract manually](#deploying-minter-contract-manually)
    - [Entry Point methods](#Minter-entry-point-methods)
      - [mint](#Minter-mint)
      - [mint_many](#Minter-mint-many)
      - [mint_for](#Minter-mint-for)
      - [toggle_approve_mint](#Minter-toggle-approve-mint)
      - [token](#Minter-token)
      - [Controller](#Minter-Controller)
      - [allowed_to_mint_for](#Minter-allowed-to-mint-for)
      - [minted](#Minter-minted)
  - [Deploying Curve Rewards contract manually](#deploying-curve-rewards-contract-manually)
    - [Entry Point methods](#CurveRewards-entry-point-methods)
      - [last_time_reward_applicable](#CurveRewards-last-time-reward-applicable)
      - [reward_per_token](#CurveRewards-reward-per-token)
      - [earned](#CurveRewards-earned)
      - [stake](#CurveRewards-stake)
      - [withdraw](#CurveRewards-withdraw)
      - [exit](#CurveRewards-exit)
      - [get_reward](#CurveRewards-get-reward)
      - [notify_reward_amount](#CurveRewards-notify-reward-amount)
      - [total_supply](#CurveRewards-total-supply)
      - [balance_of](#CurveRewards-balance-of)
      - [stake_lp](#CurveRewards-notify-stake-lp)
      - [withdraw_lp](#CurveRewards-withdraw-lp)
      - [set_reward_distribution](#CurveRewards-set-reward-distribution)
      - [owner](#CurveRewards-owner)
      - [is_owner](#CurveRewards-is-owner)
      - [renounce_ownership](#CurveRewards-renounce-ownership)
      - [transfer_ownership](#CurveRewards-transfer-ownership)
      - [uni](#CurveRewards-uni)
      - [snx](#CurveRewards-snx)
      - [duration](#CurveRewards-duration)
      - [period_finish](#CurveRewards-period-finish)
      - [reward_rate](#CurveRewards-reward-rate)
      - [last_update_time](#CurveRewards-last-update-time)
      - [reward_per_token_stored](#CurveRewards-reward-per-token-stored)
      - [user_reward_per_token_paid](#CurveRewards-user-reward-per-token-paid)
      - [rewards](#CurveRewards-rewards)
  - [Deploying Lp Token Wrapper contract manually](#deploying-lp-token-wrapper-contract-manually)
    - [Entry Point methods](#LpTokenWrapper-entry-point-methods)
      - [stake](#LpTokenWrapper-stake)
      - [withdraw](#LpTokenWrapper-withdraw)
      - [total_supply](#LpTokenWrapper-total-supply)
      - [balance_of](#LpTokenWrapper-balance-of)
      - [uni](#LpTokenWrapper-uni)
  - [Deploying Ownable contract manually](#deploying-ownable-contract-manually)
    - [Entry Point methods](#Ownable-entry-point-methods)
      - [owner](#Ownable-owner)
      - [is_owner](#Ownable-is-owner)
      - [renounce_ownership](#Ownable-renounce-ownership)
      - [transfer_ownership](#Ownable-transfer-ownership)
  - [Deploying Liquidity Gauge Wrapper contract manually](#deploying-liquidity-gauge-wrapper-contract-manually)
    - [Entry Point methods](#LiquidityGaugeWrapper-entry-point-methods)
      - [user_checkpoint](#LiquidityGaugeWrapper-user-checkpoint)
      - [claimable_tokens](#LiquidityGaugeWrapper-claimable-tokens)
      - [claim_tokens](#LiquidityGaugeWrapper-claim-tokens)
      - [set_approve_deposit](#LiquidityGaugeWrapper-set-approve-deposit)
      - [deposit](#LiquidityGaugeWrapper-deposit)
      - [withdraw](#LiquidityGaugeWrapper-user-checkpoint)
      - [allowance](#LiquidityGaugeWrapper-allowance)
      - [transfer](#LiquidityGaugeWrapper-transfer)
      - [transfer_from](#LiquidityGaugeWrapper-transfer-from)
      - [approve](#LiquidityGaugeWrapper-approve)
      - [increase_allowance](#LiquidityGaugeWrapper-increase-allowance)
      - [decrease_allowance](#LiquidityGaugeWrapper-decrease-allowance)
      - [kill_me](#LiquidityGaugeWrapper-kill-me)
      - [commit_transfer_ownership](#LiquidityGaugeWrapper-commit-transfer-ownership)
      - [apply_transfer_ownership](#LiquidityGaugeWrapper-apply-transfer-ownership)
      - [minter](#LiquidityGaugeWrapper-minter)
      - [crv_token](#LiquidityGaugeWrapper-crv-token)
      - [lp_token](#LiquidityGaugeWrapper-lp-token)
      - [gauge](#LiquidityGaugeWrapper-gauge)
      - [balance_of](#LiquidityGaugeWrapper-balance-of)
      - [total_supply](#LiquidityGaugeWrapper-total-supply)
      - [name](#LiquidityGaugeWrapper-name)
      - [symbol](#LiquidityGaugeWrapper-symbol)
      - [decimals](#LiquidityGaugeWrapper-decimals)
      - [future_admin](#LiquidityGaugeWrapper-future-admin)
      - [admin](#LiquidityGaugeWrapper-admin)
      - [claimable_crv](#LiquidityGaugeWrapper-claimable-crv)
      - [approved_to_deposit](#LiquidityGaugeWrapper-approved-to-deposit)
      - [is_killed](#LiquidityGaugeWrapper-is-killed)
  - [Deploying Liquidity Gauge Reward Wrapper contract manually](#deploying-liquidity-gauge-reward-wrapper-contract-manually)
    - [Entry Point methods](#LiquidityGaugeRewardWrapper-entry-point-methods)
      - [user_checkpoint](#LiquidityGaugeRewardWrapper-user-checkpoint)
      - [claimable_tokens](#LiquidityGaugeRewardWrapper-claimable-tokens)
      - [claimable_reward](#LiquidityGaugeRewardWrapper-claimable-reward)
      - [claim_tokens](#LiquidityGaugeRewardWrapper-claim-tokens)
      - [set_approve_deposit](#LiquidityGaugeRewardWrapper-set-approve-deposit)
      - [deposit](#LiquidityGaugeRewardWrapper-deposit)
      - [withdraw](#LiquidityGaugeRewardWrapper-withdraw)
      - [allowance](#LiquidityGaugeRewardWrapper-claim-tokens)
      - [transfer](#LiquidityGaugeRewardWrapper-transfer)
      - [transfer_from](#LiquidityGaugeRewardWrapper-transfer-from)
      - [approve](#LiquidityGaugeRewardWrapper-approve)
      - [increase_allowance](#LiquidityGaugeRewardWrapper-increase-allowance)
      - [decrease_allowance](#LiquidityGaugeRewardWrapper-decrease-allowance)
      - [kill_me](#LiquidityGaugeRewardWrapper-kill-me)
      - [commit_transfer_ownership](#LiquidityGaugeRewardWrapper-commit-transfer-ownership)
      - [apply_transfer_ownership](#LiquidityGaugeRewardWrapper-apply-transfer-ownership)
      - [minter](#LiquidityGaugeRewardWrapper-minter)
      - [crv_token](#LiquidityGaugeRewardWrapper-crv-token)
      - [lp_token](#LiquidityGaugeRewardWrapper-lp-token)
      - [rewarded_token](#LiquidityGaugeRewardWrapper-rewarded-token)
      - [gauge](#LiquidityGaugeRewardWrapper-gauge)
      - [balance_of](#LiquidityGaugeRewardWrapper-balance-of)
      - [total_supply](#LiquidityGaugeRewardWrapper-total-supply)
      - [name](#LiquidityGaugeRewardWrapper-name)
      - [symbol](#LiquidityGaugeRewardWrapper-symbol)
      - [decimals](#LiquidityGaugeRewardWrapper-decimals)
      - [future_admin](#LiquidityGaugeRewardWrapper-future-admin)
      - [admin](#LiquidityGaugeRewardWrapper-admin)
      - [claimable_crv](#LiquidityGaugeRewardWrapper-claimable-crv)
      - [approved_to_deposit](#LiquidityGaugeRewardWrapper-approved-to-deposit)
      - [is_killed](#LiquidityGaugeRewardWrapper-is-killed)
      - [reward_integral_for](#LiquidityGaugeRewardWrapper-reward-integral-for)
      - [reward_integral](#LiquidityGaugeRewardWrapper-reward-integral)
      - [claimable_rewards](#LiquidityGaugeRewardWrapper-claimable-rewards)
  - [Deploying Liquidity Gauge Reward contract manually](#deploying-liquidity-gauge-reward-contract-manually)
    - [Entry Point methods](#LiquidityGaugeReward-entry-point-methods)
      - [user_checkpoint](#LiquidityGaugeReward-user-checkpoint)
      - [claimable_tokens](#LiquidityGaugeReward-claimable-tokens)
      - [claimable_reward](#LiquidityGaugeReward-claimable-reward)
      - [kick](#LiquidityGaugeReward-kick)
      - [set_approve_deposit](#LiquidityGaugeReward-set-approve-deposit)
      - [deposit](#LiquidityGaugeReward-deposit)
      - [withdraw](#LiquidityGaugeReward-withdraw)
      - [claim_rewards](#LiquidityGaugeReward-claim-rewards)
      - [integrate_checkpoint](#LiquidityGaugeReward-integrate-checkpoint)
      - [kill_me](#LiquidityGaugeReward-kill-me)
      - [commit_transfer_ownership](#LiquidityGaugeReward-commit-transfer-ownership)
      - [apply_transfer_ownership](#LiquidityGaugeReward-apply-transfer-ownership)
      - [toggle_external_rewards_claim](#LiquidityGaugeReward-toggle-external-rewards-claim)
      - [minter](#LiquidityGaugeReward-minter)
      - [crv_token](#LiquidityGaugeReward-crv-token)
      - [lp_token](#LiquidityGaugeReward-lp-token)
      - [controller](#LiquidityGaugeReward-controller)
      - [voting_escrow](#LiquidityGaugeReward-voting-escrow)
      - [balance_of](#LiquidityGaugeReward-balance-of)
      - [total_supply](#LiquidityGaugeReward-total-supply)
      - [future_epoch_time](#LiquidityGaugeReward-future-epoch-time)
      - [approved_to_deposit](#LiquidityGaugeReward-approved-to-deposit)
      - [working_balances](#LiquidityGaugeReward-working-balances)
      - [working_supply](#LiquidityGaugeReward-working-supply)
      - [period](#LiquidityGaugeReward-period)
      - [period_timestamp](#LiquidityGaugeReward-period-timestamp)
      - [integrate_inv_supply](#LiquidityGaugeReward-integrate-inv-supply)
      - [integrate_inv_supply_of](#LiquidityGaugeReward-integrate-inv-supply-of)
      - [integrate_checkpoint_of](#LiquidityGaugeReward-integrate-checkpoint-of)
      - [integrate_fraction](#LiquidityGaugeReward-integrate-fraction)
      - [inflation_rate](#LiquidityGaugeReward-inflation-rate)
      - [reward_contract](#LiquidityGaugeReward-reward-contract)
      - [rewarded_token](#LiquidityGaugeReward-rewarded-token)
      - [reward_integral](#LiquidityGaugeReward-reward-integral)
      - [reward_integral_for](#LiquidityGaugeReward-reward-integral-for)
      - [rewards_for](#LiquidityGaugeReward-rewards-for)
      - [claimed_rewards_for](#LiquidityGaugeReward-claimed-rewards-for)
      - [admin](#LiquidityGaugeReward-admin)
      - [future_admin](#LiquidityGaugeReward-future-admin)
      - [is_killed](#LiquidityGaugeReward-is-killed)
      - [is_claiming_rewards](#LiquidityGaugeReward-is-claiming-rewards)
  - [Deploying Liquidity Gauge V3 contract manually](#deploying-liquidity-gauge-v3-contract-manually)
    - [Entry Point methods](#LiquidityGaugeV3-entry-point-methods)
      - [decimals](#LiquidityGaugeV3-decimals)
      - [integrate_checkpoint](#LiquidityGaugeV3-integrate-checkpoint)
      - [user_checkpoint](#LiquidityGaugeV3-user-checkpoint)
      - [claimable_tokens](#LiquidityGaugeV3-claimable-tokens)
      - [reward_contract](#LiquidityGaugeV3-reward-contract)
      - [last_claim](#LiquidityGaugeV3-last-claim)
      - [claimed_reward](#LiquidityGaugeV3-claimed-reward)
      - [claimable_reward](#LiquidityGaugeV3-claimable-reward)
      - [claimable_reward_write](#LiquidityGaugeV3-claimable-reward-write)
      - [set_rewards_receiver](#LiquidityGaugeV3-set-rewards-receiver)
      - [claim_rewards](#LiquidityGaugeV3-claim-rewards)
      - [kick](#LiquidityGaugeV3-kick)
      - [deposit](#LiquidityGaugeV3-deposit)
      - [withdraw](#LiquidityGaugeV3-withdraw)
      - [transfer](#LiquidityGaugeV3-transfer)
      - [transfer_from](#LiquidityGaugeV3-transfer-from)
      - [approve](#LiquidityGaugeV3-approve)
      - [increase_allowance](#LiquidityGaugeV3-increase-allowance)
      - [decrease_allowance](#LiquidityGaugeV3-decrease-allowance)
      - [set_rewards](#LiquidityGaugeV3-set-rewards)
      - [set_killed](#LiquidityGaugeV3-set-killed)
      - [commit_transfer_ownership](#LiquidityGaugeV3-commit-transfer-ownership)
      - [accept_transfer_ownership](#LiquidityGaugeV3-accept-transfer-ownership)
      - [minter](#LiquidityGaugeV3-minter)
      - [crv_token](#LiquidityGaugeV3-crv-token)
      - [lp_token](#LiquidityGaugeV3-lp-token)
      - [controller](#LiquidityGaugeV3-controller)
      - [voting_escrow](#LiquidityGaugeV3-voting-escrow)
      - [future_epoch_time](#LiquidityGaugeV3-future-epoch-time)
      - [balance_of](#LiquidityGaugeV3-balance-of)
      - [total_supply](#LiquidityGaugeV3-total-supply)
      - [allowance](#LiquidityGaugeV3-allowance)
      - [name](#LiquidityGaugeV3-name)
      - [symbol](#LiquidityGaugeV3-symbol)
      - [working_balances](#LiquidityGaugeV3-working-balances)
      - [working_supply](#LiquidityGaugeV3-working-supply)
      - [period](#LiquidityGaugeV3-period)
      - [period_timestamp](#LiquidityGaugeV3-period-timestamp)
      - [integrate_inv_supply](#LiquidityGaugeV3-integrate-inv-supply)
      - [integrate_inv_supply_of](#LiquidityGaugeV3-integrate-inv-supply-of)
      - [integrate_checkpoint_of](#LiquidityGaugeV3-integrate-checkpoint-of)
      - [integrate_fraction](#LiquidityGaugeV3-integrate-fraction)
      - [inflation_rate](#LiquidityGaugeV3-inflation-rate)
      - [reward_tokens](#LiquidityGaugeV3-reward-tokens)
      - [rewards_receiver](#LiquidityGaugeV3-rewards-receiver)
      - [reward_integral](#LiquidityGaugeV3-reward-integral)
      - [reward_integral_for](#LiquidityGaugeV3-reward-integral-for)
      - [admin](#LiquidityGaugeV3-admin)
      - [future_admin](#LiquidityGaugeV3-future-admin)
      - [is_killed](#LiquidityGaugeV3-is-killed)
  - [Deploying Curve Token V3 contract manually](#deploying-curve-token-v3-contract-manually)
    - [Entry Point methods](#CurveTokenV3-entry-point-methods)
      - [decimals](#CurveTokenV3-decimals)
      - [transfer](#CurveTokenV3-transfer)
      - [transfer_from](#CurveTokenV3-transfer-from)
      - [approve](#CurveTokenV3-approve)
      - [increase_allowance](#CurveTokenV3-increase-allowance)
      - [decrease_allowance](#CurveTokenV3-decrease-allowance)
      - [mint](#CurveTokenV3-mint)
      - [burn_from](#CurveTokenV3-burn-from)
      - [set_minter](#CurveTokenV3-set-minter)
      - [set_name](#CurveTokenV3-set-name)
      - [name](#CurveTokenV3-name)
      - [symbol](#CurveTokenV3-symbol)
      - [total_supply](#CurveTokenV3-total-supply)
      - [minter](#CurveTokenV3-minter)
      - [balance_of](#CurveTokenV3-balance-of)
      - [allowance](#CurveTokenV3-allowance)
  - [Deploying ERC20 CRV contract manually](#deploying-erc20-crv-contract-manually)
    - [Entry Point methods](#ERC20CRV-entry-point-methods)
      - [start_epoch_time_write](#ERC20CRV-start-epoch-time-write)
      - [future-epoch-time-write](#ERC20CRV-future-epoch-time-write)
      - [available_supply](#ERC20CRV-available-supply)
      - [update_mining_parameters](#ERC20CRV-update-mining-parameters)
      - [mintable_in_timeframe](#ERC20CRV-mintable-in-timeframe)
      - [set_minter](#ERC20CRV-set-minter)
      - [set_admin](#ERC20CRV-set-admin)
      - [total_supply](#ERC20CRV-total-supply)
      - [allowance](#ERC20CRV-allowance)
      - [transfer](#ERC20CRV-transfer)
      - [transfer_from](#ERC20CRV-transfer-from)
      - [approve](#ERC20CRV-approve)
      - [mint](#ERC20CRV-mint)
      - [burn](#ERC20CRV-burn)
      - [set_name](#ERC20CRV-set-name)
      - [name](#ERC20CRV-name)
      - [symbol](#ERC20CRV-symbol)
      - [decimals](#ERC20CRV-decimals)
      - [balance_of](#ERC20CRV-balance-of)
      - [minter](#ERC20CRV-minter)
      - [admin](#ERC20CRV-admin)
      - [mining_epoch](#ERC20CRV-mining-epoch)
      - [rate](#ERC20CRV-rate)
      - [increase_allowance](#ERC20CRV-increase-allowance)
      - [decrease_allowance](#ERC20CRV-decrease-allowance)
  - [Deploying ERC20 contract manually](#deploying-erc20-contract-manually)
    - [Entry Point methods](#erc20-entry-point-methods)
      - [total_supply](#erc20-total-supply)
      - [allowance](#erc20-allowance)
      - [transfer](#erc20-transfer)
      - [transfer_from](#erc20-transfer-from)
      - [approve](#erc20-approve)
      - [mint](#erc20-mint)
      - [burn](#erc20-burn)
      - [name](#erc20-name)
      - [symbol](#erc20-symbol)
      - [decimals](#erc20-decimals)
      - [balance_of](#erc20-balance-of)
      - [nonce](#erc20-nonce)
      - [increase_allowance](#erc20-increase-allowance)
      - [decrease_allowance](#erc20-decrease-allowance)
      - [package_hash](#erc20-package-hash)
  - [Deploying Fee Distributor contract manually](#deploying-fee-distributor-contract-manually)
    - [Entry Point methods](#FeeDistributor-entry-point-methods)
      - [checkpoint_token](#FeeDistributor-checkpoint-token)
      - [ve_for_at](#FeeDistributor-ve-for-at)
      - [checkpoint-total-supply](#FeeDistributor-checkpoint-total-supply)
      - [claim](#FeeDistributor-claim)
      - [claim_many](#FeeDistributor-claim-many)
      - [burn](#FeeDistributor-burn)
      - [commit_admin](#FeeDistributor-commit-admin)
      - [apply_admin](#FeeDistributor-apply-admin)
      - [toggle_allow_checkpoint_token](#FeeDistributor-toggle-allow-checkpoint-token)
      - [kill_me](#FeeDistributor-kill-me)
      - [recover_balance](#FeeDistributor-recover-balance)
      - [start_time](#FeeDistributor-start-time)
      - [time_cursor](#FeeDistributor-time-cursor)
      - [time_cursor_of](#FeeDistributor-time-cursor-of)
      - [user_epoch_of](#FeeDistributor-user-epoch-of)
      - [last_token_time](#FeeDistributor-last-token-time)
      - [tokens_per_week](#FeeDistributor-tokens-per-week)
      - [voting_escrow](#FeeDistributor-voting-escrow)
      - [token](#FeeDistributor-token)
      - [total_received](#FeeDistributor-total-received)
      - [token_last_balance](#FeeDistributor-token-last-balance)
      - [ve_supply](#FeeDistributor-ve-supply)
      - [admin](#FeeDistributor-admin)
      - [future_admin](#FeeDistributor-future-admin)
      - [can_checkpoint_token](#FeeDistributor-can-checkpoint-token)
      - [emergency_return](#FeeDistributor-emergency-return)
      - [is_killed](#FeeDistributor-is-killed)
  - [Deploying Gauge Proxy contract manually](#deploying-gauge-proxy-contract-manually)
    - [Entry Point methods](#GaugeProxy-entry-point-methods)
      - [commit_set_admins](#GaugeProxy-commit-set-admins)
      - [accept_set_admins](#GaugeProxy-accept-set-admins)
      - [commit_transfer_ownership](#GaugeProxy-commit-transfer-ownership)
      - [accept_transfer_ownership](#GaugeProxy-accept-transfer-ownership)
      - [set_killed](#GaugeProxy-set-killed)
      - [set_rewards](#GaugeProxy-set-rewards)
      - [ownership_admin](#GaugeProxy-ownership-admin)
      - [emergency_admin](#GaugeProxy-emergency-admin)
      - [future_ownership_admin](#GaugeProxy-future-ownership-admin)
      - [future_emergency_admin](#GaugeProxy-future-emergency-admin)
  - [Deploying I Reward Distribution Recipient contract manually](#deploying-i-reward-distribution-recipient-contract-manually)
    - [Entry Point methods](#IRewardDistributionRecipient-entry-point-methods)
      - [set_reward_distribution](#IRewardDistributionRecipient-set-reward-distribution)
      - [owner](#IRewardDistributionRecipient-owner)
      - [is_owner](#IRewardDistributionRecipient-is-owner)
      - [renounce_ownership](#IRewardDistributionRecipient-renounce-ownership)
      - [transfer_ownership](#IRewardDistributionRecipient-transfer-ownership)
  - [Deploying Vesting Escrow contract manually](#deploying-vesting-escrow-contract-manually)
    - [Entry Point methods](#VestingEscrow-entry-point-methods)
      - [add_tokens](#VestingEscrow-add-tokens)
      - [fund](#VestingEscrow-fund)
      - [toggle_disable](#VestingEscrow-toggle-disable)
      - [disable_can_disable](#VestingEscrow-disable-can-disable)
      - [disable_fund_admins](#VestingEscrow-disable-fund-admins)
      - [vested_supply](#VestingEscrow-vested-supply)
      - [lock_supply](#VestingEscrow-lock-supply)
      - [vested_of](#VestingEscrow-vested-of)
      - [balance_of](#VestingEscrow-balance-of)
      - [locked_of](#VestingEscrow-locked-of)
      - [claim](#VestingEscrow-claim)
      - [commit_transfer_ownership](#VestingEscrow-commit-transfer-ownership)
      - [apply_transfer_ownership](#VestingEscrow-apply-transfer-ownership)
      - [token](#VestingEscrow-token)
      - [start_time](#VestingEscrow-start-time)
      - [end_time](#VestingEscrow-end-time)
      - [initial_locked](#VestingEscrow-initial-locked)
      - [total_claimed](#VestingEscrow-total-claimed)
      - [initial_locked_supply](#VestingEscrow-initial-locked-supply)
      - [unallocated_supply](#VestingEscrow-unallocated-supply)
      - [can_disable](#VestingEscrow-can-disable)
      - [disabled_at](#VestingEscrow-disabled-at)
      - [admin](#VestingEscrow-admin)
      - [future_admin](#VestingEscrow-future-admin)
      - [fund_admins_enabled](#VestingEscrow-fund-admins-enabled)
      - [fund_admins](#VestingEscrow-fund-admins)
  - [Deploying Vesting Escrow Simple contract manually](#deploying-vesting-escrow-simple-contract-manually)
    - [Entry Point methods](#VestingEscrowSimple-entry-point-methods)
      - [initialize](#VestingEscrowSimple-initialize)
      - [toggle_disable](#VestingEscrowSimple-toggle-disable)
      - [disable_can_disable](#VestingEscrowSimple-disable-can-disable)
      - [vested_supply](#VestingEscrowSimple-vested-supply)
      - [lock_supply](#VestingEscrowSimple-lock-supply)
      - [vested_of](#VestingEscrowSimple-vested-of)
      - [balance_of](#VestingEscrowSimple-balance-of)
      - [locked_of](#VestingEscrowSimple-locked-of)
      - [claim](#VestingEscrowSimple-claim)
      - [commit_transfer_ownership](#VestingEscrowSimple-commit-transfer-ownership)
      - [apply_transfer_ownership](#VestingEscrowSimple-apply-transfer-ownership)
      - [token](#VestingEscrowSimple-token)
      - [start_time](#VestingEscrowSimple-start-time)
      - [end_time](#VestingEscrowSimple-end-time)
      - [initial_locked](#VestingEscrowSimple-initial-locked)
      - [total_claimed](#VestingEscrowSimple-total-claimed)
      - [initial_locked_supply](VestingEscrowSimple-initial-locked-supply)
      - [can_disable](#VestingEscrowSimple-can-disable)
      - [disabled_at](#VestingEscrowSimple-disabled-at)
      - [admin](#VestingEscrowSimple-admin)
      - [future_admin](#VestingEscrowSimple-future-admin)
  - [Deploying Vesting Escrow Factory contract manually](#deploying-vesting-escrow-factory-contract-manually)
    - [Entry Point methods](#VestingEscrowFactory-entry-point-methods)
      - [deploy_vesting_contract](#VestingEscrowFactory-deploy-vesting-contract)
      - [commit_transfer_ownership_vef](#VestingEscrowFactory-commit-transfer-ownership-vef)
      - [apply_transfer_ownership_vef](#VestingEscrowFactory-apply-transfer-ownership-vef)
      - [future_admin_vef](#VestingEscrowFactory-future-admin-vef)
      - [admin_vef](#VestingEscrowFactory-admin-vef)
      - [target](#VestingEscrowFactory-target)
      - [initialize](#VestingEscrowFactory-initialize)
      - [toggle_disable](#VestingEscrowFactory-toggle-disable)
      - [disable_can_disable](#VestingEscrowFactory-disable-can-disable)
      - [vested_supply](#VestingEscrowFactory-vested-supply)
      - [lock_supply](#VestingEscrowFactory-lock-supply)
      - [vested_of](#VestingEscrowFactory-vested-of)
      - [balance_of](#VestingEscrowFactory-balance-of)
      - [locked_of](#VestingEscrowFactory-locked-of)
      - [claim](#VestingEscrowFactory-claim)
      - [commit_transfer_ownership](#VestingEscrowFactory-commit-transfer-ownership)
      - [apply_transfer_ownership](#VestingEscrowFactory-apply-transfer-ownership)
      - [token](#VestingEscrowFactory-token)
      - [start_time](#VestingEscrowFactory-start-time)
      - [end_time](#VestingEscrowFactory-end-time)
      - [initial_locked](#VestingEscrowFactory-initial-locked)
      - [total_claimed](#VestingEscrowFactory-total-claimed)
      - [initial_locked_supply](VestingEscrowFactory-initial-locked-supply)
      - [can_disable](#VestingEscrowFactory-can-disable)
      - [disabled_at](#VestingEscrowFactory-disabled-at)
      - [admin](#VestingEscrowFactory-admin)
      - [future_admin](#VestingEscrowFactory-future-admin)
  - [Deploying Voting Escrow contract manually](#deploying-voting-escrow-contract-manually)
    - [Entry Point methods](#VotingEscrow-entry-point-methods)
      - [commit_transfer_ownership](#VotingEscrow-commit-transfer-ownership)
      - [apply_transfer_ownership](#VotingEscrow-apply-transfer-ownership)
      - [get_last_user_slope](#VotingEscrow-get-last-user-slope)
      - [user_point_history_ts](#VotingEscrow-user-point-history-ts)
      - [locked_end](#VotingEscrow-locked-end)
      - [checkpoint](#VotingEscrow-checkpoint)
      - [deposit_for](#VotingEscrow-deposit-for)
      - [create_lock](#VotingEscrow-create-lock)
      - [increase_amount](#VotingEscrow-increase-amount)
      - [increase_unlock_time](#VotingEscrow-increase-unlock-time)
      - [withdraw](#VotingEscrow-withdraw)
      - [balance_of](#VotingEscrow-balance-of)
      - [balance_of_at](#VotingEscrow-balance-of-at)
      - [total_supply](#VotingEscrow-total-supply)
      - [total_supply_at](#VotingEscrow-total-supply-at)
      - [change_controller](#VotingEscrow-change-controller)
      - [future_admin](#VotingEscrow-future-admin)
      - [admin](#VotingEscrow-admin)
      - [token](#VotingEscrow-token)
      - [supply](#VotingEscrow-supply)
      - [locked](#VotingEscrow-locked)
      - [epoch](#VotingEscrow-epoch)
      - [point_history](#VotingEscrow-point-history)
      - [user_point_history](#VotingEscrow-user-point-history)
      - [user_point_epoch](#VotingEscrow-user-point-epoch)
      - [slope_changes](#VotingEscrow-slope-changes)
      - [controller](#VotingEscrow-controller)
      - [transfers_enabled](#VotingEscrow-transfers-enabled)
      - [name](#VotingEscrow-name)
      - [symbol](#VotingEscrow-symbol)
      - [version](#VotingEscrow-version)
      - [decimals](#VotingEscrow-decimals)

## Interacting with the contract

You need to have `casper-client` and `jq` installed on your system to run the examples. The instructions have been tested on Ubuntu 20.04.0 LTS.

## Install the prerequisites

You can install the required software by issuing the following commands. If you are on an up-to-date Casper node, you probably already have all of the prerequisites installed so you can skip this step.

## Note: If any command fails try again by restarting the terminal to reset the enviornment variable.

## Update package repositories

```
sudo apt update
```

## Install the command-line JSON processor

```
sudo apt install jq -y
```

## Install rust

Choose cutomize intallation to install nightly version
Install the nightly version (by default stable toolchain is installed)

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```
rustup install nightly-2022-04-04
rustup install nightly-2022-11-17
```

## Check that nightly toolchain version is installed(this will list stable and nightly versions)

```
rustup toolchain list
```

## Set rust nightly as default (this is the older one for contracts)

```
rustup default nightly-2022-04-04-x86_64-unknown-linux-gnu
```

## Install wasm32-unknown-unknown

```
rustup target add wasm32-unknown-unknown
```

## Rust Version

```
rustup --version
```

## Install Cmake

```
sudo apt-get -y install cmake
```

Note:https://cgold.readthedocs.io/en/latest/first-step/installation.html

## Check if cmake is installed properly

```
cmake --version
```

## Install the Casper Crates

```
cargo +nightly-2022-11-17-x86_64-unknown-linux-gnu install cargo-casper
```

## Add Casper repository

```
echo "deb https://repo.casperlabs.io/releases" bionic main | sudo tee -a /etc/apt/sources.list.d/casper.list
```

```
curl -O https://repo.casperlabs.io/casper-repo-pubkey.asc
```

```
sudo apt-key add casper-repo-pubkey.asc
```

```
sudo apt update
```

```
sudo apt install libssl-dev
```

```
sudo apt install pkg-config
```

## Install the Casper client software

```
cargo +nightly-2022-11-17-x86_64-unknown-linux-gnu install casper-client
```

## To check Casper Client Version

```
casper-client --version
```

# Additonal commands for help

```
casper-client --help
casper-client <command> --help
```

## Generate the keys

```
casper-client keygen script/keys
```

## Fund the key

The keys can be funded from casper live website [testnet faucet](https://testnet.cspr.live/tools/faucet). Requires chrome browser and the casper signer extension. You should import the keys that were generated in the previous step

## Deployment

A handy script is being created to deploy all required dependencies sequentially for curve DAO contract deployment.

Note:- Following are the prerequisites to be considered before executing the deploy command

- Make sure 'Node' is installed in your machine _(use node version 16.14.0 for avoiding unknown errors)_
- Your deployemnt keys should be in a new folder named 'keys' at 'script/keys'

### Deploy stakeable contract

Use the following command to execute the deployments

```
make deploy
```

After deployments, hashes can be found in the folder 'script/hashes' and a zip is also created 'hashes.zip'

## Install

Make sure `wasm32-unknown-unknown` is installed.

```
  make prepare
```

It's also recommended to have [wasm-strip](https://github.com/WebAssembly/wabt)
available in your PATH to reduce the size of compiled Wasm.

## Build individual Smart Contract

You can run this commands to build individual smart contracts.

```
make build-contract-curve-token-v3
make build-liquidity-gauge-reward-wrapper-session-code
make build-i-reward-distribution-recipient
make build-liquidity-gauge-wrapper-session-code
make build-contract-erc20-crv
make build-contract-fee-distributor
make build-contract-gauge-controller
make build-contract-gauge-proxy
make build-contract-liquidity-gauge-reward
make build-contract-liquidity-gauge-reward-wrapper
make build-contract-liquidity-gauge-wrapper
make build-contract-minter
make build-contract-reward-only-gauge
make build-contract-vesting-escrow
make build-contract-vesting-escrow-factory
make build-contract-liquidity-gauge-v3
make build-contract-vesting-escrow-simple
make build-contract-voting-escrow
make build-contract-ownable
make build-lp-token-wrapper
make build-curve-rewards
```

## Test individual Smart Contract

You can run this commands to build individual smart contracts.

```
make test-curve-token-v3
make test-curve-erc20
make test-erc20-crv
make test-fee-distributor
make test-gauge-controller
make test-gauge-proxy
make test-liquidity-gauge-reward
make test-liquidity-gauge-reward-wrapper
make test-liquidity-gauge-wrapper
make test-minter
make test-reward-only-gauge
make test-vesting-escrow
make test-vesting-escrow-factory
make test-voting-escrow
make test-ownable
make test-i-reward-distribution-recipient
make test-lp-token-wrapper
make test-curve-rewards
make test-liquidity-gauge-v3
```

# Note: High processing power and memory is required to run liquidity gauge v3 test cases.

## Test Liquidity Gauge V3 Smart Contract

```
  make test-liquidity-gauge-v3
```

## Run All Smart Contracts

Run this command to build & test all smart contract.

```
make all
```

## Check Lint in Smart Contracts

Run this command to build & test all smart contract.

```
make check-lint
```

## Deploying Reward Only Gauge contract manually

If you need to deploy the `Reward Only Gauge` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - admin : Admin account hash for curve-dao
  - lp_token : LP Token contract package hash
  - contract_name : Contract name for deployment

Following is the command to deploy the `Reward Only Gauge contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 240000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="admin:Key='key'" \
    --session-arg="lp_token:Key='liquidity pool contract address'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="RewardOnlyGauge-entry-point-methods"></a>

Following are the RewardOnlyGauge's entry point methods.

- ### decimals <a id="RewardOnlyGauge-decimals"></a>

  Returns U256 decimal places.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### reward_contract <a id="RewardOnlyGauge-reward-contract"></a>

  Returns zero address if no reward is active.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### last_claim <a id="RewardOnlyGauge-last-claim"></a>

  Returns rewards that are claimed at most once per hour in order to reduce gas costs.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### claimed_reward <a id="RewardOnlyGauge-claimed-reward"></a>

  Returns U256 Total amount of `token` already claimed by `addr`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |
  | token          | Key  |

  This method **returns** `U256`.

- ### claimable_reward <a id="RewardOnlyGauge-claimable-reward"></a>

  Returns U256 Claimable reward token amount.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |
  | token          | Key  |

  This method **returns** `U256`.

- ### claimable_reward_write <a id="RewardOnlyGauge-claimable-reward-write"></a>

  Returns U256 Claimable reward token amount.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |
  | token          | Key  |

  This method **returns** `U256`.

- ### set_rewards_receiver <a id="RewardOnlyGauge-set-rewards-receiver"></a>

  Receiver address for any rewards claimed via `claim_rewards`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | receiver       | Key  |

  This method **returns** nothing.

- ### claim_rewards <a id="RewardOnlyGauge-claim-rewards"></a>

  Claim available reward tokens for `addr

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | addr           | Option`<Key>` |
  | receiver       | Option`<Key>` |

  This method **returns** nothing.

- ### deposit <a id="RewardOnlyGauge-deposit"></a>

  Deposit `value` LP tokens

  Following is the table of parameters.

  | Parameter Name | Type           |
  | -------------- | -------------- |
  | value          | U256           |
  | addr           | Option`<Key>`  |
  | claim_rewards  | Option`<bool>` |

  This method **returns** nothing.

- ### withdraw <a id="RewardOnlyGauge-withdraw"></a>

  Withdraw `value` LP tokens

  Following is the table of parameters.

  | Parameter Name | Type           |
  | -------------- | -------------- |
  | value          | U256           |
  | claim_rewards  | Option`<bool>` |

  This method **returns** nothing.

- ### transfer <a id="RewardOnlyGauge-transfer"></a>

  Returns Result<(), u32> if amount transfered successfully return ok().

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- ### transfer_from <a id="RewardOnlyGauge-transfer-from"></a>

  Returns Result<(), u32> if amount transfered successfully return ok().

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | recipient      | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- ### approve <a id="RewardOnlyGauge-approve"></a>

  Returns Result<(),u32> on success.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** nothing.

- ### increase_allowance <a id="RewardOnlyGauge-increase-allowance"></a>

  Returns Result<(),u32> on success.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** `Result<(),u32>`.

- ### decrease_allowance <a id="RewardOnlyGauge-decrease-allowance"></a>

  Returns Result<(),u32> on success.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** `Result<(),u32>`.

- ### set_rewards <a id="RewardOnlyGauge-set-rewards"></a>

  Set the active reward contract.

  Following is the table of parameters.

  | Parameter Name  | Type          |
  | --------------- | ------------- |
  | reward_contract | Key           |
  | claim_sig       | String        |
  | reward_tokens   | Vec`<String>` |

  This method **returns** nothing.

- ### commit_transfer_ownership <a id="RewardOnlyGauge-commit-transfer-ownership"></a>

  Transfer ownership of GaugeController to `addr`

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** nothing.

- ### accept_transfer_ownership <a id="RewardOnlyGauge-accept-transfer-ownership"></a>

  Accept a pending ownership transfer

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### decimals <a id="RewardOnlyGauge-decimals"></a>

  Returns the number of decimals for this token.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `u8`.

- ### future_admin <a id="RewardOnlyGauge-future-admin"></a>

  Returns the future admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### admin <a id="RewardOnlyGauge-admin"></a>

  Returns the admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### reward_integral_for <a id="RewardOnlyGauge-reward-integral-for"></a>

  Returns the U256 for reward_token and cliaming address.

  Following is the table of parameters.

  | Parameter Name   | Type |
  | ---------------- | ---- |
  | reward_token     | Key  |
  | claiming_address | Key  |

  This method **returns** `U256`.

- ### reward_integral <a id="RewardOnlyGauge-reward-integral"></a>

  Returns the U256 for reward_token .

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### claim_sig <a id="RewardOnlyGauge-claim-sig"></a>

  Returns the String.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `String`.

- ### rewards_receiver <a id="RewardOnlyGauge-rewards-receiver"></a>

  Returns the address of receiver.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `Key`.

- ### reward_balances <a id="RewardOnlyGauge-reward-balances"></a>

  Returns the U256(amount) of receiver_balance.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### reward_tokens <a id="RewardOnlyGauge-reward-tokens"></a>

  Returns the key of reward_tokens.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | index          | U256 |

  This method **returns** `Key`.

- ### lp_token <a id="RewardOnlyGauge-lp-token"></a>

  Returns the key of lp_token.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### balance_of <a id="RewardOnlyGauge-balance-of"></a>

  Returns the balance of provided key.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### total_supply <a id="RewardOnlyGauge-total-supply"></a>

  Returns the total_supply.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### allowance <a id="RewardOnlyGauge-allowance"></a>

  Returns the allowance of provided key.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | spender        | Key  |

  This method **returns** `U256`.

- ### name <a id="RewardOnlyGauge-name"></a>

  Returns the name.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `String`.

- ### symbol <a id="RewardOnlyGauge-symbol"></a>

  Returns the symbol.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `String`.

## Deploying Gauge Controller contract manually

If you need to deploy the `Gauge Controller` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - token : Erc20 Crv token contract package hash
  - voting_escrow : Voting escrow deployed contract package hash
  - contract_name : Contract name for deployment

Following is the command to deploy the `Gauge Controller contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 280000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="token:key='Erc20 Crv Contract Address'" \
    --session-arg="voting_escrow:key='voting_escrow Contract Address'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="GaugeController-entry-point-methods"></a>

Following are the GaugeController's entry point methods.

- ### commit_transfer_ownership <a id="GaugeController-commit-transfer-ownership"></a>

  Transfer ownership of GaugeController to `addr`

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** nothing.

- ### apply_transfer_ownership <a id="GaugeController-apply-transfer-ownership"></a>

  Apply a pending ownership transfer

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### gauge_types <a id="GaugeController-gauge-types"></a>

  Get gauge type for address. Returns Gauge Type id.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** `U128`.

- ### add_gauge <a id="GaugeController-add-gauge"></a>

  Add gauge `addr` of type `gauge_type` with weight `weight`.

  Following is the table of parameters.

  | Parameter Name | Type           |
  | -------------- | -------------- |
  | addr           | Key            |
  | type_id        | (bool,U128)    |
  | weight         | Option`<U256>` |

  This method **returns** nothing.

- ### checkpoint <a id="GaugeController-checkpoint"></a>

  Checkpoint to fill data common for all gauges

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### checkpoint_gauge <a id="GaugeController-checkpoint-gauge"></a>

  Checkpoint to fill data for both a specific gauge and common for all gauges.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** nothing.

- ### gauge_relative_weight <a id="GaugeController-gauge-relative-weight"></a>

  Returns Gauge relative weight

  Following is the table of parameters.

  | Parameter Name | Type           |
  | -------------- | -------------- |
  | addr           | Key            |
  | time           | Option`<U256>` |

  This method **returns** `U256`.

- ### gauge_relative_weight_write <a id="GaugeController-gauge-relative-weight-write"></a>

  Returns gauge weight normalized to 1e9 and also fill all the unfilled values for type and gauge records.

  Following is the table of parameters.

  | Parameter Name | Type           |
  | -------------- | -------------- |
  | addr           | Key            |
  | time           | Option`<U256>` |

  This method **returns** `U256`.

- ### add_type <a id="GaugeController-add-type"></a>

  Add gauge type with name and weight.

  Following is the table of parameters.

  | Parameter Name | Type           |
  | -------------- | -------------- |
  | name           | String         |
  | weight         | Option`<U256>` |

  This method **returns** nothing.

- ### change_type_weight <a id="GaugeController-change-type-weight"></a>

  Change gauge type `type_id` weight to `weight`.

  Following is the table of parameters.

  | Parameter Name | Type        |
  | -------------- | ----------- |
  | type_id        | (bool,U128) |
  | weight         | U256        |

  This method **returns** nothing.

- ### change_gauge_weight <a id="GaugeController-change-gauge-weight"></a>

  Change weight of gauge `addr` to `weight`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |
  | weight         | U256 |

  This method **returns** nothing.

- ### vote_for_gauge_weights <a id="GaugeController-vote-for-gauge-weights"></a>

  Allocate voting power for changing pool weights.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | gauge_addr     | Key  |
  | user_weight    | U256 |

  This method **returns** nothing.

- ### get_gauge_weight <a id="GaugeController-get-gauge-weight"></a>

  Returns current gauge weight.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** `U256`.

- ### get_type_weight <a id="GaugeController-get-type-weight"></a>

  Returns current type weight.

  Following is the table of parameters.

  | Parameter Name | Type        |
  | -------------- | ----------- |
  | type_id        | (bool,U128) |

  This method **returns** `U256`.

- ### get_total_weight <a id="GaugeController-get-total-weight"></a>

  Returns current total (type-weighted) weight.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### get_weights_sum_per_type <a id="GaugeController-get-weights-sum-per-type"></a>

  Returns Sum of gauge weights.

  Following is the table of parameters.

  | Parameter Name | Type        |
  | -------------- | ----------- |
  | type_id        | (bool,U128) |

  This method **returns** `U256`.

- ### future_admin <a id="GaugeController-future-admin"></a>

  Returns the future admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### admin <a id="GaugeController-admin"></a>

  Returns the admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### token <a id="GaugeController-token"></a>

  Returns the key of CRV token.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### voting_escrow <a id="GaugeController-voting-escrow"></a>

  Returns the key of voting_escrow.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### n_gauge_types <a id="GaugeController-n-gauge-types"></a>

  Returns U128.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U128`.

- ### n_gauges <a id="GaugeController-n-gauges"></a>

  Returns U128.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U128`.

- ### gauge_type_names <a id="GaugeController-gauge-type-names"></a>

  Returns String.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | U128 |

  This method **returns** `String`.

- ### gauges <a id="GaugeController-gauges"></a>

  Returns Key. Needed for enumeration.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | U256 |

  This method **returns** `Key`.

- ### vote_user_slopes <a id="GaugeController-vote-user-slopes"></a>

  Returns VotedSlope by providing address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | spender        | Key  |

  This method **returns** `VotedSlope`.

- ### vote_user_power <a id="GaugeController-vote-user-power"></a>

  Returns Total vote power used by user

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### last_user_vote <a id="GaugeController-last-user-vote"></a>

  Returns Last user vote's timestamp for each gauge address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | spender        | Key  |

  This method **returns** `U256`.

- ### points_weight <a id="GaugeController-points-weight"></a>

  Returns Point by providing gauge address and time.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | spender        | U256 |

  This method **returns** `Point`.

- ### time_weight <a id="GaugeController-time-weight"></a>

  Returns last scheduled time (next week).

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | U256 |

  This method **returns** `U256`.

- ### points_sum <a id="GaugeController-points-sum"></a>

  Returns Point by providing gauge address and time.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | U128 |
  | spender        | U256 |

  This method **returns** `Point`.

- ### time_sum <a id="GaugeController-time-sum"></a>

  Returns last scheduled time (next week).

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | U256 |

  This method **returns** `U256`.

- ### points_total <a id="GaugeController-points-total"></a>

  Returns total weight.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | U256 |

  This method **returns** `U256`.

- ### time_total <a id="GaugeController-time-total"></a>

  Returns last scheduled time.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### points_type_weight <a id="GaugeController-points-type-weight"></a>

  Returns type weight by providing type id and time.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | U128 |
  | spender        | U256 |

  This method **returns** `U256`.

- ### time_type_weight <a id="GaugeController-time-type-weight"></a>

  Returns last scheduled time (next week) by providing type id.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | U256 |

  This method **returns** `U256`.

## Deploying Minter contract manually

If you need to deploy the `Minter` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - token : Erc20 Crv token contract package hash
  - controller : Gauge controller deployed contract package hash
  - contract_name : Contract name for deployment

Following is the command to deploy the `Minter contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 150000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="token:key='Token Address'" \
    --session-arg="controller:key='Controller Contract Address'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="Minter-entry-point-methods"></a>

Following are the Minter's entry point methods.

- ### mint <a id="Minter-mint"></a>

  Mint everything which belongs to `msg.sender` and send to them.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | gauge_addr     | Key  |

  This method **returns** nothing.

- ### mint_many <a id="Minter-mint-many"></a>

  Mint everything which belongs to `msg.sender` across multiple gauges.

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | gauge_addr     | Vec`<String>` |

  This method **returns** nothing.

- ### mint_for <a id="Minter-mint-for"></a>

  Mint tokens for `for`. Only possible when `msg.sender` has been approved via `toggle_approve_mint`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | gauge_addr     | Key  |
  | for            | Key  |

  This method **returns** nothing.

- ### toggle_approve_mint <a id="Minter-toggle-approve-mint"></a>

  Allow `minting_user` to mint for `msg.sender`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | minting_user   | Key  |

  This method **returns** nothing.

- ### token <a id="Minter-token"></a>

  Returns the key of token.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### Controller <a id="Minter-Controller"></a>

  Returns the key of Controller.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### allowed_to_mint_for <a id="Minter-allowed-to-mint-for"></a>

  Returns bool. This is used to check user is allowed to mint or not.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | spender        | Key  |

  This method **returns** `bool`.

- ### minted <a id="Minter-minted"></a>

  Returns U256 by providing user and gauge keys.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | spender        | Key  |

  This method **returns** `U256`.

## Deploying Curve Rewards contract manually

If you need to deploy the `Curve Rewards` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - token : Erc20 token contract package hash
  - reward : Reward contract deployed contract package hash
  - contract_name : Contract name for deployment

Following is the command to deploy the `Curve Rewards contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 290000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="token:key='Token(ERC20) Contract Address'" \
    --session-arg="reward:key='Reward(ERC20) Contract Address'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="CurveRewards-entry-point-methods"></a>

Following are the CurveRewards's entry point methods.

- ### last_time_reward_applicable <a id="CurveRewards-last-time-reward-applicable"></a>

  Returns the min value between blocktime and period_finish.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### reward_per_token <a id="CurveRewards-reward-per-token"></a>

  Return the rewards that is earned.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### earned <a id="CurveRewards-earned"></a>

  Return the earned amount of account that is provided.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | account        | Key  |

  This method **returns** `U256`.

- ### stake <a id="CurveRewards-stake"></a>

  Stake the amount.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | amount         | U256 |

  This method **returns** nothing.

- ### withdraw <a id="CurveRewards-withdraw"></a>

  Withdraw the amount.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | amount         | U256 |

  This method **returns** nothing.

- ### exit <a id="CurveRewards-exit"></a>

  Exit and withdraw all amount.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### get_reward <a id="CurveRewards-get-reward"></a>

  Use to get the reward that is earned by providing the reward amount.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | reward         | U256 |

  This method **returns** nothing.

- ### notify_reward_amount <a id="CurveRewards-notify-reward-amount"></a>

  Use to notify the reward amount.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### total_supply <a id="CurveRewards-total-supply"></a>

  Returns the total supply.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### balance_of <a id="CurveRewards-balance-of"></a>

  Returns the balance of provided address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### stake_lp <a id="CurveRewards-stake-lp"></a>

  Stake function of lp token wrapper.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | amount         | U256 |

  This method **returns** nothing.

- ### withdraw_lp <a id="CurveRewards-withdraw-lp"></a>

  Withdraw function lp token wrapper.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | amount         | U256 |

  This method **returns** nothing.

- ### set_reward_distribution <a id="CurveRewards-set-reward-distribution"></a>

  Set the reward distribution key.

  Following is the table of parameters.

  | Parameter Name      | Type |
  | ------------------- | ---- |
  | reward_distribution | Key  |

  This method **returns** nothing.

- ### owner <a id="CurveRewards-owner"></a>

  Returns owner key.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### is_owner <a id="CurveRewards-is-owner"></a>

  Returns the bool.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `bool`.

- ### renounce_ownership <a id="CurveRewards-renounce-ownership"></a>

  Renouncing ownership will leave the contract without an owner,thereby removing any functionality that is only available to the owner.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### transfer_ownership <a id="CurveRewards-transfer-ownership"></a>

  Transfers ownership of the contract to a new account (`newOwner`).Can only be called by the current owner.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | new_owner      | Key  |

  This method **returns** nothing.

- ### uni <a id="CurveRewards-uni"></a>

  Return the address of uni(ERC20).

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### snx <a id="CurveRewards-snx"></a>

  Return the address of snx(ERC20).

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### duration <a id="CurveRewards-duration"></a>

  Return the duration.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### period_finish <a id="CurveRewards-period-finish"></a>

  Return the period_finish value.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### reward_rate <a id="CurveRewards-reward-rate"></a>

  Return the reward_rate value.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### last_update_time <a id="CurveRewards-last-update-time"></a>

  Return the last_update_time value.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### reward_per_token_stored <a id="CurveRewards-reward-per-token-stored"></a>

  Return the reward_per_token_stored value.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### user_reward_per_token_paid <a id="CurveRewards-user-reward-per-token-paid"></a>

  Return the user_reward_per_token_paid value by providing the account.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | account        | Key  |

  This method **returns** `U256`.

- ### rewards <a id="CurveRewards-rewards"></a>

  Return the rewards value by providing the account.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | account        | Key  |

  This method **returns** `U256`.

## Deploying Lp Token Wrapper contract manually

If you need to deploy the `Lp Token Wrapper` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - uni : ERC20 token contract package hash
  - contract_name : Contract name for deployment

Following is the command to deploy the `Lp Token Wrapper contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 110000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="uni:Key='Address of ERC20 Contract Package Hash'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="LpTokenWrapper-entry-point-methods"></a>

Following are the LpTokenWrapper's entry point methods.

- ### stake <a id="LpTokenWrapper-stake"></a>

  Stake the amount.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | amount         | U256 |

  This method **returns** nothing.

- ### withdraw <a id="LpTokenWrapper-withdraw"></a>

  Withdraw the amount.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | amount         | U256 |

  This method **returns** nothing.

- ### total_supply <a id="LpTokenWrapper-total-supply"></a>

  Returns the total supply.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### balance_of <a id="LpTokenWrapper-balance-of"></a>

  Returns the balance of provided address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### uni <a id="LpTokenWrapper-uni"></a>

  Return the address of uni(ERC20).

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

## Deploying Ownable contract manually

If you need to deploy the `Ownable` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - uni : ERC20 token contract package hash
  - contract_name : Contract name for deployment

Following is the command to deploy the `Ownable contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 100000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="Ownable-entry-point-methods"></a>

Following are the Ownable's entry point methods.

- ### owner <a id="Ownable-owner"></a>

  Returns owner key.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### is_owner <a id="Ownable-is-owner"></a>

  Returns the bool.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `bool`.

- ### renounce_ownership <a id="Ownable-renounce-ownership"></a>

  Renouncing ownership will leave the contract without an owner,thereby removing any functionality that is only available to the owner.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### transfer_ownership <a id="Ownable-transfer-ownership"></a>

  Transfers ownership of the contract to a new account (`newOwner`).Can only be called by the current owner.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | new_owner      | Key  |

  This method **returns** nothing.

## Deploying Liquidity Gauge Wrapper contract manually

If you need to deploy the `Liquidity Gauge Wrapper` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - name : ERC20 token's name to be set
  - symbol : ERC20 token's symbol to be set
  - gauge : Gauge controller deployed contract package hash
  - admin : Admin who can kill the gauge
  - contract_name : Contract name for deployment

Following is the command to deploy the `Liquidity Gauge Wrapper contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 230000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:String='Token full name'" \
    --session-arg="symbol:String='Token symbol'" \
    --session-arg="gauge:Key='Liquidity gauge contract address'" \
    --session-arg="admin:Key='Admin who can kill the gauge'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="LiquidityGaugeWrapper-entry-point-methods"></a>

Following are the LiquidityGaugeWrapper's entry point methods.

- ### user_checkpoint <a id="LiquidityGaugeWrapper-user-checkpoint"></a>

  Record a checkpoint for `addr`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** `bool`.

- ### claimable_tokens <a id="LiquidityGaugeWrapper-claimable-tokens"></a>

  Return the number of claimable tokens per user.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** `U256`.

- ### claim_tokens <a id="LiquidityGaugeWrapper-claim-tokens"></a>

  Claim mintable CR.

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | addr           | Option`<Key>` |

  This method **returns** nothing.

- ### set_approve_deposit <a id="LiquidityGaugeWrapper-set-approve-deposit"></a>

  Set whether `addr` can deposit tokens for `msg.sender`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |
  | can_deposit    | bool |

  This method **returns** nothing.

- ### deposit <a id="LiquidityGaugeWrapper-deposit"></a>

  Deposit `value` LP tokens.

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | value          | U256          |
  | addr           | Option`<Key>` |

  This method **returns** nothing.

- ### withdraw <a id="LiquidityGaugeWrapper-withdraw"></a>

  Withdraw `value` LP tokens.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | value          | U256 |

  This method **returns** nothing.

- ### allowance <a id="LiquidityGaugeWrapper-allowance"></a>

  Returns an U256 specifying the amount of tokens still available for the spender..

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | spender        | Key  |

  This method **returns** `U256`.

- ### transfer <a id="LiquidityGaugeWrapper-transfer"></a>

  Returns Result<(), u32>. Transfer token for a specified address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- ### transfer_from <a id="LiquidityGaugeWrapper-transfer-from"></a>

  Returns Result<(), u32>. Transfer tokens from one address to another.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | recipient      | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- ### approve <a id="LiquidityGaugeWrapper-approve"></a>

  Approve the passed address to transfer the specified amount of
  tokens on behalf of msg.sender.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** nothing.

- ### increase_allowance <a id="LiquidityGaugeWrapper-increase-allowance"></a>

  Increase the allowance granted to `spender` by the caller.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** `Result<(),u32>`.

- ### decrease_allowance <a id="LiquidityGaugeWrapper-decrease-allowance"></a>

  Decrease the allowance granted to `spender` by the caller.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** `Result<(),u32>`.

- ### kill_me <a id="LiquidityGaugeWrapper-kill-me"></a>

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### commit_transfer_ownership <a id="LiquidityGaugeWrapper-commit-transfer-ownership"></a>

  Transfer ownership of GaugeController to `addr`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** nothing.

- ### apply_transfer_ownership <a id="LiquidityGaugeWrapper-apply-transfer-ownership"></a>

  Apply a pending ownership transfer.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### minter <a id="LiquidityGaugeWrapper-minter"></a>

  Return key of minter.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### crv_token <a id="LiquidityGaugeWrapper-crv-token"></a>

  Return key of crv_token.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### lp_token <a id="LiquidityGaugeWrapper-lp-token"></a>

  Return key of lp_token.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### gauge <a id="LiquidityGaugeWrapper-gauge"></a>

  Return key of gauge.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### balance_of <a id="LiquidityGaugeWrapper-balance-of"></a>

  Return balance of given address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### total_supply <a id="LiquidityGaugeWrapper-total-supply"></a>

  Return the total supply.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### name <a id="LiquidityGaugeWrapper-name"></a>

  Returns the name.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `String`.

- ### symbol <a id="LiquidityGaugeWrapper-symbol"></a>

  Returns the symbol.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `String`.

- ### decimals <a id="LiquidityGaugeWrapper-decimals"></a>

  Returns the decimals.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### future_admin <a id="LiquidityGaugeWrapper-future-admin"></a>

  Returns the future admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### admin <a id="LiquidityGaugeWrapper-admin"></a>

  Returns the admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### claimable_crv <a id="LiquidityGaugeWrapper-claimable-crv"></a>

  Returns the claimable_crv.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | account        | Key  |

  This method **returns** `U256`.

- ### approved_to_deposit <a id="LiquidityGaugeWrapper-approved-to-deposit"></a>

  Returns bool that the provided address is allowed to deposit.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | spender        | Key  |

  This method **returns** `bool`.

- ### is_killed <a id="LiquidityGaugeWrapper-is-killed"></a>

  Return the bool.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `bool`.

## Deploying Liquidity Gauge Reward Wrapper contract manually

If you need to deploy the `Liquidity Gauge Reward Wrapper` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - name : ERC20 token's name to be set
  - symbol : ERC20 token's symbol to be set
  - gauge : Gauge controller deployed contract package hash
  - admin : Admin who can kill the gauge
  - contract_name : Contract name for deployment

Following is the command to deploy the `Liquidity Gauge Reward Wrapper contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 250000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:String='Token full name'" \
    --session-arg="symbol:String='Token symbol'" \
    --session-arg="gauge:Key='Liquidity gauge contract address'" \
    --session-arg="admin:Key='Admin who can kill the gauge'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="LiquidityGaugeRewardWrapper-entry-point-methods"></a>

Following are the LiquidityGaugeRewardWrapper's entry point methods.

- ### user_checkpoint <a id="LiquidityGaugeRewardWrapper-user-checkpoint"></a>

  Record a checkpoint for `addr`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** `bool`.

- ### claimable_tokens <a id="LiquidityGaugeRewardWrapper-claimable-tokens"></a>

  Return the number of claimable tokens per user.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** `U256`.

- ### claimable_reward <a id="LiquidityGaugeRewardWrapper-claimable-reward"></a>

  Return the number of claimable reward per user.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** `U256`.

- ### claim_tokens <a id="LiquidityGaugeRewardWrapper-claim-tokens"></a>

  Claim mintable CRV and reward tokens.

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | addr           | Option`<Key>` |

  This method **returns** nothing.

- ### set_approve_deposit <a id="LiquidityGaugeRewardWrapper-set-approve-deposit"></a>

  Set whether `addr` can deposit tokens for `msg.sender`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |
  | can_deposit    | bool |

  This method **returns** nothing.

- ### deposit <a id="LiquidityGaugeRewardWrapper-deposit"></a>

  Deposit `value` LP tokens.

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | value          | U256          |
  | addr           | Option`<Key>` |

  This method **returns** nothing.

- ### withdraw <a id="LiquidityGaugeRewardWrapper-withdraw"></a>

  Withdraw `value` LP tokens.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | value          | U256 |

  This method **returns** nothing.

- ### allowance <a id="LiquidityGaugeRewardWrapper-allowance"></a>

  Returns an U256 specifying the amount of tokens still available for the spender..

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | spender        | Key  |

  This method **returns** `U256`.

- ### transfer <a id="LiquidityGaugeRewardWrapper-transfer"></a>

  Returns Result<(), u32>. Transfer token for a specified address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- ### transfer_from <a id="LiquidityGaugeRewardWrapper-transfer-from"></a>

  Returns Result<(), u32>. Transfer tokens from one address to another.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | recipient      | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- ### approve <a id="LiquidityGaugeRewardWrapper-approve"></a>

  Approve the passed address to transfer the specified amount of
  tokens on behalf of msg.sender.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** nothing.

- ### increase_allowance <a id="LiquidityGaugeRewardWrapper-increase-allowance"></a>

  Increase the allowance granted to `spender` by the caller.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** `Result<(),u32>`.

- ### decrease_allowance <a id="LiquidityGaugeRewardWrapper-decrease-allowance"></a>

  Decrease the allowance granted to `spender` by the caller.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** `Result<(),u32>`.

- ### kill_me <a id="LiquidityGaugeRewardWrapper-kill-me"></a>

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### commit_transfer_ownership <a id="LiquidityGaugeRewardWrapper-commit-transfer-ownership"></a>

  Transfer ownership of GaugeController to `addr`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** nothing.

- ### apply_transfer_ownership <a id="LiquidityGaugeRewardWrapper-apply-transfer-ownership"></a>

  Apply a pending ownership transfer.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### minter <a id="LiquidityGaugeRewardWrapper-minter"></a>

  Return key of minter.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### crv_token <a id="LiquidityGaugeRewardWrapper-crv-token"></a>

  Return key of crv_token.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### lp_token <a id="LiquidityGaugeRewardWrapper-lp-token"></a>

  Return key of lp_token.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### rewarded_token <a id="LiquidityGaugeRewardWrapper-rewarded-token"></a>

  Return key of rewarded_token.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### gauge <a id="LiquidityGaugeRewardWrapper-gauge"></a>

  Return key of gauge.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### balance_of <a id="LiquidityGaugeRewardWrapper-balance-of"></a>

  Return balance of given address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### total_supply <a id="LiquidityGaugeRewardWrapper-total-supply"></a>

  Return the total supply.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### name <a id="LiquidityGaugeRewardWrapper-name"></a>

  Returns the name.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `String`.

- ### symbol <a id="LiquidityGaugeRewardWrapper-symbol"></a>

  Returns the symbol.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `String`.

- ### decimals <a id="LiquidityGaugeRewardWrapper-decimals"></a>

  Returns the decimals.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### future_admin <a id="LiquidityGaugeRewardWrapper-future-admin"></a>

  Returns the future admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### admin <a id="LiquidityGaugeRewardWrapper-admin"></a>

  Returns the admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### claimable_crv <a id="LiquidityGaugeRewardWrapper-claimable-crv"></a>

  Returns the claimable_crv.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | account        | Key  |

  This method **returns** `U256`.

- ### approved_to_deposit <a id="LiquidityGaugeRewardWrapper-approved-to-deposit"></a>

  Returns bool that the provided address is allowed to deposit.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | spender        | Key  |

  This method **returns** `bool`.

- ### is_killed <a id="LiquidityGaugeRewardWrapper-is-killed"></a>

  Return the bool.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `bool`.

- ### reward_integral_for <a id="LiquidityGaugeRewardWrapper-reward-integral-for"></a>

  Returns the reward integral for of provided address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | account        | Key  |

  This method **returns** `U256`.

- ### reward_integral <a id="LiquidityGaugeRewardWrapper-reward-integral"></a>

  Returns the reward integral.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### claimable_rewards <a id="LiquidityGaugeRewardWrapper-claimable-rewards"></a>

  Returns the claimable_rewards.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | account        | Key  |

  This method **returns** `U256`.

## Deploying Liquidity Gauge Reward contract manually

If you need to deploy the `Liquidity Gauge Reward` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - lp_addr : Liquidity pool deployed contract package hash
  - minter : Minter deployed contract package hash
  - reward_contract : Synthetix reward deployed contract package hash
  - rewarded_token : Received synthetix token contract package address
  - admin : Admin who can kill the gauge
  - contract_name : Contract name for deployment

Following is the command to deploy the `Liquidity Gauge Reward contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 260000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="lp_addr:Key='Liquidity Pool contract address'" \
    --session-arg="minter:Key='Minter contract address'" \
    --session-arg="reward_contract:Key='Synthetix reward contract address'" \
    --session-arg="rewarded_token:Key='Received synthetix token contract address'" \
    --session-arg="admin:Key='Admin who can kill the gauge'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="LiquidityGaugeReward-entry-point-methods"></a>

Following are the LiquidityGaugeReward's entry point methods.

- ### user_checkpoint <a id="LiquidityGaugeReward-user-checkpoint"></a>

  Record a checkpoint for `addr`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** `bool`.

- ### claimable_tokens <a id="LiquidityGaugeReward-claimable-tokens"></a>

  Return the number of claimable tokens per user.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** `U256`.

- ### claimable_reward <a id="LiquidityGaugeReward-claimable-reward"></a>

  Return the number of claimable reward per user.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** `U256`.

- ### kick <a id="LiquidityGaugeReward-kick"></a>

  Kick `addr` for abusing their boost. Only if either they had another voting event, or their voting escrow lock expired.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** nothing.

- ### set_approve_deposit <a id="LiquidityGaugeReward-set-approve-deposit"></a>

  Set whether `addr` can deposit tokens for `msg.sender`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |
  | can_deposit    | bool |

  This method **returns** nothing.

- ### deposit <a id="LiquidityGaugeReward-deposit"></a>

  Deposit `value` LP tokens.

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | value          | U256          |
  | addr           | Option`<Key>` |

  This method **returns** nothing.

- ### withdraw <a id="LiquidityGaugeReward-withdraw"></a>

  Withdraw `value` LP tokens.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | value          | U256 |

  This method **returns** nothing.

- ### claim_rewards <a id="LiquidityGaugeReward-claim-rewards"></a>

  Claim reward tokens.

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | addr           | Option`<Key>` |

  This method **returns** nothing.

- ### integrate_checkpoint <a id="LiquidityGaugeReward-integrate-checkpoint"></a>

  Return U256.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### kill_me <a id="LiquidityGaugeReward-kill-me"></a>

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### commit_transfer_ownership <a id="LiquidityGaugeReward-commit-transfer-ownership"></a>

  Transfer ownership of GaugeController to `addr`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** nothing.

- ### apply_transfer_ownership <a id="LiquidityGaugeReward-apply-transfer-ownership"></a>

  Apply a pending ownership transfer.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### toggle_external_rewards_claim <a id="LiquidityGaugeReward-toggle-external-rewards-claim"></a>

  Switch claiming rewards on/off.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | val            | bool |

  This method **returns** nothing.

- ### minter <a id="LiquidityGaugeReward-minter"></a>

  Return key of minter.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### crv_token <a id="LiquidityGaugeReward-crv-token"></a>

  Return key of crv_token.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### lp_token <a id="LiquidityGaugeReward-lp-token"></a>

  Return key of lp_token.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### controller <a id="LiquidityGaugeReward-controller"></a>

  Return key of controller.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### voting_escrow <a id="LiquidityGaugeReward-voting-escrow"></a>

  Return key of voting_escrow.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### balance_of <a id="LiquidityGaugeReward-balance-of"></a>

  Return balance of given address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### total_supply <a id="LiquidityGaugeReward-total-supply"></a>

  Return the total supply.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### future_epoch_time <a id="LiquidityGaugeReward-future-epoch-time"></a>

  Return the future_epoch_time.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### approved_to_deposit <a id="LiquidityGaugeReward-approved-to-deposit"></a>

  Returns bool that the provided address is allowed to deposit.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | spender        | Key  |

  This method **returns** `bool`.

- ### working_balances <a id="LiquidityGaugeReward-working-balances"></a>

  Return working_balances of given address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### working_supply <a id="LiquidityGaugeReward-working-supply"></a>

  Return the working supply.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### period <a id="LiquidityGaugeReward-period"></a>

  Return the period.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U128`.

- ### period_timestamp <a id="LiquidityGaugeReward-period-timestamp"></a>

  Return the period_timestamp.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | U256 |

  This method **returns** `U256`.

- ### integrate_inv_supply <a id="LiquidityGaugeReward-integrate-inv-supply"></a>

  Return the integrate_inv_supply.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | U256 |

  This method **returns** `U256`.

- ### integrate_inv_supply_of <a id="LiquidityGaugeReward-integrate-inv-supply-of"></a>

  Return the integrate_inv_supply_of.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### integrate_checkpoint_of <a id="LiquidityGaugeReward-integrate-checkpoint-of"></a>

  Return the integrate_checkpoint_of.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### integrate_fraction <a id="LiquidityGaugeReward-integrate-fraction"></a>

  Return the integrate_fraction.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### inflation_rate <a id="LiquidityGaugeReward-inflation-rate"></a>

  Return the inflation_rate.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### reward_contract <a id="LiquidityGaugeReward-reward-contract"></a>

  Return key of reward_contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### rewarded_token <a id="LiquidityGaugeReward-rewarded-token"></a>

  Return key of rewarded_token.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### reward_integral <a id="LiquidityGaugeReward-reward-integral"></a>

  Returns the reward integral.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### reward_integral_for <a id="LiquidityGaugeReward-reward-integral-for"></a>

  Returns the reward integral for of provided address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### rewards_for <a id="LiquidityGaugeReward-rewards-for"></a>

  Returns the rewards for of provided address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### claimed_rewards_for <a id="LiquidityGaugeReward-claimed-rewards-for"></a>

  Returns the claimed_rewards_for of provided address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### admin <a id="LiquidityGaugeReward-admin"></a>

  Returns the admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### future_admin <a id="LiquidityGaugeReward-future-admin"></a>

  Returns the future admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### is_killed <a id="LiquidityGaugeReward-is-killed"></a>

  Return the bool.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `bool`.

- ### is_claiming_rewards <a id="LiquidityGaugeReward-is-claiming-rewards"></a>

  Returns the bool.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | account        | Key  |

  This method **returns** `bool`.

## Deploying Liquidity Gauge V3 contract manually

If you need to deploy the `Liquidity Gauge V3` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - lp_addr : Liquidity pool deployed contract package hash
  - minter : Minter deployed contract package hash
  - admin : Admin who can kill the gauge
  - contract_name : Contract name for deployment

Following is the command to deploy the `Liquidity Gauge V3 contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 320000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="lp_addr:Key='Liquidity Pool contract address'" \
    --session-arg="minter:Key='Minter contract address'" \
    --session-arg="admin:Key='Admin who can kill the gauge'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="LiquidityGaugeV3-entry-point-methods"></a>

Following are the LiquidityGaugeV3's entry point methods.

- ### decimals <a id="LiquidityGaugeV3-decimals"></a>

  Returns the decimals.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `u8`.

- ### integrate_checkpoint <a id="LiquidityGaugeV3-integrate-checkpoint"></a>

  Return the U256.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### user_checkpoint <a id="LiquidityGaugeV3-user-checkpoint"></a>

  Record a checkpoint for `addr`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** `bool`.

- ### claimable_tokens <a id="LiquidityGaugeV3-claimable-tokens"></a>

  Return the number of claimable tokens per user.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** `U256`.

- ### reward_contract <a id="LiquidityGaugeV3-reward-contract"></a>

  Returns `ZERO_ADDRESS` if there is no reward contract active. Address of the reward contract providing non-CRV incentives for this gauge.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### last_claim <a id="LiquidityGaugeV3-last-claim"></a>

  Rewards are claimed at most once per hour in order to reduce gas costs.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### claimed_reward <a id="LiquidityGaugeV3-claimed-reward"></a>

  Get the number of already-claimed reward tokens for a user. Returns total amount of `token` already claimed by `addr`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |
  | token          | Key  |

  This method **returns** `U256`.

- ### claimable_reward <a id="LiquidityGaugeV3-claimable-reward"></a>

  Get the number of claimable reward tokens for a user. This call does not consider pending claimable amount in `reward_contract`.Off-chain callers should instead use `claimable_rewards_write` as a
  view method. Returns U256 Claimable reward token amount.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |
  | token          | Key  |

  This method **returns** `U256`.

- ### claimable_reward_write <a id="LiquidityGaugeV3-claimable-reward-write"></a>

  Get the number of claimable reward tokens for a user. Returns U256 Claimable reward token amount.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |
  | token          | Key  |

  This method **returns** `U256`.

- ### set_rewards_receiver <a id="LiquidityGaugeV3-set-rewards-receiver"></a>

  Set the default reward receiver for the caller.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | receiver       | Key  |

  This method **returns** nothing.

- ### claim_rewards <a id="LiquidityGaugeV3-claim-rewards"></a>

  Claim available reward tokens for `addr`.

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | addr           | Option`<Key>` |
  | receiver       | Option`<Key>` |

  This method **returns** nothing.

- ### kick <a id="LiquidityGaugeV3-kick"></a>

  Kick `addr` for abusing their boost. Only if either they had another voting event, or their voting escrow lock expired.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** nothing.

- ### deposit <a id="LiquidityGaugeV3-deposit"></a>

  Deposit `value` LP tokens.

  Following is the table of parameters.

  | Parameter Name | Type           |
  | -------------- | -------------- |
  | value          | U256           |
  | addr           | Option`<Key>`  |
  | claim_rewards  | Option`<bool>` |

  This method **returns** nothing.

- ### withdraw <a id="LiquidityGaugeV3-withdraw"></a>

  Withdraw `value` LP tokens.

  Following is the table of parameters.

  | Parameter Name | Type           |
  | -------------- | -------------- |
  | value          | U256           |
  | claim_rewards  | Option`<bool>` |

  This method **returns** nothing.

- ### transfer <a id="LiquidityGaugeV3-transfer"></a>

  Returns Result<(), u32>. Transfer token for a specified address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- ### transfer_from <a id="LiquidityGaugeV3-transfer-from"></a>

  Returns Result<(), u32>. Transfer tokens from one address to another.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | recipient      | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- ### approve <a id="LiquidityGaugeV3-approve"></a>

  Approve the passed address to transfer the specified amount of
  tokens on behalf of msg.sender

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** nothing.

- ### increase_allowance <a id="LiquidityGaugeV3-increase-allowance"></a>

  Increase the allowance granted to `spender` by the caller.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** `Result<(),u32>`.

- ### decrease_allowance <a id="LiquidityGaugeV3-decrease-allowance"></a>

  Decrease the allowance granted to `spender` by the caller.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** `Result<(),u32>`.

- ### set_rewards <a id="LiquidityGaugeV3-set-rewards"></a>

  Set the active reward contract. A reward contract cannot be set while this contract has no deposits.

  Following is the table of parameters.

  | Parameter Name  | Type          |
  | --------------- | ------------- |
  | reward_contract | Key           |
  | claim_sig       | String        |
  | reward_tokens   | Vec`<String>` |

  This method **returns** nothing.

- ### set_killed <a id="LiquidityGaugeV3-set-killed"></a>

  Set the killed status for this contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | is_killed      | bool |

  This method **returns** nothing.

- ### commit_transfer_ownership <a id="LiquidityGaugeV3-commit-transfer-ownership"></a>

  Transfer ownership of GaugeController to `addr`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** nothing.

- ### accept_transfer_ownership <a id="LiquidityGaugeV3-accept-transfer-ownership"></a>

  Accept a pending ownership transfer.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### minter <a id="LiquidityGaugeV3-minter"></a>

  Return key of minter.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### crv_token <a id="LiquidityGaugeV3-crv-token"></a>

  Return key of crv_token.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### lp_token <a id="LiquidityGaugeV3-lp-token"></a>

  Return key of lp_token.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### controller <a id="LiquidityGaugeV3-controller"></a>

  Return key of controller.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### voting_escrow <a id="LiquidityGaugeV3-voting-escrow"></a>

  Return key of voting_escrow.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### future_epoch_time <a id="LiquidityGaugeV3-future-epoch-time"></a>

  Return the future_epoch_time.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### balance_of <a id="LiquidityGaugeV3-balance-of"></a>

  Return balance of given address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### total_supply <a id="LiquidityGaugeV3-total-supply"></a>

  Return the total supply.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### allowance <a id="LiquidityGaugeV3-allowance"></a>

  Returns the allowance of provided key.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | spender        | Key  |

  This method **returns** `U256`.

- ### name <a id="LiquidityGaugeV3-name"></a>

  Returns the name.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `String`.

- ### symbol <a id="LiquidityGaugeV3-symbol"></a>

  Returns the symbol.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `String`.

- ### working_balances <a id="LiquidityGaugeV3-working-balances"></a>

  Return working_balances of given address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### working_supply <a id="LiquidityGaugeV3-working-supply"></a>

  Return the working supply.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### period <a id="LiquidityGaugeV3-period"></a>

  Return the period.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U128`.

- ### period_timestamp <a id="LiquidityGaugeV3-period-timestamp"></a>

  Return the period_timestamp.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | U256 |

  This method **returns** `U256`.

- ### integrate_inv_supply <a id="LiquidityGaugeV3-integrate-inv-supply"></a>

  Return the integrate_inv_supply.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | U256 |

  This method **returns** `U256`.

- ### integrate_inv_supply_of <a id="LiquidityGaugeV3-integrate-inv-supply-of"></a>

  Return the integrate_inv_supply_of.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### integrate_checkpoint_of <a id="LiquidityGaugeV3-integrate-checkpoint-of"></a>

  Return the integrate_checkpoint_of.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### integrate_fraction <a id="LiquidityGaugeV3-integrate-fraction"></a>

  Return the integrate_fraction.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### inflation_rate <a id="LiquidityGaugeV3-inflation-rate"></a>

  Return the inflation_rate.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### reward_tokens <a id="LiquidityGaugeV3-reward-tokens"></a>

  Return key of reward_tokens.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | U256 |

  This method **returns** `Key`.

- ### rewards_receiver <a id="LiquidityGaugeV3-rewards-receiver"></a>

  Returns the reward receiver.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `Key`.

- ### reward_integral <a id="LiquidityGaugeV3-reward-integral"></a>

  Returns the reward integral.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### reward_integral_for <a id="LiquidityGaugeV3-reward-integral-for"></a>

  Returns the reward integral for of provided address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | spender        | Key  |

  This method **returns** `U256`.

- ### admin <a id="LiquidityGaugeV3-admin"></a>

  Returns the admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### future_admin <a id="LiquidityGaugeV3-future-admin"></a>

  Returns the future admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### is_killed <a id="LiquidityGaugeV3-is-killed"></a>

  Return the bool.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `bool`.

## Deploying Curve Token V3 contract manually

If you need to deploy the `Curve Token V3` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - name : Token name to be setup
  - symbol : Token Symbol to be setup
  - contract_name : Contract name for deployment

Following is the command to deploy the `Curve Token V3 contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 160000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:String='name of contract'" \
    --session-arg="symbol:String='symbol of contract'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="CurveTokenV3-entry-point-methods"></a>

Following are the CurveTokenV3's entry point methods.

- ### decimals <a id="CurveTokenV3-decimals"></a>

  Returns the decimals.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### transfer <a id="CurveTokenV3-transfer"></a>

  Returns Result<(), u32>. Transfer token for a specified address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- ### transfer_from <a id="CurveTokenV3-transfer-from"></a>

  Returns Result<(), u32>. Transfer tokens from one address to another.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | recipient      | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- ### approve <a id="CurveTokenV3-approve"></a>

  Approve the passed address to transfer the specified amount of
  tokens on behalf of msg.sender

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** nothing.

- ### increase_allowance <a id="CurveTokenV3-increase-allowance"></a>

  Increase the allowance granted to `spender` by the caller.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** `Result<(),u32>`.

- ### decrease_allowance <a id="CurveTokenV3-decrease-allowance"></a>

  Decrease the allowance granted to `spender` by the caller.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** `Result<(),u32>`.

- ### mint <a id="CurveTokenV3-mint"></a>

  Mint an amount of the token and assigns it to an account. This encapsulates the modification of balances such that the proper events are emitted.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | to             | Key  |
  | amount         | U256 |

  This method **returns** `bool`.

- ### burn_from <a id="CurveTokenV3-burn-from"></a>

  Burn an amount of the token from a given account.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | from           | Key  |
  | amount         | U256 |

  This method **returns** `bool`.

- ### set_minter <a id="CurveTokenV3-set-minter"></a>

  Set the minter.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | minter         | Key  |

  This method **returns** nothing.

- ### set_name <a id="CurveTokenV3-set-name"></a>

  Set the name of contract.

  Following is the table of parameters.

  | Parameter Name | Type   |
  | -------------- | ------ |
  | name           | String |
  | symbol         | String |

  This method **returns** nothing.

- ### name <a id="CurveTokenV3-name"></a>

  Returns the name.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `String`.

- ### symbol <a id="CurveTokenV3-symbol"></a>

  Returns the symbol.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `String`.

- ### total_supply <a id="CurveTokenV3-total-supply"></a>

  Return the total supply.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### minter <a id="CurveTokenV3-minter"></a>

  Return the key of minter.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### balance_of <a id="CurveTokenV3-balance-of"></a>

  Return balance of given address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### allowance <a id="CurveTokenV3-allowance"></a>

  Returns the allowance of provided key.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | spender        | Key  |

  This method **returns** `U256`.

## Deploying ERC20 CRV contract manually

If you need to deploy the `ERC20 CRV` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - name : Token name to be setup
  - symbol : Token Symbol to be setup
  - contract_name : Contract name for deployment

Following is the command to deploy the `ERC20 CRV contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 200000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:String='name of contract'" \
    --session-arg="symbol:String='symbol of contract'" \
    --session-arg="decimals:u8='Number of decimals for token'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="ERC20CRV-entry-point-methods"></a>

Following are the ERC20CRV's entry point methods.

- ### update_mining_parameters <a id="ERC20CRV-update-mining-parameters"></a>

  Update mining rate and supply at the start of the epoch.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### start_epoch_time_write <a id="ERC20CRV-start-epoch-time-write"></a>

  Get timestamp of the current mining epoch start while simultaneously updating mining parameters. Returns Timestamp of the epoch.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### future_epoch_time_write <a id="ERC20CRV-future-epoch-time-write"></a>

  Get timestamp of the next mining epoch start while simultaneously updating mining parameters. Returns Timestamp of the next epoch.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### available_supply <a id="ERC20CRV-available-supply"></a>

  Current number of tokens in existence (claimed or unclaimed).

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### mintable_in_timeframe <a id="ERC20CRV-mintable-in-timeframe"></a>

  How much supply is mintable from start timestamp till end timestamp. Returns Tokens mintable from `start` till `end`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | start          | U256 |
  | end            | U256 |

  This method **returns** `U256`.

- ### set_minter <a id="ERC20CRV-set-minter"></a>

  Set the minter.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | minter         | Key  |

  This method **returns** nothing.

- ### set_admin <a id="ERC20CRV-set-admin"></a>

  Set the new admin.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | admin          | Key  |

  This method **returns** nothing.

- ### total_supply <a id="ERC20CRV-total-supply"></a>

  Total number of tokens in existence.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### allowance <a id="ERC20CRV-allowance"></a>

  Check the amount of tokens that an owner allowed to a spender.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | spender        | Key  |

  This method **returns** `U256`.

- ### transfer <a id="ERC20CRV-transfer"></a>

  Returns `Result<(), u32>`. Transfer token for a specified address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- ### transfer_from <a id="ERC20CRV-transfer-from"></a>

  Returns Result<(), u32>. Transfer tokens from one address to another.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | recipient      | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- ### approve <a id="ERC20CRV-approve"></a>

  Approve the passed address to transfer the specified amount of
  tokens on behalf of msg.sender

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** nothing.

- ### mint <a id="ERC20CRV-mint"></a>

  Mint `value` tokens and assign them to `to`

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | to             | Key  |
  | value          | U256 |

  This method **returns** `bool`.

- ### burn <a id="ERC20CRV-burn"></a>

  Burn an amount of the token from a given account.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | value          | U256 |

  This method **returns** nothing.

- ### set_name <a id="ERC20CRV-set-name"></a>

  Change the token name and symbol to `name` and `symbol`.

  Following is the table of parameters.

  | Parameter Name | Type   |
  | -------------- | ------ |
  | name           | String |
  | symbol         | String |

  This method **returns** nothing.

- ### name <a id="ERC20CRV-name"></a>

  Returns the name.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `String`.

- ### symbol <a id="ERC20CRV-symbol"></a>

  Returns the symbol.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `String`.

- ### decimals <a id="ERC20CRV-decimals"></a>

  Returns the decimals.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### balance_of <a id="ERC20CRV-balance-of"></a>

  Return balance of given address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### minter <a id="ERC20CRV-minter"></a>

  Return the key of minter.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### admin <a id="ERC20CRV-admin"></a>

  Return the key of admin.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### mining_epoch <a id="ERC20CRV-mining-epoch"></a>

  Return the mining epoch.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U128`.

- ### start_epoch_time <a id="ERC20CRV-start-epoch-time"></a>

  Return the start_epoch_time.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### rate <a id="ERC20CRV-rate"></a>

  Return the rate.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### increase_allowance <a id="ERC20CRV-increase-allowance"></a>

  Increase the amount of tokens that an owner allowed to a spender.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- ### decrease_allowance <a id="ERC20CRV-decrease-allowance"></a>

  Decrease the amount of tokens that an owner allowed to a spender..

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

## Deploying ERC20 contract manually

If you need to deploy the `ERC20 contract` manually you need to pass the following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - name : Token name to be setup
  - symbol : Token Symbol to be setup
  - decimals : Decimals unit value for the token
  - initial_supply : Starting supply for the token
  - contract_name : Contract name for deployment

Following is the command to deploy the `ERC20 contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 160000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:string='token-name'" \
    --session-arg="symbol:string='token-symbol'" \
    --session-arg="decimals:u8='unsigned integer value'" \
    --session-arg="initial_supply:U256='unsigned integer value'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="erc20-entry-point-methods"></a>

Following are the ERC20's entry point methods.

- ### transfer <a id="erc20-transfer"></a>

  Lets `self.get_caller()` send pool tokens to a recipient hash.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- ### transfer_from <a id="erc20-transfer-from"></a>

  Sends pool tokens from one hash to another.
  <br>User needs to call approve method before calling the `tranfer_from`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | recipient      | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

  <br>**Recommendation:**
  The exploit is mitigated through use of functions that increase/decrease the allowance relative to its current value, such as `increaseAllowance()` and `decreaseAllowance()`,
  Pending community agreement on an ERC standard that would protect against this exploit, we recommend that developers of applications dependent on approve() / transferFrom()
  should keep in mind that they have to set allowance to 0 first and verify if it was used before setting the new value.
  <br>**Note:** Teams who decide to wait for such a standard should make these
  recommendations to app developers who work with their token contract.

- ### approve <a id="erc20-approve"></a>

  Lets `self.get_caller()` set their allowance for a spender.
  <br>user needs to call this `approve` method before calling the `transfer_from` method.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** nothing.

  <br>**Recommendation:**
  The exploit is mitigated through use of functions that increase/decrease the allowance relative to its current value, such as `increaseAllowance()` and `decreaseAllowance()`,
  Pending community agreement on an ERC standard that would protect against this exploit, we recommend that developers of applications dependent on approve() / transferFrom()
  should keep in mind that they have to set allowance to 0 first and verify if it was used before setting the new value.
  <br>**Note:** Teams who decide to wait for such a standard should make these
  recommendations to app developers who work with their token contract.

- ### balance_of <a id="erc20-balance-of"></a>

  This method will return the balance of owner in `ERC20 Contract`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### nonce <a id="erc20-nonce"></a>

  Returns the current `nonce` for an address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### allowance <a id="erc20-allowance"></a>

  Returns the amount of liquidity tokens owned by an hash that a spender is allowed to transfer via `transfer_from`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | spender        | Key  |

  This method **returns** `U256`.

- ### total_supply <a id="erc20-total-supply"></a>

  Returns the total amount of pool tokens for a pair.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### mint <a id="erc20-mint"></a>

  This method mints the number of tokens provided by user against the hash provided by user.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | to             | Key  |
  | amount         | U256 |

  This method **returns** nothing.

- ### burn <a id="erc20-burn"></a>

  This method burns the number of tokens provided by user against the hash provided by user.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | from           | Key  |
  | amount         | U256 |

  This method **returns** nothing.

  <br>**Note:** To `burn` the tokens against the hash provided by user, User needs to `mint` tokens first in `ERC20`.

- ### name <a id="erc20-name"></a>

  Returns the `name` of tokens for a pair.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `String`.

- ### symbol <a id="erc20-symbol"></a>

  Returns the `symbol` of tokens for a pair.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `String`.

- ### decimals <a id="erc20-decimals"></a>

  Returns the `decimals`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `u8`.

- ### increase_allowance <a id="erc20-increase-allowance"></a>

  Increase the amount of tokens that an owner allowed to a spender.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- ### decrease_allowance <a id="erc20-decrease-allowance"></a>

  Decrease the amount of tokens that an owner allowed to a spender..

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- ### package_hash <a id="erc20-package-hash"></a>

  Return the package hash of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `ContractPackageHash`.

## Deploying Fee Distributor contract manually

If you need to deploy the `Fee Distributor` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - voting_escrow : Voting escrow deployed contract package hash
  - start_time : Epoch time for fee distribution to start
  - token : Fee token deployed contract package hash (3CRV)
  - admin : Admin account hash
  - emergency_return : Specify address to transfer `_token` balance to if this contract if killed
  - contract_name : Contract name for deployment

Following is the command to deploy the `Fee Distributor contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 220000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="voting_escrow:Key='VotingEscrow contract address'" \
    --session-arg="start_time:U256='Epoch time for fee distribution to start'" \
    --session-arg="token:Key='Fee token address (3CRV)'" \
    --session-arg="admin:Key='Admin address'" \
    --session-arg="emergency_return:Key='Address to transfer `_token` balance to if this contract is killed'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="FeeDistributor-entry-point-methods"></a>

Following are the FeeDistributor's entry point methods.

- ### checkpoint_token <a id="FeeDistributor-checkpoint-token"></a>

  Update the token checkpoint. Calculates the total number of tokens to be distributed in a given week. During setup for the initial distribution this function is only callable by the contract owner. Beyond initial distro, it can be enabled for anyone to call.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### ve_for_at <a id="FeeDistributor-ve-for-at"></a>

  Get the veCRV balance for `user` at `timestamp`. Return veCRV balance.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | user           | U256 |
  | timestamp      | U256 |

  This method **returns** `U256`.

- ### checkpoint_total_supply <a id="FeeDistributor-checkpoint-total-supply"></a>

  Update the veCRV total supply checkpoint. The checkpoint is also updated by the first claimant each new epoch week. This function may be called independently of a claim, to reduce claiming gas costs.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### claim <a id="FeeDistributor-claim"></a>

  Claim fees for `addr`. Each call to claim look at a maximum of 50 user veCRV points. For accounts with many veCRV related actions, this function may need to be called more than once to claim all available fees. In the `Claimed` event that fires, if `claim_epoch` is less than `max_epoch`, the account may claim again.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** `U256`.

- ### claim_many <a id="FeeDistributor-claim-many"></a>

  Make multiple fee claims in a single call. Used to claim for many accounts at once, or to make multiple claims for the same address when that address has significant veCRV history.

  Following is the table of parameters.

  | Parameter Name | Type       |
  | -------------- | ---------- |
  | receivers      | Vec`<Key>` |

  This method **returns** `bool`.

- ### burn <a id="FeeDistributor-burn"></a>

  Receive CRV into the contract and trigger a token checkpoint.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | coin           | Key  |

  This method **returns** `bool`.

- ### commit_admin <a id="FeeDistributor-commit-admin"></a>

  Commit transfer of ownership.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** nothing.

- ### apply_admin <a id="FeeDistributor-apply-admin"></a>

  Apply transfer of ownership.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### toggle_allow_checkpoint_token <a id="FeeDistributor-toggle-allow-checkpoint-token"></a>

  Toggle permission for checkpointing by any account.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### kill_me <a id="FeeDistributor-kill-me"></a>

  Kill the contract. Killing transfers the entire 3CRV balance to the emergency return address and blocks the ability to claim or burn. The contract cannot be unkilled.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### recover_balance <a id="FeeDistributor-recover-balance"></a>

  Recover ERC20 tokens from this contract. Tokens are sent to the emergency return address. Return bool success.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | coin           | Key  |

  This method **returns** `bool`.

- ### start_time <a id="FeeDistributor-start-time"></a>

  Return Start time.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### time_cursor <a id="FeeDistributor-time-cursor"></a>

  Return time cursor.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### time_cursor_of <a id="FeeDistributor-time-cursor-of"></a>

  Return time cursor of.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** `U256`.

- ### user_epoch_of <a id="FeeDistributor-user-epoch-of"></a>

  Return user_epoch_of.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** `U256`.

- ### last_token_time <a id="FeeDistributor-last-token-time"></a>

  Return last_token_time.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### tokens_per_week <a id="FeeDistributor-tokens-per-week"></a>

  Return tokens_per_week.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | week           | U256 |

  This method **returns** `U256`.

- ### voting_escrow <a id="FeeDistributor-voting-escrow"></a>

  Return voting_escrow key.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### token <a id="FeeDistributor-token"></a>

  Return token key.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### total_received <a id="FeeDistributor-total-received"></a>

  Return total_received.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### token_last_balance <a id="FeeDistributor-token-last-balance"></a>

  Return token_last_balance.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### ve_supply <a id="FeeDistributor-ve-supply"></a>

  Return ve_supply.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | week           | U256 |

  This method **returns** `U256`.

- ### admin <a id="FeeDistributor-admin"></a>

  Return admin key.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### future_admin <a id="FeeDistributor-future-admin"></a>

  Return future_admin key.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### can_checkpoint_token <a id="FeeDistributor-can-checkpoint-token"></a>

  Return can_checkpoint_token bool.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `bool`.

- ### emergency_return <a id="FeeDistributor-emergency-return"></a>

  Return emergency_return key.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### is_killed <a id="FeeDistributor-is-killed"></a>

  Return is_killed bool.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `bool`.

## Deploying Gauge Proxy contract manually

If you need to deploy the `Gauge Proxy` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - ownership_admin : Address specified for the contract ownership admin
  - emergency_admin : Address of emergency admin in case of any issue with the ownership admin
  - contract_name : Contract name for deployment

Following is the command to deploy the `Gauge Proxy contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 130000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="ownership_admin:Key='ownership_admin address'" \
    --session-arg="emergency_admin:Key='emergency_admin address'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="GaugeProxy-entry-point-methods"></a>

Following are the GaugeProxy's entry point methods.

- ### commit_set_admins <a id="GaugeProxy-commit-set-admins"></a>

  Set ownership admin to `o_admin` and emergency admin to `e_admin`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | o_admin        | Key  |
  | e_admin        | Key  |

  This method **returns** nothing.

- ### accept_set_admins <a id="GaugeProxy-accept-set-admins"></a>

  Apply the effects of `commit_set_admins`. Only callable by the new owner admin.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### commit_transfer_ownership <a id="GaugeProxy-commit-transfer-ownership"></a>

  Transfer ownership for liquidity gauge `gauge` to `new_owner`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | gauge          | Key  |
  | new_owner      | Key  |

  This method **returns** nothing.

- ### accept_transfer_ownership <a id="GaugeProxy-accept-transfer-ownership"></a>
  Apply transferring ownership of `gauge`.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| gauge          | Key  |

This method **returns** nothing.

- ### set_killed <a id="GaugeProxy-set-killed"></a>

  Set the killed status for `gauge`. When killed, the gauge always yields a rate of 0 and so cannot mint CRV.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | gauge          | Key  |
  | is_killed      | bool |

  This method **returns** nothing.

- ### set_rewards <a id="GaugeProxy-set-rewards"></a>

  Set the active reward contract for `_gauge`.

  Following is the table of parameters.

  | Parameter Name  | Type       |
  | --------------- | ---------- |
  | gauge           | Key        |
  | reward_contract | Key        |
  | sigs            | String     |
  | reward_tokens   | Vec`<Key>` |

  This method **returns** nothing.

- ### ownership_admin <a id="GaugeProxy-ownership-admin"></a>

  Return ownership_admin.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### emergency_admin <a id="GaugeProxy-emergency-admin"></a>

  Return emergency_admin.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### future_ownership_admin <a id="GaugeProxy-future-ownership-admin"></a>

  Return future_ownership_admin.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### future_emergency_admin <a id="GaugeProxy-future-emergency-admin"></a>

  Return future_emergency_admin.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

## Deploying I Reward Distribution Recipient contract manually

If you need to deploy the `I Reward Distribution Recipient` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - contract_name : Contract name for deployment

Following is the command to deploy the `I Reward Distribution Recipient contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 110000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="IRewardDistributionRecipient-entry-point-methods"></a>

Following are the IRewardDistributionRecipient's entry point methods.

- ### set_reward_distribution <a id="IRewardDistributionRecipient-set-reward-distribution"></a>

  Set reward Distribution.

  Following is the table of parameters.

  | Parameter Name      | Type |
  | ------------------- | ---- |
  | reward_distribution | Key  |

  This method **returns** nothing.

- ### owner <a id="IRewardDistributionRecipient-owner"></a>

  Returns owner key.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### is_owner <a id="IRewardDistributionRecipient-is-owner"></a>

  Returns the bool.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `bool`.

- ### renounce_ownership <a id="IRewardDistributionRecipient-renounce-ownership"></a>

  Renouncing ownership will leave the contract without an owner,thereby removing any functionality that is only available to the owner.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### transfer_ownership <a id="IRewardDistributionRecipient-transfer-ownership"></a>

  Transfers ownership of the contract to a new account (`newOwner`).Can only be called by the current owner.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | new_owner      | Key  |

  This method **returns** nothing.

## Deploying Vesting Escrow contract manually

If you need to deploy the `Vesting Escrow` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - token : Contract package hash of the ERC20 token being distributed
  - start_time : Timestamp at which the distribution of token starts
  - end_time : Timestamp until tokens should be vested
  - can_disable : Option for admin to disable accounts in this deployment
  - fund_admins : Temporary admin accounts used only for funding
  - contract_name : Contract name for deployment

Following is the command to deploy the `Vesting Escrow contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 380000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="token:Key='Address of the ERC20 token being distributed'" \
    --session-arg="start_time:U256='Timestamp at which the distribution starts'" \
    --session-arg="end_time:U256='Time until everything should be vested'" \
    --session-arg="can_disable:bool='Whether admin can disable accounts in this deployment'" \
    --session-arg="fund_admins:Vec<String>='Temporary admin accounts used only for funding'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="VestingEscrow-entry-point-methods"></a>

Following are the VestingEscrow's entry point methods.

- ### add_tokens <a id="VestingEscrow-add-tokens"></a>

  Transfer vestable tokens into the contract. Handled separate from `fund` to reduce transaction count when using funding admins.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | amount         | U256 |

  This method **returns** nothing.

- ### fund <a id="VestingEscrow-fund"></a>

  Vest tokens for multiple recipients.

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | recipients     | Vec`<String>` |
  | amounts        | Vec`<U256>`   |

  This method **returns** nothing.

- ### toggle_disable <a id="VestingEscrow-toggle-disable"></a>

  Disable or re-enable a vested address's ability to claim tokens. When disabled, the address is only unable to claim tokens which are still locked at the time of this call. It is not possible to block the claim of tokens which have already vested.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |

  This method **returns** nothing.

- ### disable_can_disable <a id="VestingEscrow-disable-can-disable"></a>

  Disable the ability to call `toggle_disable`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### disable_fund_admins <a id="VestingEscrow-disable-fund-admins"></a>

  Disable the funding admin accounts.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### vested_supply <a id="VestingEscrow-vested-supply"></a>

  Get the total number of tokens which have vested, that are held by this contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### lock_supply <a id="VestingEscrow-lock-supply"></a>

  Get the total number of tokens which are still locked (have not yet vested).

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### vested_of <a id="VestingEscrow-vested-of"></a>

  Get the number of tokens which have vested for a given address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |

  This method **returns** `U256`.

- ### balance_of <a id="VestingEscrow-balance-of"></a>

  Get the number of unclaimed, vested tokens for a given address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |

  This method **returns** `U256`.

- ### locked_of <a id="VestingEscrow-locked-of"></a>

  Get the number of locked tokens for a given address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |

  This method **returns** `U256`.

- ### claim <a id="VestingEscrow-claim"></a>

  Claim tokens which have vested.

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | addr           | Option`<Key>` |

  This method **returns** `U256`.

- ### commit_transfer_ownership <a id="VestingEscrow-commit-transfer-ownership"></a>

  Transfer ownership of GaugeController to `addr`

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** nothing.

- ### apply_transfer_ownership <a id="VestingEscrow-apply-transfer-ownership"></a>

  Apply a pending ownership transfer

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### token <a id="VestingEscrow-token"></a>

  Return the token address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### start_time <a id="VestingEscrow-start-time"></a>

  Return the start time.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### end_time <a id="VestingEscrow-end-time"></a>

  Return the end time.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### initial_locked <a id="VestingEscrow-initial-locked"></a>

  Return the initial locked.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### total_claimed <a id="VestingEscrow-total-claimed"></a>

  Return the total claimed.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### initial_locked_supply <a id="VestingEscrow-initial-locked-supply"></a>

  Return the initial_locked_supply.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### unallocated_supply <a id="VestingEscrow-unallocated-supply"></a>

  Return the unallocated_supply.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### can_disable <a id="VestingEscrow-can-disable"></a>

  Return the can_disable.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `bool`.

- ### disabled_at <a id="VestingEscrow-disabled-at"></a>

  Return the disabled_at.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### admin <a id="VestingEscrow-admin"></a>

  Returns the admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### future_admin <a id="VestingEscrow-future-admin"></a>

  Returns the future admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### fund_admins_enabled <a id="VestingEscrow-fund-admins-enabled"></a>

  Returns the bool.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `bool`.

- ### fund_admins <a id="VestingEscrow-fund-admins"></a>

  Returns the bool.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `bool`.

## Deploying Vesting Escrow Simple contract manually

If you need to deploy the `Vesting Escrow Simple` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - contract_name : Contract name for deployment

Following is the command to deploy the `Vesting Escrow Simple contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 160000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="VestingEscrowSimple-entry-point-methods"></a>

Following are the VestingEscrowSimple's entry point methods.

- ### initialize <a id="VestingEscrowSimple-initialize"></a>

  Initialize the contract. This function is seperate from `__init__` because of the factory pattern used in `VestingEscrowFactory.deploy_vesting_contract`. It may be called once per deployment.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | admin          | Key  |
  | token          | Key  |
  | recipient      | Key  |
  | amount         | U256 |
  | start_time     | U256 |
  | end_time       | U256 |
  | can_disable    | bool |

  This method **returns** `bool`.

- ### toggle_disable <a id="VestingEscrowSimple-toggle-disable"></a>

  Disable or re-enable a vested address's ability to claim tokens. When disabled, the address is only unable to claim tokens which are still locked at the time of this call. It is not possible to block the claim of tokens which have already vested.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |

  This method **returns** nothing.

- ### disable_can_disable <a id="VestingEscrowSimple-disable-can-disable"></a>

  Disable the ability to call `toggle_disable`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### vested_supply <a id="VestingEscrowSimple-vested-supply"></a>

  Get the total number of tokens which have vested, that are held by this contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### lock_supply <a id="VestingEscrowSimple-lock-supply"></a>

  Get the total number of tokens which are still locked (have not yet vested).

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### vested_of <a id="VestingEscrowSimple-vested-of"></a>

  Get the number of tokens which have vested for a given address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |

  This method **returns** `U256`.

- ### balance_of <a id="VestingEscrowSimple-balance-of"></a>

  Get the number of locked tokens for a given address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |

  This method **returns** `U256`.

- ### locked_of <a id="VestingEscrowSimple-locked-of"></a>

  Get the number of locked tokens for a given address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |

  This method **returns** `U256`.

- ### claim <a id="VestingEscrowSimple-claim"></a>

  Claim tokens which have vested.

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | addr           | Option`<Key>` |

  This method **returns** `U256`.

- ### commit_transfer_ownership <a id="VestingEscrowSimple-commit-transfer-ownership"></a>

  Transfer ownership of GaugeController to `addr`

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** nothing.

- ### apply_transfer_ownership <a id="VestingEscrowSimple-apply-transfer-ownership"></a>

  Apply a pending ownership transfer

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### token <a id="VestingEscrowSimple-token"></a>

  Return the token address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### start_time <a id="VestingEscrowSimple-start-time"></a>

  Return the start time.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### end_time <a id="VestingEscrowSimple-end-time"></a>

  Return the end time.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### initial_locked <a id="VestingEscrowSimple-initial-locked"></a>

  Return the initial locked.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### total_claimed <a id="VestingEscrowSimple-total-claimed"></a>

  Return the total claimed.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### initial_locked_supply <a id="VestingEscrowSimple-initial-locked-supply"></a>

  Return the initial_locked_supply.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### can_disable <a id="VestingEscrowSimple-can-disable"></a>

  Return the can_disable.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `bool`.

- ### disabled_at <a id="VestingEscrowSimple-disabled-at"></a>

  Return the disabled_at.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### admin <a id="VestingEscrowSimple-admin"></a>

  Returns the admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### future_admin <a id="VestingEscrowSimple-future-admin"></a>

  Returns the future admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

## Deploying Vesting Escrow Factory contract manually

If you need to deploy the `Vesting Escrow Factory` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - target : Vesting escrow simple deployed contract package hash
  - admin : Admin account hash for vesting escrow factory
  - contract_name : Contract name for deployment

Following is the command to deploy the `Vesting Escrow Factory contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 230000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="target:Key='`VestingEscrowSimple` contract address'" \
    --session-arg="admin:Key='admin address'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="VestingEscrowFactory-entry-point-methods"></a>

Following are the VestingEscrowFactory's entry point methods.

- ### deploy_vesting_contract <a id="VestingEscrowFactory-deploy-vesting-contract"></a>

  Deploy a new vesting contract. Each contract holds tokens which vest for a single account. Tokens must be sent to this contract via the regular `ERC20.transfer` method prior to calling this method.

  Following is the table of parameters.

  | Parameter Name   | Type           |
  | ---------------- | -------------- |
  | token            | Key            |
  | recipient        | Key            |
  | amount           | U256           |
  | can_disable      | bool           |
  | vesting_duration | U256           |
  | vesting_start    | Option`<U256>` |

  This method **returns** `Key`.

- ### commit_transfer_ownership_vef <a id="VestingEscrowFactory-commit-transfer-ownership-vef"></a>

  Transfer ownership of GaugeController to `addr`

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** nothing.

- ### apply_transfer_ownership_vef <a id="VestingEscrowFactory-apply-transfer-ownership-vef"></a>

  Apply a pending ownership transfer

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### future_admin_vef <a id="VestingEscrowFactory-future-admin-vef"></a>

  Returns the future admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### admin_vef <a id="VestingEscrowFactory-admin-vef"></a>

  Returns the admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### target <a id="VestingEscrowFactory-target"></a>

  Returns the admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### initialize <a id="VestingEscrowFactory-initialize"></a>

  Initialize the contract. This function is seperate from `__init__` because of the factory pattern used in `VestingEscrowFactory.deploy_vesting_contract`. It may be called once per deployment.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | admin          | Key  |
  | token          | Key  |
  | recipient      | Key  |
  | amount         | U256 |
  | start_time     | U256 |
  | end_time       | U256 |
  | can_disable    | bool |

  This method **returns** `bool`.

- ### toggle_disable <a id="VestingEscrowFactory-toggle-disable"></a>

  Disable or re-enable a vested address's ability to claim tokens. When disabled, the address is only unable to claim tokens which are still locked at the time of this call. It is not possible to block the claim of tokens which have already vested.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |

  This method **returns** nothing.

- ### disable_can_disable <a id="VestingEscrowFactory-disable-can-disable"></a>

  Disable the ability to call `toggle_disable`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### vested_supply <a id="VestingEscrowFactory-vested-supply"></a>

  Get the total number of tokens which have vested, that are held by this contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### lock_supply <a id="VestingEscrowFactory-lock-supply"></a>

  Get the total number of tokens which are still locked (have not yet vested).

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### vested_of <a id="VestingEscrowFactory-vested-of"></a>

  Get the number of tokens which have vested for a given address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |

  This method **returns** `U256`.

- ### balance_of <a id="VestingEscrowFactory-balance-of"></a>

  Get the number of locked tokens for a given address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |

  This method **returns** `U256`.

- ### locked_of <a id="VestingEscrowFactory-locked-of"></a>

  Get the number of locked tokens for a given address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |

  This method **returns** `U256`.

- ### claim <a id="VestingEscrowFactory-claim"></a>

  Claim tokens which have vested.

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | addr           | Option`<Key>` |

  This method **returns** `U256`.

- ### commit_transfer_ownership <a id="VestingEscrowFactory-commit-transfer-ownership"></a>

  Transfer ownership of GaugeController to `addr`

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** nothing.

- ### apply_transfer_ownership <a id="VestingEscrowFactory-apply-transfer-ownership"></a>

  Apply a pending ownership transfer

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### token <a id="VestingEscrowFactory-token"></a>

  Return the token address.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### start_time <a id="VestingEscrowFactory-start-time"></a>

  Return the start time.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### end_time <a id="VestingEscrowFactory-end-time"></a>

  Return the end time.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### initial_locked <a id="VestingEscrowFactory-initial-locked"></a>

  Return the initial locked.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### total_claimed <a id="VestingEscrowFactory-total-claimed"></a>

  Return the total claimed.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### initial_locked_supply <a id="VestingEscrowFactory-initial-locked-supply"></a>

  Return the initial_locked_supply.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### can_disable <a id="VestingEscrowFactory-can-disable"></a>

  Return the can_disable.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `bool`.

- ### disabled_at <a id="VestingEscrowFactory-disabled-at"></a>

  Return the disabled_at.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** `U256`.

- ### admin <a id="VestingEscrowFactory-admin"></a>

  Returns the admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### future_admin <a id="VestingEscrowFactory-future-admin"></a>

  Returns the future admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

## Deploying Voting Escrow contract manually

If you need to deploy the `Voting Escrow` contract manually you need to pass following parameters

- chain-name : Name of casper chain to perform deployment on
- node-address : Address of chain node to perform transactions
- secret-key : Path to the account's secret key file (.pem)
- session-path : Path to Curve-Dao wasm file (.wasm)
- payment-amount : Gas amount in motes (unit) to perform wasm deployment
- session-arg : Parameters for the contract initializer
  - public_key : Deployer account's public key in hex
  - token : Contract package hash of the ERC20 token being distributed
  - name : Token name of contract to deploy
  - symbol : Token symbol of contract to deploy
  - version : Version handling of the contract
  - contract_name : Contract name for deployment

Following is the command to deploy the `Voting Escrow contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 250000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="token_addr:Key='ERC20CRV token address'" \
    --session-arg="name:String='Token name'" \
    --session-arg="symbol:String='Token symbol'" \
    --session-arg="version:String='Contract version'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="VotingEscrow-entry-point-methods"></a>

Following are the VotingEscrow's entry point methods.

- ### commit_transfer_ownership <a id="VotingEscrow-commit-transfer-ownership"></a>

  Transfer ownership of VotingEscrow contract to `addr`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** nothing.

- ### apply_transfer_ownership <a id="VotingEscrow-apply-transfer-ownership"></a>

  Apply a pending ownership transfer

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### get_last_user_slope <a id="VotingEscrow-get-last-user-slope"></a>

  Get the most recently recorded rate of voting power decrease for `addr`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** `Tuple i128`.

- ### user_point_history_ts <a id="VotingEscrow-user-point-history-ts"></a>

  Get the timestamp for checkpoint `idx` for `addr`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |
  | idx            | U256 |

  This method **returns** `U256`.

- ### locked_end <a id="VotingEscrow-locked-end"></a>

  Get timestamp when `addr`'s lock finishes. Return Epoch time of the lock end.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** `U256`.

- ### checkpoint <a id="VotingEscrow-checkpoint"></a>

  Record global data to checkpoint.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### deposit_for <a id="VotingEscrow-deposit-for"></a>

  Deposit `value` tokens for `addr` and add to the lock. Anyone (even a smart contract) can deposit for someone else, but cannot extend their locktime and deposit for a brand new user.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |
  | value          | U256 |

  This method **returns** nothing.

- ### create_lock <a id="VotingEscrow-create-lock"></a>

  Deposit `value` tokens for `msg.sender` and lock until `unlock_time`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | value          | U256 |
  | unlock_time    | U256 |

  This method **returns** nothing.

- ### increase_amount <a id="VotingEscrow-increase-amount"></a>

  Deposit `value` additional tokens for `msg.sender` without modifying the unlock time.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | value          | U256 |

  This method **returns** nothing.

- ### increase_unlock_time <a id="VotingEscrow-increase-unlock-time"></a>

  Extend the unlock time for `msg.sender` to `unlock_time`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | unlock_time    | U256 |

  This method **returns** nothing.

- ### withdraw <a id="VotingEscrow-withdraw"></a>

  Withdraw all tokens for `msg.sender`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- ### balance_of <a id="VotingEscrow-balance_of"></a>

  Get the current voting power for `msg.sender`. Adheres to the ERC20 `balanceOf` interface for Aragon compatibility.

  Following is the table of parameters.

  | Parameter Name | Type           |
  | -------------- | -------------- |
  | addr           | Key            |
  | t              | Option`<U256>` |

  This method **returns** `U256`.

- ### balance_of_at <a id="VotingEscrow-balance_of_at"></a>

  Measure voting power of `addr` at block height `block`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |
  | time           | U256 |

  This method **returns** `U256`.

- ### total_supply <a id="VotingEscrow-total_supply"></a>

  Calculate total voting power. Adheres to the ERC20 `totalSupply` interface for Aragon compatibility.

  Following is the table of parameters.

  | Parameter Name | Type           |
  | -------------- | -------------- |
  | t              | Option`<U256>` |

  This method **returns** `U256`.

- ### total_supply_at <a id="VotingEscrow-total_supply_at"></a>

  Calculate total voting power at some point in the past. Returns Total voting power at `block`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | block          | U256 |

  This method **returns** `U256`.

- ### change_controller <a id="VotingEscrow-change-controller"></a>

  Change the controller. Dummy method required for Aragon compatibility.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | new_controller | Key  |

  This method **returns** nothing.

- ### future_admin <a id="VotingEscrow-future-admin"></a>

  Returns the future admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### admin <a id="VotingEscrow-admin"></a>

  Returns the admin of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### token <a id="VotingEscrow-token"></a>

  Returns the token of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### supply <a id="VotingEscrow-supply"></a>

  Returns the supply of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### locked <a id="VotingEscrow-locked"></a>

  Returns the locked balance of contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | addr           | Key  |

  This method **returns** `LockedBalance`.

- ### epoch <a id="VotingEscrow-epoch"></a>

  Returns the epoch.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- ### point_history <a id="VotingEscrow-point-history"></a>

  Returns the point history.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | epoch          | U256 |

  This method **returns** `Point`.

- ### user_point_history <a id="VotingEscrow-user-point-history"></a>

  Returns the user_point_history.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | user           | Key  |
  | user_epoch     | U256 |

  This method **returns** `Point`.

- ### user_point_epoch <a id="VotingEscrow-user-point-epoch"></a>

  Returns the user_point_epoch.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | user           | Key  |

  This method **returns** `U256`.

- ### slope_changes <a id="VotingEscrow-slope-changes"></a>

  Returns the slope_changes.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | time           | U256 |

  This method **returns** `Tuple(bool,U128)`.

- ### controller <a id="VotingEscrow-controller"></a>

  Returns the address of controller.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- ### transfers_enabled <a id="VotingEscrow-transfers-enabled"></a>

  Returns the transfers_enabled.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `bool`.

- ### name <a id="VotingEscrow-name"></a>

  Returns the name.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `String`.

- ### symbol <a id="VotingEscrow-symbol"></a>

  Returns the symbol.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `String`.

- ### version <a id="VotingEscrow-version"></a>

  Returns the version.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `String`.

- ### decimals <a id="VotingEscrow-decimals"></a>

  Returns the decimals.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.
