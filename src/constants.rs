// Program constants and seeds

// Seeds for PDAs
pub const CONFIG_SEED: &[u8] = b"config";
pub const BLOCKLIST_SEED: &[u8] = b"blocklist";

// Maximum blocklist size
pub const MAX_BLOCKLIST_SIZE: usize = 1000;

// Authority pubkey (to be set during initialization)
pub const DEFAULT_AUTHORITY: &str = "YourAuthorityPubkeyHere111111111111111111111111";

// Program version
pub const PROGRAM_VERSION: u8 = 1;