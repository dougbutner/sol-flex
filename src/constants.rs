// Program constants and seeds

// Seeds for PDAs
pub const CONFIG_SEED: &[u8] = b"config";
pub const BLOCKLIST_SEED: &[u8] = b"blocklist";
pub const TOKEN_ACCOUNT_SEED: &[u8] = b"token_account";
pub const TOKEN_POOL_SEED: &[u8] = b"token_pool";
pub const USER_PREFERENCES_SEED: &[u8] = b"user_preferences";
pub const GLOBAL_POOLS_SEED: &[u8] = b"global_pools";
pub const DISTRIBUTION_CONFIG_SEED: &[u8] = b"distribution_config";
pub const FEE_POOL_SEED: &[u8] = b"fee_pool";
pub const POSITION_SEED: &[u8] = b"position";

// Maximum sizes
pub const MAX_BLOCKLIST_SIZE: usize = 1000;
pub const MAX_MEMO_LENGTH: usize = 200;

// Authority pubkey (to be set during initialization)
pub const DEFAULT_AUTHORITY: &str = "YourAuthorityPubkeyHere111111111111111111111111";

// Program version
pub const PROGRAM_VERSION: u8 = 1;