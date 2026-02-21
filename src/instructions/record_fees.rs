use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount as SplTokenAccount};
use crate::state::{Config, DistributionConfig};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RecordFeesParams {
    pub amount: u64,
}

#[derive(Accounts)]
#[instruction(params: RecordFeesParams)]
pub struct RecordFees<'info> {
    #[account(
        seeds = [Config::SEED_PREFIX],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [DistributionConfig::SEED_PREFIX],
        bump = distribution_config.bump
    )]
    pub distribution_config: Account<'info, DistributionConfig>,

    #[account(
        mut,
        constraint = fee_vault.key() == distribution_config.fee_vault @ crate::errors::SolFlexError::InvalidTokenAccount,
        constraint = fee_vault.mint == distribution_config.token_mint @ crate::errors::SolFlexError::InvalidTokenAccount,
        constraint = fee_vault.owner == distribution_config.key() @ crate::errors::SolFlexError::InvalidTokenAccount
    )]
    pub fee_vault: Account<'info, SplTokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<RecordFees>, params: RecordFeesParams) -> Result<()> {
    let config = &ctx.accounts.config;
    let distribution_config = &mut ctx.accounts.distribution_config;

    require!(
        ctx.accounts.authority.key() == config.authority,
        crate::errors::SolFlexError::Unauthorized
    );
    require!(params.amount > 0, crate::errors::SolFlexError::InvalidParameters);

    // For the current distribution model, incoming fees are split only
    // between reflection and dev pools.
    distribution_config.validate_reflection_dev_only_split()?;

    let reflection_amount = params
        .amount
        .checked_mul(distribution_config.reflection_rate as u64)
        .ok_or(crate::errors::SolFlexError::ArithmeticOverflow)?
        / 10000;
    let dev_amount = params
        .amount
        .checked_mul(distribution_config.dev_fee_rate as u64)
        .ok_or(crate::errors::SolFlexError::ArithmeticOverflow)?
        / 10000;

    distribution_config.reflection_pool = distribution_config
        .reflection_pool
        .checked_add(reflection_amount)
        .ok_or(crate::errors::SolFlexError::ArithmeticOverflow)?;
    distribution_config.dev_pool = distribution_config
        .dev_pool
        .checked_add(dev_amount)
        .ok_or(crate::errors::SolFlexError::ArithmeticOverflow)?;
    distribution_config.total_fees = distribution_config
        .total_fees
        .checked_add(params.amount)
        .ok_or(crate::errors::SolFlexError::ArithmeticOverflow)?;
    distribution_config.updated_at = Clock::get()?.unix_timestamp;

    msg!(
        "Recorded {} incoming fees => reflection={}, dev={}",
        params.amount,
        reflection_amount,
        dev_amount
    );
    Ok(())
}
