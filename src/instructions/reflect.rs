use anchor_lang::prelude::*;
use crate::state::Config;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ReflectParams {
    pub amount: u64,
    pub recipient: Pubkey,
}

#[derive(Accounts)]
#[instruction(params: ReflectParams)]
pub struct Reflect<'info> {
    #[account(
        seeds = [Config::SEED_PREFIX],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        constraint = recipient.key() == params.recipient
    )]
    pub recipient: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Reflect>, params: ReflectParams) -> Result<()> {
    let config = &ctx.accounts.config;
    let user = &ctx.accounts.user;

    // Check if user is blocklisted
    require!(!config.is_blocklisted(user.key()), crate::errors::SolFlexError::Unauthorized);

    // Validate parameters
    require!(params.amount > 0, crate::errors::SolFlexError::InvalidParameters);

    // Perform the reflection logic (transfer SOL)
    let transfer_ix = anchor_lang::system_program::Transfer {
        from: ctx.accounts.user.to_account_info(),
        to: ctx.accounts.recipient.to_account_info(),
    };

    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_ix,
        ),
        params.amount,
    )?;

    msg!("Reflection executed: {} SOL transferred from {} to {}",
         params.amount,
         user.key(),
         params.recipient);

    Ok(())
}