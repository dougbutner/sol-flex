use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use crate::state::{Config, DistributionConfig, GlobalTokenPools, UserPreferences};

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
        constraint = fee_vault.key() == distribution_config.fee_vault @ crate::errors::SolFlexError::InvalidTokenAccount,
        constraint = fee_vault.mint == distribution_config.token_mint @ crate::errors::SolFlexError::InvalidTokenAccount,
        constraint = fee_vault.owner == distribution_config.key() @ crate::errors::SolFlexError::InvalidTokenAccount
    )]
    pub fee_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = dev_token_account.key() == distribution_config.dev_account @ crate::errors::SolFlexError::InvalidTokenAccount,
        constraint = dev_token_account.mint == distribution_config.token_mint @ crate::errors::SolFlexError::InvalidTokenAccount
    )]
    pub dev_token_account: Account<'info, TokenAccount>,


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
    let global_pools = &ctx.accounts.global_pools;
    let fee_vault = &ctx.accounts.fee_vault;
    let dev_token_account = &ctx.accounts.dev_token_account;

    require!(
        ctx.accounts.authority.key() == config.authority,
        crate::errors::SolFlexError::Unauthorized
    );

    // Check if there are sufficient reflections to distribute
    require!(
        distribution_config.reflection_pool >= config.min_reflection_pool,
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
    let mut new_last_paid = distribution_config.last_paid;
    let mut last_seen_owner: Option<Pubkey> = None;

    for pair in ctx.remaining_accounts.chunks_exact(2) {
        if recipients.len() >= batch_limit {
            break;
        }

        let pref_info = &pair[0];
        let recipient_token_info = &pair[1];
        let user_pref: Account<UserPreferences> = Account::try_from(pref_info)?;

        // Require strictly increasing owner order to make cursoring deterministic.
        if let Some(prev_owner) = last_seen_owner {
            require!(user_pref.owner > prev_owner, crate::errors::SolFlexError::InvalidParameters);
        }
        last_seen_owner = Some(user_pref.owner);

        // Cursor-based batching.
        if user_pref.owner <= distribution_config.last_paid {
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
        new_last_paid = user_pref.owner;
    }

    if recipients.is_empty() {
        // If nothing exists after the current cursor, reset for next cycle.
        if distribution_config.last_paid != Pubkey::default() {
            distribution_config.last_paid = Pubkey::default();
            distribution_config.updated_at = Clock::get()?.unix_timestamp;
            msg!("No eligible accounts after cursor; last_paid reset for next cycle");
            return Ok(());
        }
        return Err(crate::errors::SolFlexError::NoEligibleAccounts.into());
    }

    // Keep the existing "10% per run" throttle, then split across this batch.
    let amount_to_distribute = distribution_config.reflection_pool / 10;
    let per_recipient_amount = amount_to_distribute / recipients.len() as u64;
    require!(
        per_recipient_amount >= config.min_reflection_per_account,
        crate::errors::SolFlexError::InvalidParameters
    );

    let signer_seeds: &[&[u8]] = &[DistributionConfig::SEED_PREFIX, &[distribution_config.bump]];
    for recipient in recipients.iter() {
        let cpi_accounts = Transfer {
            from: fee_vault.to_account_info(),
            to: recipient.clone(),
            authority: distribution_config.to_account_info(),
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
    distribution_config.distribute_reflection(distributed_total)?;

    // Dev pool payout is tracked separately and paid to the configured dev token account.
    if distribution_config.dev_pool > 0 {
        let dev_amount = distribution_config.dev_pool;
        let dev_cpi_accounts = Transfer {
            from: fee_vault.to_account_info(),
            to: dev_token_account.to_account_info(),
            authority: distribution_config.to_account_info(),
        };
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                dev_cpi_accounts,
                &[signer_seeds],
            ),
            dev_amount,
        )?;
        distribution_config.distribute_dev(dev_amount)?;
    }

    distribution_config.last_paid = new_last_paid;
    distribution_config.updated_at = Clock::get()?.unix_timestamp;

    msg!(
        "Distributed {} to {} holders, cursor={}",
        per_recipient_amount,
        recipients.len(),
        distribution_config.last_paid
    );

    Ok(())
}