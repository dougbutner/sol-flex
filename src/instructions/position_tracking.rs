use anchor_lang::prelude::*;
use crate::state::{Position, UserPreferences};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreatePositionParams {
    pub pool_id: u64,
    pub amount_invested: u64,
}

#[derive(Accounts)]
#[instruction(params: CreatePositionParams)]
pub struct CreatePosition<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + Position::INIT_SPACE,
        seeds = [Position::SEED_PREFIX, user.key().as_ref(), &params.pool_id.to_le_bytes()],
        bump
    )]
    pub position: Account<'info, Position>,

    #[account(
        seeds = [UserPreferences::SEED_PREFIX, user.key().as_ref()],
        bump
    )]
    pub user_preferences: Account<'info, UserPreferences>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_position_handler(ctx: Context<CreatePosition>, params: CreatePositionParams) -> Result<()> {
    let position = &mut ctx.accounts.position;
    let user = &ctx.accounts.user;

    *position = Position::new(user.key(), params.pool_id, params.amount_invested);

    msg!("Position created for user {} in pool {}", user.key(), params.pool_id);

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdatePositionParams {
    pub pool_id: u64,
    pub new_value: u64,
}

#[derive(Accounts)]
#[instruction(params: UpdatePositionParams)]
pub struct UpdatePosition<'info> {
    #[account(
        mut,
        seeds = [Position::SEED_PREFIX, user.key().as_ref(), &params.pool_id.to_le_bytes()],
        bump
    )]
    pub position: Account<'info, Position>,

    #[account(mut)]
    pub user: Signer<'info>,
}

pub fn update_position_handler(ctx: Context<UpdatePosition>, params: UpdatePositionParams) -> Result<()> {
    let position = &mut ctx.accounts.position;

    position.current_value = params.new_value;
    position.updated_at = Clock::get().unwrap().unix_timestamp;

    msg!("Position updated for user {} in pool {}", ctx.accounts.user.key(), params.pool_id);

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ClosePositionParams {
    pub pool_id: u64,
}

#[derive(Accounts)]
#[instruction(params: ClosePositionParams)]
pub struct ClosePosition<'info> {
    #[account(
        mut,
        close = user,
        seeds = [Position::SEED_PREFIX, user.key().as_ref(), &params.pool_id.to_le_bytes()],
        bump
    )]
    pub position: Account<'info, Position>,

    #[account(mut)]
    pub user: Signer<'info>,
}

pub fn close_position_handler(ctx: Context<ClosePosition>, params: ClosePositionParams) -> Result<()> {
    msg!("Position closed for user {} in pool {}", ctx.accounts.user.key(), params.pool_id);

    Ok(())
}