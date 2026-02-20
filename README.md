# Sol Flex

Sol Flex is an Anchor program that distributes rewards from a program-owned token vault.
It is not a token contract and does not mint tokens. It only transfers tokens it already holds.

## Program ID

`5im5SdEc2dg63B5C9vm83mwQqxGUAphG2K47uGgA69ZS`

## Current Instruction Set

- `initialize`
  - Creates the `Config` PDA and sets the initial authority.
- `update_config`
  - Updates authority and reflection thresholds.
- `set_distribution_config`
  - Creates/sets distribution parameters (mint, rates, batch limit, payout accounts).
- `add_to_blocklist` / `remove_from_blocklist`
  - Manages global blocklist entries in `Config`.
- `set_user_preferences`
  - Creates and updates per-user preferences.
- `ban_user`
  - Sets user ban status in `UserPreferences`.
- `add_pool` / `remove_pool`
  - Manages `GlobalTokenPools`.
- `reflect`
  - Executes batched reward distribution from the program vault.

## Reflection Behavior (Implemented)

- Requires authority signer to match `Config.authority`.
- Requires `fee_pool.reflection_pool >= config.min_reflection_pool`.
- Processes recipients in batches:
  - Uses `distribution_config.limit` as hard cap per call.
  - Uses `distribution_config.start_key` as cursor (only owners greater than cursor are processed).
  - Updates `distribution_config.start_key` to the last processed owner.
- Skips users who are:
  - globally blocklisted (`Config.blocklist`)
  - banned (`UserPreferences.is_banned`)
- Default payout behavior:
  - Rewards are sent in the configured base token (`distribution_config.token_mint`).
  - If a user has a non-zero `preferred_pool_id` and that pool is missing/inactive, payout falls back to default base token.
- Payout amount model:
  - Uses 10% of current `fee_pool.reflection_pool` per run.
  - Splits evenly across eligible recipients in this batch.
  - Enforces `per_recipient_amount >= config.min_reflection_per_account`.
- Transfers are SPL token transfers from `fee_vault` to recipient token accounts, signed by the `fee_pool` PDA.

## Required Accounts for `reflect`

Fixed accounts:
- `config` PDA
- `distribution_config` PDA
- `fee_pool` PDA
- `fee_vault` token account
- `token_mint` mint account
- `global_pools` PDA
- `authority` signer
- `token_program`
- `system_program`

Remaining accounts (strict pair layout):
- `[user_preferences, recipient_token_account, user_preferences, recipient_token_account, ...]`

Validation on each pair:
- `recipient_token_account.mint == distribution_config.token_mint`
- `recipient_token_account.owner == user_preferences.owner`

## State Accounts

- `Config`
  - Authority, blocklist, reflection thresholds, bump.
- `DistributionConfig`
  - Base token mint, cursor (`start_key`), batch limit, fee rates, payout accounts, bump.
- `FeePool`
  - Reflection/burn/project/dev pools, total fee counters, bump.
- `UserPreferences`
  - Preferred pool ID, memo, tree metadata, ban status.
- `GlobalTokenPools`
  - List of pool entries and authority.
- `TokenPool`
  - Pool id, mint/program/address, active flag.

## Notes on Pool/Swap Routing

- Pool metadata and user pool preferences are stored.
- Actual swap execution is not implemented in `reflect` yet.
- Current runtime behavior is safe fallback to default token payout when pool routing is not usable.

## Constants and Limits

- Max blocklist size: `1000`
- Max memo length: `200`
- Distribution limit bounds in `set_distribution_config`: `1..=1000`

## Error Codes

- `Unauthorized`
- `InvalidConfig`
- `AlreadyInBlocklist`
- `NotInBlocklist`
- `BlocklistFull`
- `InvalidParameters`
- `ArithmeticOverflow`
- `PoolAlreadyExists`
- `PoolNotFound`
- `InsufficientFunds`
- `NoReflectionsToDistribute`
- `InvalidMemoLength`
- `AccountNotFound`
- `InvalidRemainingAccounts`
- `InvalidTokenAccount`
- `NoEligibleAccounts`

## Build and Test

```bash
anchor build
anchor test
anchor deploy
```