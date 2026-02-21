# Sol Flex

Solana/Anchor program for reflection fee distribution with:

- authority-managed configuration
- blocklist and per-user ban controls
- batch payout cursoring
- optional pool preference metadata

This program is a fee distribution contract, not a token contract.

## Program ID

`5im5SdEc2dg63B5C9vm83mwQqxGUAphG2K47uGgA69ZS`

## Implemented Instructions

- `initialize`
  - Creates `Config` PDA.
- `update_config`
  - Updates authority and reflection thresholds.
- `set_distribution_config`
  - Creates and sets `DistributionConfig` PDA (limit, rates, token mint, accounts).
- `add_to_blocklist` / `remove_from_blocklist`
  - Maintains global blocklist.
- `set_user_preferences`
  - Creates `UserPreferences` PDA for a user and stores preferences.
- `ban_user`
  - Sets per-user ban status.
- `add_pool` / `remove_pool`
  - Maintains `GlobalTokenPools` registry.
- `reflect`
  - Executes batch reflection transfer in configured base asset.

## Reflection Behavior (Current)

`reflect` currently does all of the following:

- Requires signer to match `config.authority`.
- Requires `fee_pool.reflection_pool >= config.min_reflection_pool`.
- Uses batch cap `distribution_config.limit`.
- Uses cursor `distribution_config.start_key`.
- Reads remaining accounts in pairs:
  - `[user_preferences, recipient_token_account]` repeated.
- Skips users that are:
  - behind/equal to cursor
  - banned
  - blocklisted
- Handles pool preference:
  - if `preferred_pool_id == 0`: default configured asset is used
  - if `preferred_pool_id != 0` and pool is invalid/inactive: falls back to default configured asset
  - if `preferred_pool_id != 0` and pool is valid/active: default configured asset is still used (no swap CPI yet)
- Transfer amount:
  - computes `amount_to_distribute = fee_pool.reflection_pool / 10`
  - splits evenly across processed recipients
  - enforces `per_recipient_amount >= config.min_reflection_per_account`
- Performs SPL `token::transfer` from `fee_vault` to each recipient ATA using `fee_pool` PDA signer seeds.
- Updates:
  - `fee_pool.reflection_pool` (deduct distributed amount)
  - `distribution_config.start_key` (last processed owner)
  - `distribution_config.updated_at`

## Required `reflect` Accounts

Primary accounts:

- `config` PDA
- `distribution_config` PDA
- `fee_pool` PDA
- `fee_vault` token account (mint == configured token mint, owner == `fee_pool` PDA)
- `token_mint` account (must equal configured token mint)
- `global_pools` PDA
- `authority` signer
- `token_program`, `system_program`

Remaining accounts:

- strict pair layout:
  - `user_preferences`, `recipient_token_account`
  - repeated N times

## Current Non-Goals / Not Yet Implemented

- Jupiter swap CPI execution is not implemented in `reflect` yet.
- Burn/project/dev fee transfer execution is not implemented in `reflect`.
- `reflect` does not create recipient token accounts; they must already exist.

## Account Models

- `Config`
  - authority, blocklist, thresholds, bump
- `DistributionConfig`
  - token mint, cursor (`start_key`), batch `limit`, fee rates, project/dev accounts, bump
- `FeePool`
  - reflection/burn/project/dev accumulators, total fees, bump
- `UserPreferences`
  - owner, `preferred_pool_id`, memo, tree fields, ban flag
- `GlobalTokenPools`
  - pool registry and authority

## Constants and Limits

- `MAX_BLOCKLIST_SIZE = 1000`
- `MAX_MEMO_LENGTH = 200`
- Distribution limit validation: `1..=1000` in `set_distribution_config`

PDA seed constants:

- `CONFIG_SEED`
- `TOKEN_ACCOUNT_SEED`
- `TOKEN_POOL_SEED`
- `USER_PREFERENCES_SEED`
- `GLOBAL_POOLS_SEED`
- `DISTRIBUTION_CONFIG_SEED`
- `FEE_POOL_SEED`
- `POSITION_SEED`

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

## Build / Test / Deploy

```bash
anchor build
anchor test
anchor deploy
```
