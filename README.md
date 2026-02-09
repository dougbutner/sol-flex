# Sol Flex

A Solana program for flexible reflection operations with blocklist management.

## Features

- **Initialize**: Set up the program configuration
- **Update Config**: Modify program settings (authority, etc.)
- **Blocklist Management**: Add/remove accounts from blocklist
- **Reflect**: Main reflection logic with blocklist checking

## Program Structure

```
src/
├── lib.rs                 # Entry point with program declaration
├── constants.rs           # Program constants and seeds
├── errors.rs              # Custom error definitions
├── state/
│   └── config.rs          # Config account structure
└── instructions/
    ├── initialize.rs      # Initialize instruction
    ├── update_config.rs   # Update configuration
    ├── add_to_blocklist.rs # Add to blocklist
    ├── remove_from_blocklist.rs # Remove from blocklist
    └── reflect.rs         # Main reflection logic
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