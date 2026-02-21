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
- `initialize_global_pools`
  - Creates/updates `GlobalTokenPools` PDA.
- `update_config`
  - Updates authority and reflection thresholds.
- `set_distribution_config`
  - Creates/updates `DistributionConfig` PDA (limit, rates, token mint, vault/account settings).
- `add_to_blocklist` / `remove_from_blocklist`
  - Maintains global blocklist.
- `set_user_preferences`
  - Creates/updates `UserPreferences` PDA for a user and stores preferences.
- `ban_user`
  - Sets per-user ban status.
- `add_pool` / `remove_pool`
  - Maintains `GlobalTokenPools` registry.
- `reflect`
  - Executes batch reflection transfer in configured base asset.
- `record_fees`
  - Splits recorded incoming fees into reflection/dev buckets tracked inside `DistributionConfig`.

## Reflection Behavior (Current)

`reflect` currently does all of the following:

- Requires signer to match `config.authority`.
- Requires `distribution_config.reflection_pool >= config.min_reflection_pool`.
- Uses batch cap `distribution_config.limit`.
- Uses cursor `distribution_config.last_paid`.
- Reads remaining accounts in pairs:
  - `[user_preferences, recipient_token_account]` repeated.
- Requires remaining account pairs to be provided in strictly increasing owner order.
- Skips users that are:
  - behind/equal to cursor
  - banned
  - blocklisted
- Handles pool preference:
  - if `preferred_pool_id == 0`: default configured asset is used
  - if `preferred_pool_id != 0` and pool is invalid/inactive: falls back to default configured asset
  - if `preferred_pool_id != 0` and pool is valid/active: default configured asset is still used (no swap CPI yet)
- Transfer amount:
  - computes `amount_to_distribute = distribution_config.reflection_pool / 10`
  - splits evenly across processed recipients
  - enforces `per_recipient_amount >= config.min_reflection_per_account`
- Performs SPL `token::transfer` from `fee_vault` to each recipient ATA using `distribution_config` PDA signer seeds.
- Pays accumulated `dev_pool` to configured `dev_account` token account.
- Updates:
  - `distribution_config.reflection_pool` (deduct distributed amount)
  - `distribution_config.dev_pool` (deduct dev payout amount)
  - `distribution_config.last_paid` (last processed owner)
  - `distribution_config.updated_at`
- End-of-list behavior:
  - if no eligible account exists after current `last_paid`, cursor is reset to default so next run starts a new cycle

## Required `reflect` Accounts

Primary accounts:

- `config` PDA
- `distribution_config` PDA
- `fee_vault` token account (mint == configured token mint, owner == `distribution_config` PDA)
- `dev_token_account` token account (must equal configured `dev_account`)
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
- Burn/project fee transfer execution is not implemented in `reflect`.
- `reflect` does not create recipient token accounts; they must already exist.

## Account Models

- `Config`
  - authority, blocklist, thresholds, bump
- `DistributionConfig`
  - token mint, `fee_vault`, reflection/dev counters, total fees, cursor (`last_paid`), batch `limit`, fee rates, project/dev accounts, bump
- `UserPreferences`
  - owner, `preferred_pool_id`, memo, ban flag
- `GlobalTokenPools`
  - pool registry and authority

## Constants and Limits

- `MAX_BLOCKLIST_SIZE = 100`
- `MAX_MEMO_LENGTH = 200`
- Distribution limit validation: `1..=1000` in `set_distribution_config`

PDA seed constants:

- `CONFIG_SEED`
- `TOKEN_ACCOUNT_SEED`
- `TOKEN_POOL_SEED`
- `USER_PREFERENCES_SEED`
- `GLOBAL_POOLS_SEED`
- `DISTRIBUTION_CONFIG_SEED`

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
