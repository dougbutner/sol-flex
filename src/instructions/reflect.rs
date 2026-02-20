use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use crate::state::{Config, DistributionConfig, FeePool, GlobalTokenPools, UserPreferences};

#[derive(Accounts)]
pub struct Reflect<'info> {
    #[account(
        seeds = [Config::SEED_PREFIX],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [DistributionConfig::SEED_PREFIX],
        bump = distribution_config.bump
    )]
    pub distribution_config: Account<'info, DistributionConfig>,

    #[account(
        mut,
        seeds = [FeePool::SEED_PREFIX],
        bump = fee_pool.bump
    )]
    pub fee_pool: Account<'info, FeePool>,

    #[account(
        mut,
        constraint = fee_vault.mint == distribution_config.token_mint @ crate::errors::SolFlexError::InvalidTokenAccount,
        constraint = fee_vault.owner == fee_pool.key() @ crate::errors::SolFlexError::InvalidTokenAccount
    )]
    pub fee_vault: Account<'info, TokenAccount>,

    #[account(
        constraint = token_mint.key() == distribution_config.token_mint @ crate::errors::SolFlexError::InvalidTokenAccount
    )]
    pub token_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [GlobalTokenPools::SEED_PREFIX],
        bump = global_pools.bump
    )]
    pub global_pools: Account<'info, GlobalTokenPools>,

    /// CHECK: Authority account
    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Reflect>) -> Result<()> {
    let config = &ctx.accounts.config;
    let distribution_config = &mut ctx.accounts.distribution_config;
    let fee_pool = &mut ctx.accounts.fee_pool;
    let global_pools = &ctx.accounts.global_pools;
    let fee_vault = &ctx.accounts.fee_vault;

    require!(
        ctx.accounts.authority.key() == config.authority,
        crate::errors::SolFlexError::Unauthorized
    );

    // Check if there are sufficient reflections to distribute
    require!(
        fee_pool.reflection_pool >= config.min_reflection_pool,
        crate::errors::SolFlexError::NoReflectionsToDistribute
    );

    msg!(
        "Reflection distribution: pool minimum={}, per-account minimum={}, batch limit={}",
        config.min_reflection_pool,
        config.min_reflection_per_account,
        distribution_config.limit
    );

    // Remaining accounts layout:
    // [user_preferences, recipient_token_account, user_preferences, recipient_token_account, ...]
    require!(
        ctx.remaining_accounts.len() % 2 == 0,
        crate::errors::SolFlexError::InvalidRemainingAccounts
    );

    let batch_limit = distribution_config.limit as usize;
    let mut recipients: Vec<AccountInfo<'_>> = Vec::new();
    let mut last_processed = distribution_config.start_key;

    for pair in ctx.remaining_accounts.chunks_exact(2) {
        if recipients.len() >= batch_limit {
            break;
        }

        let pref_info = &pair[0];
        let recipient_token_info = &pair[1];
        let user_pref: Account<UserPreferences> = Account::try_from(pref_info)?;

        // Cursor-based batching.
        if user_pref.owner <= distribution_config.start_key {
            continue;
        }
        if user_pref.is_banned || config.is_blocklisted(user_pref.owner) {
            continue;
        }

        // Default route always sends configured base asset.
        // If a pool preference exists but is invalid/inactive, we gracefully fall back to default.
        if user_pref.preferred_pool_id != 0 {
            let pool_valid_and_active = global_pools
                .get_pool(user_pref.preferred_pool_id)
                .map(|p| p.is_active)
                .unwrap_or(false);
            if !pool_valid_and_active {
                msg!(
                    "Pool {} invalid/inactive for user {}, defaulting to configured asset",
                    user_pref.preferred_pool_id,
                    user_pref.owner
                );
            }
        }

        let recipient_token: Account<TokenAccount> = Account::try_from(recipient_token_info)?;
        require!(
            recipient_token.mint == distribution_config.token_mint
                && recipient_token.owner == user_pref.owner,
            crate::errors::SolFlexError::InvalidTokenAccount
        );

        recipients.push(recipient_token_info.to_account_info());
        last_processed = user_pref.owner;
    }

    require!(
        !recipients.is_empty(),
        crate::errors::SolFlexError::NoEligibleAccounts
    );

    // Keep the existing "10% per run" throttle, then split across this batch.
    let amount_to_distribute = fee_pool.reflection_pool / 10;
    let per_recipient_amount = amount_to_distribute / recipients.len() as u64;
    require!(
        per_recipient_amount >= config.min_reflection_per_account,
        crate::errors::SolFlexError::InvalidParameters
    );

    let signer_seeds: &[&[u8]] = &[FeePool::SEED_PREFIX, &[fee_pool.bump]];
    for recipient in recipients.iter() {
        let cpi_accounts = Transfer {
            from: fee_vault.to_account_info(),
            to: recipient.clone(),
            authority: fee_pool.to_account_info(),
        };
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
                &[signer_seeds],
            ),
            per_recipient_amount,
        )?;
    }

    let distributed_total = per_recipient_amount * recipients.len() as u64;
    fee_pool.distribute_reflection(distributed_total)?;
    distribution_config.start_key = last_processed;
    distribution_config.updated_at = Clock::get()?.unix_timestamp;

    msg!(
        "Distributed {} to {} holders, cursor={}",
        per_recipient_amount,
        recipients.len(),
        distribution_config.start_key
    );

    Ok(())
}