# Sol Flex

A Solana program for flexible reflection operations with blocklist management.

## Features

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

### Jupiter Swap Integration
The program maintains a registry of available Jupiter swap pools that users can select for automatic token swaps during reflections. Users store only a pool ID (efficient u64) instead of full pool addresses, enabling fast lookups and updates by the authority.

## Program Structure

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
    ├── add_to_blocklist.rs # Add to blocklist
    ├── remove_from_blocklist.rs # Remove from blocklist
    ├── reflect.rs         # Main reflection logic
    ├── set_user_preferences.rs # User preference management
    ├── ban_user.rs        # User banning/unbanning
    ├── manage_pool.rs     # Jupiter pool management
    └── set_distribution_config.rs # Fee distribution config
```

## Building

```bash
anchor build
```

## Testing

```bash
anchor test
```

## Deployment

```bash
anchor deploy
```