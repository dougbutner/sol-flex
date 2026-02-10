use anchor_lang::prelude::*;
use crate::state::UserPreferences;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SetUserPreferencesParams {
    pub preferred_token_mint: Pubkey,
    pub custom_memo: String,
    pub tree_parent: Pubkey,
    pub tree_rate: u16,
}

#[derive(Accounts)]
#[instruction(params: SetUserPreferencesParams)]
pub struct SetUserPreferences<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserPreferences::INIT_SPACE,
        seeds = [UserPreferences::SEED_PREFIX, user.key().as_ref()],
        bump
    )]
    pub user_preferences: Account<'info, UserPreferences>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SetUserPreferences>, params: SetUserPreferencesParams) -> Result<()> {
    let user_preferences = &mut ctx.accounts.user_preferences;
    let user = &ctx.accounts.user;

    // Validate parameters
    require!(params.tree_rate <= 10000, crate::errors::SolFlexError::InvalidParameters);
    require!(params.custom_memo.len() <= 200, crate::errors::SolFlexError::InvalidMemoLength);

    // If this is a new account, initialize it
    if user_preferences.owner == Pubkey::default() {
        *user_preferences = UserPreferences::new(user.key());
    }

    // Update preferences
    user_preferences.preferred_token_mint = params.preferred_token_mint;
    user_preferences.custom_memo = params.custom_memo;
    user_preferences.tree_parent = if params.tree_parent == Pubkey::default() {
        user.key()
    } else {
        params.tree_parent
    };
    user_preferences.tree_rate = params.tree_rate;
    user_preferences.updated_at = Clock::get().unwrap().unix_timestamp;

    msg!("User preferences updated for {}", user.key());

    Ok(())
}