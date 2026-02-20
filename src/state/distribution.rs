use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DistributionConfig {
    pub token_mint: Pubkey, // The base token mint for fees
    pub start_key: Pubkey, // For pagination (last processed user)
    pub limit: u32, // Max users to process per reflect call
    pub reflection_rate: u16, // Reflection fee rate (basis points)
    pub burn_rate: u16, // Burn fee rate (basis points)
    pub project_rate: u16, // Project fee rate (basis points)
    pub project_account: Pubkey, // Account to receive project fees
    pub dev_fee_rate: u16, // Dev fee rate (0.2% = 20 basis points)
    pub dev_account: Pubkey, // Account to receive dev fees
    pub created_at: i64,
    pub updated_at: i64,
    pub bump: u8, // Store PDA bump seed for efficiency
}

impl DistributionConfig {
    pub const SEED_PREFIX: &'static [u8] = crate::constants::DISTRIBUTION_CONFIG_SEED;

    pub fn new(token_mint: Pubkey, project_account: Pubkey, dev_account: Pubkey, bump: u8) -> Self {
        let clock = Clock::get().unwrap();
        Self {
            token_mint,
            start_key: Pubkey::default(),
            limit: 100,
            reflection_rate: 1000, // 10%
            burn_rate: 200, // 2%
            project_rate: 200, // 2%
            project_account,
            dev_fee_rate: 20, // 0.2%
            dev_account,
            created_at: clock.unix_timestamp,
            updated_at: clock.unix_timestamp,
            bump,
        }
    }

    pub fn validate_rates(&self) -> Result<()> {
        require!(self.reflection_rate <= 10000, crate::errors::SolFlexError::InvalidParameters);
        require!(self.burn_rate <= 10000, crate::errors::SolFlexError::InvalidParameters);
        require!(self.project_rate <= 10000, crate::errors::SolFlexError::InvalidParameters);
        require!(self.dev_fee_rate <= 10000, crate::errors::SolFlexError::InvalidParameters);

        let total_rate = self.reflection_rate + self.burn_rate + self.project_rate + self.dev_fee_rate;
        require!(total_rate <= 10000, crate::errors::SolFlexError::InvalidParameters);

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct FeePool {
    pub token_mint: Pubkey,
    pub reflection_pool: u64, // Accumulated reflection fees
    pub burn_pool: u64, // Accumulated burn fees
    pub project_pool: u64, // Accumulated project fees
    pub dev_pool: u64, // Accumulated dev fees
    pub total_fees: u64, // Total accumulated fees
    pub created_at: i64,
    pub updated_at: i64,
    pub bump: u8, // Store PDA bump seed for efficiency
}

impl FeePool {
    pub const SEED_PREFIX: &'static [u8] = crate::constants::FEE_POOL_SEED;

    pub fn new(token_mint: Pubkey, bump: u8) -> Self {
        let clock = Clock::get().unwrap();
        Self {
            token_mint,
            reflection_pool: 0,
            burn_pool: 0,
            project_pool: 0,
            dev_pool: 0,
            total_fees: 0,
            created_at: clock.unix_timestamp,
            updated_at: clock.unix_timestamp,
            bump,
        }
    }

    pub fn add_fees(&mut self, amount: u64, config: &DistributionConfig) {
        let reflection_fee = (amount * config.reflection_rate as u64) / 10000;
        let burn_fee = (amount * config.burn_rate as u64) / 10000;
        let project_fee = (amount * config.project_rate as u64) / 10000;
        let dev_fee = (amount * config.dev_fee_rate as u64) / 10000;

        self.reflection_pool += reflection_fee;
        self.burn_pool += burn_fee;
        self.project_pool += project_fee;
        self.dev_pool += dev_fee;
        self.total_fees += reflection_fee + burn_fee + project_fee + dev_fee;

        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    pub fn distribute_reflection(&mut self, amount: u64) -> Result<()> {
        require!(self.reflection_pool >= amount, crate::errors::SolFlexError::InsufficientFunds);
        self.reflection_pool -= amount;
        self.updated_at = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }

    pub fn clear_pools(&mut self) {
        self.burn_pool = 0;
        self.project_pool = 0;
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }
}

// Position tracking for Jupiter swaps
#[account]
#[derive(InitSpace)]
pub struct Position {
    pub owner: Pubkey,
    pub pool_id: u64,
    pub amount_invested: u64,
    pub current_value: u64,
    pub entry_price: u64, // Price at entry (for tracking)
    pub created_at: i64,
    pub updated_at: i64,
}

impl Position {
    pub const SEED_PREFIX: &'static [u8] = crate::constants::POSITION_SEED;

    pub fn new(owner: Pubkey, pool_id: u64, amount_invested: u64) -> Self {
        let clock = Clock::get().unwrap();
        Self {
            owner,
            pool_id,
            amount_invested,
            current_value: amount_invested,
            entry_price: 0, // To be set by external price feed
            created_at: clock.unix_timestamp,
            updated_at: clock.unix_timestamp,
        }
    }
}