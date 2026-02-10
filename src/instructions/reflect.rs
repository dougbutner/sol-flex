use anchor_lang::prelude::*;
use crate::state::{DistributionConfig, FeePool, UserPreferences, GlobalTokenPools, Config};
use std::collections::BTreeMap;

#[derive(Accounts)]
pub struct Reflect<'info> {
    #[account(
        seeds = [Config::SEED_PREFIX],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [DistributionConfig::SEED_PREFIX],
        bump
    )]
    pub distribution_config: Account<'info, DistributionConfig>,

    #[account(
        mut,
        seeds = [FeePool::SEED_PREFIX],
        bump
    )]
    pub fee_pool: Account<'info, FeePool>,

    #[account(
        mut,
        seeds = [GlobalTokenPools::SEED_PREFIX],
        bump
    )]
    pub global_pools: Account<'info, GlobalTokenPools>,

    /// CHECK: Authority account
    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Reflect>) -> Result<()> {
    let _config = &ctx.accounts.config;
    let distribution_config = &mut ctx.accounts.distribution_config;
    let fee_pool = &mut ctx.accounts.fee_pool;
    let _global_pools = &ctx.accounts.global_pools;

    // Check if there are sufficient reflections to distribute
    require!(fee_pool.reflection_pool >= _config.min_reflection_pool, crate::errors::SolFlexError::NoReflectionsToDistribute);

    msg!("Reflection distribution: pool minimum={}, per-account minimum={}", _config.min_reflection_pool, _config.min_reflection_per_account);

    // Placeholder logic - distribute a portion of the reflection pool
    let amount_to_distribute = fee_pool.reflection_pool / 10; // Distribute 10% at a time

    if amount_to_distribute > 0 {
        require!(amount_to_distribute >= _config.min_reflection_per_account, crate::errors::SolFlexError::InvalidParameters);

        // In full implementation, iterate through user preferences and check:
        // for each user_preference in user_list {
        //     if user_token_balance < config.min_reflection_per_account {
        //         continue; // Skip accounts below minimum to prevent small transfers
        //     }
        //     // Process distribution for eligible accounts only
        // }

        fee_pool.distribute_reflection(amount_to_distribute)?;

        msg!("Distributed {} tokens in reflections", amount_to_distribute);
    }

    // Handle burn and project fees
    if fee_pool.burn_pool > 0 {
        // TODO: Implement burn logic (could send to a burn address)
        msg!("Burned {} tokens", fee_pool.burn_pool);
    }

    if fee_pool.project_pool > 0 && distribution_config.project_account != Pubkey::default() {
        // TODO: Transfer project fees to project account
        msg!("Distributed {} tokens to project account", fee_pool.project_pool);
    }

    // Clear the pools after distribution
    fee_pool.clear_pools();

    msg!("Reflection distribution completed");

    Ok(())
}