use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DistributionConfig {
    pub token_mint: Pubkey, // The base token mint for fees
    pub fee_vault: Pubkey, // Program-owned token vault that receives fee deposits
    pub reflection_pool: u64, // Accounting mirror for reflection amount held in fee_vault
    pub dev_pool: u64, // Accounting mirror for dev amount held in fee_vault
    pub total_fees: u64, // Total recorded incoming fees
    pub last_paid: Pubkey, // Cursor: last user paid in ordered traversal
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

    pub fn new(
        token_mint: Pubkey,
        fee_vault: Pubkey,
        project_account: Pubkey,
        dev_account: Pubkey,
        bump: u8,
    ) -> Self {
        let clock = Clock::get().unwrap();
        Self {
            token_mint,
            fee_vault,
            reflection_pool: 0,
            dev_pool: 0,
            total_fees: 0,
            last_paid: Pubkey::default(),
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

    pub fn validate_reflection_dev_only_split(&self) -> Result<()> {
        require!(self.burn_rate == 0, crate::errors::SolFlexError::InvalidConfig);
        require!(self.project_rate == 0, crate::errors::SolFlexError::InvalidConfig);
        require!(
            self.reflection_rate + self.dev_fee_rate == 10000,
            crate::errors::SolFlexError::InvalidConfig
        );
        Ok(())
    }

    pub fn distribute_reflection(&mut self, amount: u64) -> Result<()> {
        require!(self.reflection_pool >= amount, crate::errors::SolFlexError::InsufficientFunds);
        self.reflection_pool -= amount;
        self.updated_at = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }

    pub fn distribute_dev(&mut self, amount: u64) -> Result<()> {
        require!(self.dev_pool >= amount, crate::errors::SolFlexError::InsufficientFunds);
        self.dev_pool -= amount;
        self.updated_at = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }
}