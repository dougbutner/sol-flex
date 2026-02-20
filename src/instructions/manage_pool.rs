use anchor_lang::prelude::*;
use crate::state::{GlobalTokenPools, TokenPool, Config};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddPoolParams {
    pub pool_id: u64,
    pub token_mint: Pubkey,
    pub token_program: Pubkey,
    pub pool_address: Pubkey,
}

#[derive(Accounts)]
#[instruction(params: AddPoolParams)]
pub struct AddPool<'info> {
    #[account(
        seeds = [Config::SEED_PREFIX],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [GlobalTokenPools::SEED_PREFIX],
        bump = global_pools.bump
    )]
    pub global_pools: Account<'info, GlobalTokenPools>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn add_pool_handler(ctx: Context<AddPool>, params: AddPoolParams) -> Result<()> {
    let config = &ctx.accounts.config;
    let global_pools = &mut ctx.accounts.global_pools;
    let authority = &ctx.accounts.authority;

    // Check authorization
    require!(authority.key() == config.authority, crate::errors::SolFlexError::Unauthorized);

    // Create new pool
    let pool = TokenPool::new(
        params.pool_id,
        params.token_mint,
        params.token_program,
        params.pool_address,
    );

    // Add pool to global registry
    global_pools.add_pool(pool)?;

    msg!("Added token pool {} for mint {}", params.pool_id, params.token_mint);

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RemovePoolParams {
    pub pool_id: u64,
}

#[derive(Accounts)]
#[instruction(params: RemovePoolParams)]
pub struct RemovePool<'info> {
    #[account(
        seeds = [Config::SEED_PREFIX],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [GlobalTokenPools::SEED_PREFIX],
        bump = global_pools.bump
    )]
    pub global_pools: Account<'info, GlobalTokenPools>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn remove_pool_handler(ctx: Context<RemovePool>, params: RemovePoolParams) -> Result<()> {
    let config = &ctx.accounts.config;
    let global_pools = &mut ctx.accounts.global_pools;
    let authority = &ctx.accounts.authority;

    // Check authorization
    require!(authority.key() == config.authority, crate::errors::SolFlexError::Unauthorized);

    // Remove pool from global registry
    global_pools.remove_pool(params.pool_id)?;

    msg!("Removed token pool {}", params.pool_id);

    Ok(())
}