use anchor_lang::prelude::*;
use crate::state::{DistributionConfig, Config};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SetDistributionConfigParams {
    pub token_mint: Pubkey,
    pub limit: u32,
    pub reflection_rate: u16,
    pub burn_rate: u16,
    pub project_rate: u16,
    pub project_account: Pubkey,
    pub dev_fee_rate: u16,
    pub dev_account: Pubkey,
}

#[derive(Accounts)]
#[instruction(params: SetDistributionConfigParams)]
pub struct SetDistributionConfig<'info> {
    #[account(
        seeds = [Config::SEED_PREFIX],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = authority,
        space = 8 + DistributionConfig::INIT_SPACE,
        seeds = [DistributionConfig::SEED_PREFIX],
        bump
    )]
    pub distribution_config: Account<'info, DistributionConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SetDistributionConfig>, params: SetDistributionConfigParams) -> Result<()> {
    let config = &ctx.accounts.config;
    let distribution_config = &mut ctx.accounts.distribution_config;
    let authority = &ctx.accounts.authority;

    // Check authorization
    require!(authority.key() == config.authority, crate::errors::SolFlexError::Unauthorized);

    // Validate parameters
    require!(params.limit > 0 && params.limit <= 1000, crate::errors::SolFlexError::InvalidParameters);

    // Initialize the distribution config
    let bump = ctx.bumps.distribution_config;
    **distribution_config = DistributionConfig::new(
        params.token_mint,
        params.project_account,
        params.dev_account,
        bump,
    );

    // Update configuration
    distribution_config.token_mint = params.token_mint;
    distribution_config.limit = params.limit;
    distribution_config.reflection_rate = params.reflection_rate;
    distribution_config.burn_rate = params.burn_rate;
    distribution_config.project_rate = params.project_rate;
    distribution_config.project_account = params.project_account;
    distribution_config.dev_fee_rate = params.dev_fee_rate;
    distribution_config.dev_account = params.dev_account;
    distribution_config.updated_at = Clock::get().unwrap().unix_timestamp;

    // Validate rates
    distribution_config.validate_rates()?;

    msg!("Distribution configuration updated");

    Ok(())
}