# Sol Flex 👨‍💻

> A Solana program for flexible reflection operations with blocklist management and Jupiter swap integration.

---

## Features 🌟

### Core Program Management
- **Initialize**: Set up the program configuration with authority
- **Update Config**: Modify program settings (authority, reflection parameters)
- **Set Distribution Config**: Configure fee distribution rates and accounts

### Blocklist & User Management
- **Blocklist Management**: Add/remove accounts from blocklist
- **Ban User**: Ban/unban specific users from reflections
- **Set User Preferences**: Configure user reflection preferences and Jupiter swap pools

### Pool Management (Jupiter Swaps)
- **Add Pool**: Add new Jupiter swap pool configurations
- **Remove Pool**: Remove Jupiter swap pool configurations

### Reflection System
- **Reflect**: Main reflection distribution logic with blocklist checking and Jupiter swap integration

---

## Jupiter Swap Integration 🔄

The program maintains a registry of available Jupiter swap pools that users can select for automatic token swaps during reflections. Users store only a pool ID (efficient u64) instead of full pool addresses, enabling fast lookups and updates by the authority.

---

## Program Structure 🏗️

```
src/
├── lib.rs                 # Entry point with program declaration
├── constants.rs           # Program constants and seeds
├── errors.rs              # Custom error definitions
├── state/
│   ├── config.rs          # Config account structure
│   ├── token.rs           # Token, user preferences, and pool structures
│   └── distribution.rs    # Fee distribution and position tracking
└── instructions/
    ├── initialize.rs      # Initialize instruction
    ├── update_config.rs   # Update configuration
    ├── add_to_blocklist.rs    # Add to blocklist
    ├── remove_from_blocklist.rs # Remove from blocklist
    ├── reflect.rs         # Main reflection logic
    ├── set_user_preferences.rs # User preference management
    ├── ban_user.rs        # User banning/unbanning
    ├── manage_pool.rs     # Jupiter pool management
    └── set_distribution_config.rs # Fee distribution config
```

---

## Prerequisites 📋

- **Rust**: Latest stable version
- **Solana CLI**: `solana --version`
- **Anchor**: `anchor --version`
- **Node.js**: For testing and deployment scripts

---

## Program ID 🔑

```
5im5SdEc2dg63B5C9vm83mwQqxGUAphG2K47uGgA69ZS
```

---

## Development Workflow 🚀

### Building
```bash
anchor build
```

### Testing
```bash
anchor test
```

### Deployment
```bash
anchor deploy
```

---

## Dependencies 📦

- `anchor-lang = "0.29.0"` - Anchor framework for Solana programs
- `anchor-spl = "0.29.0"` - Anchor SPL integration
- `spl-token = "4.0"` - SPL Token program
- `spl-associated-token-account = "2.3"` - Associated token account utilities

---

## Quick Start ⚡

1. **Install prerequisites**
2. **Clone the repository**
3. **Build the program**: `anchor build`
4. **Run tests**: `anchor test`
5. **Deploy to devnet**: `anchor deploy`

---

## Architecture Overview 🏗️

This Solana program implements a flexible reflection system with the following key components:

- **Config Account**: Stores program-wide settings and authority
- **Token State**: Manages user preferences and Jupiter pool configurations
- **Distribution State**: Handles fee distribution logic and tracking
- **Blocklist**: Prevents specified accounts from receiving reflections
- **Jupiter Integration**: Enables automated token swaps during reflection distribution

---

## Constants & Limits 📊

### PDA Seeds
- `CONFIG_SEED`: Program configuration account
- `BLOCKLIST_SEED`: Blocklist storage
- `TOKEN_ACCOUNT_SEED`: Token account management
- `TOKEN_POOL_SEED`: Jupiter pool configurations
- `USER_PREFERENCES_SEED`: User reflection settings
- `GLOBAL_POOLS_SEED`: Global pool registry
- `DISTRIBUTION_CONFIG_SEED`: Fee distribution settings
- `FEE_POOL_SEED`: Fee collection pool
- `POSITION_SEED`: User position tracking

### Limits
- **Max Blocklist Size**: 1000 accounts
- **Max Memo Length**: 200 characters
- **Program Version**: 1

---

## Error Codes 🚨

The program defines the following custom error codes:

- `Unauthorized`: Access denied
- `InvalidConfig`: Configuration validation failed
- `AlreadyInBlocklist`/`NotInBlocklist`: Blocklist state errors
- `BlocklistFull`: Blocklist capacity exceeded
- `InvalidParameters`: Input validation failed
- `ArithmeticOverflow`: Mathematical operation overflow
- `PoolAlreadyExists`/`PoolNotFound`: Pool management errors
- `InsufficientFunds`: Balance validation failed
- `NoReflectionsToDistribute`: Distribution logic error
- `InvalidMemoLength`: Memo size validation failed
- `AccountNotFound`: Account lookup failed

---

## License 📄

No licence you aren't permitted to use this.

---

> Built with ❤️ using FRESH syntax by [Douglas Butner](https://github.com/dougbutner/FRESH)