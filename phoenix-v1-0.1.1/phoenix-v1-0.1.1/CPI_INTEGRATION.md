# Phoenix CPI Integration Guide

This guide explains how to integrate Phoenix DEX into your Solana programs using Cross-Program Invocation (CPI).

## Table of Contents
- [Overview](#overview)
- [Setup](#setup)
- [Basic Usage](#basic-usage)
- [Available CPI Functions](#available-cpi-functions)
- [Complete Example](#complete-example)
- [Common Patterns](#common-patterns)
- [Troubleshooting](#troubleshooting)

## Overview

Phoenix is an on-chain limit order book DEX on Solana. With CPI support, your Solana programs can execute trades, place orders, and interact with Phoenix markets directly.

### Benefits of Phoenix CPI
- **Atomic Execution**: Trades settle instantly on-chain
- **No Crank Required**: Self-executing orderbook
- **Deep Integration**: Compose Phoenix with your program logic
- **Composability**: Build complex DeFi strategies

## Setup

### 1. Add Phoenix Dependency

Add Phoenix to your program's `Cargo.toml` with the `cpi` feature:

```toml
[dependencies]
phoenix = { version = "0.1.1", features = ["cpi", "no-entrypoint"] }
solana-program = "1.14.9"
```

### 2. Feature Flags

The `cpi` feature enables the CPI helper module and includes the `no-entrypoint` feature automatically. This allows Phoenix to be used as a library in your program.

## Basic Usage

### Importing Phoenix CPI Module

```rust
use phoenix::program::cpi::{swap, swap_signed, place_limit_order};
use phoenix::state::{OrderPacket, Side};
use phoenix::quantities::WrapperU64;
```

### Creating an Order Packet

Order packets define the trade parameters:

```rust
// Create a swap order (IOC - Immediate or Cancel)
let order_packet = OrderPacket::new_ioc_by_lots(
    Side::Bid,                      // Buy or sell
    WrapperU64::new(1000),          // Price in ticks
    10,                             // Number of base lots
    WrapperU64::new(u64::MAX),      // Self-trade behavior
    None,                           // Match limit
    false,                          // Use only deposited funds
);

// Create a limit order (Post-only)
let order_packet = OrderPacket::new_post_only_default(
    Side::Ask,
    WrapperU64::new(2000),
    20,
    WrapperU64::new(12345),         // Client order ID
);
```

### Executing a Swap via CPI

```rust
use phoenix::program::cpi::swap;

// Prepare account infos
// market_info, trader_info, base_account_info, etc.

// Execute the swap
swap(
    phoenix_program.key,
    market_info,
    trader_info,
    base_account_info,
    quote_account_info,
    base_vault_info,
    quote_vault_info,
    token_program_info,
    &order_packet,
)?;
```

## Available CPI Functions

### swap

Execute an immediate swap on Phoenix DEX.

```rust
pub fn swap<'a>(
    program_id: &Pubkey,
    market: &AccountInfo<'a>,
    trader: &AccountInfo<'a>,
    base_account: &AccountInfo<'a>,
    quote_account: &AccountInfo<'a>,
    base_vault: &AccountInfo<'a>,
    quote_vault: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    order_packet: &OrderPacket,
) -> ProgramResult
```

**Use Case**: Direct swaps where the trader is a regular signer.

### swap_signed

Execute a swap with PDA signing.

```rust
pub fn swap_signed<'a>(
    program_id: &Pubkey,
    market: &AccountInfo<'a>,
    trader: &AccountInfo<'a>,
    base_account: &AccountInfo<'a>,
    quote_account: &AccountInfo<'a>,
    base_vault: &AccountInfo<'a>,
    quote_vault: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    order_packet: &OrderPacket,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult
```

**Use Case**: When your program's PDA needs to execute trades.

### place_limit_order

Place a limit order on the orderbook.

```rust
pub fn place_limit_order<'a>(
    program_id: &Pubkey,
    market: &AccountInfo<'a>,
    trader: &AccountInfo<'a>,
    seat: &AccountInfo<'a>,
    base_account: &AccountInfo<'a>,
    quote_account: &AccountInfo<'a>,
    base_vault: &AccountInfo<'a>,
    quote_vault: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    order_packet: &OrderPacket,
) -> ProgramResult
```

**Use Case**: Placing resting orders that stay on the book. Requires a seat.

### place_limit_order_signed

Place a limit order with PDA signing.

```rust
pub fn place_limit_order_signed<'a>(
    program_id: &Pubkey,
    market: &AccountInfo<'a>,
    trader: &AccountInfo<'a>,
    seat: &AccountInfo<'a>,
    base_account: &AccountInfo<'a>,
    quote_account: &AccountInfo<'a>,
    base_vault: &AccountInfo<'a>,
    quote_vault: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    order_packet: &OrderPacket,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult
```

**Use Case**: PDA-based limit orders. Requires a seat.

### Helper Functions

```rust
// Get vault PDA for a market and mint
pub fn get_vault_address_helper(market: &Pubkey, mint: &Pubkey) -> (Pubkey, u8)

// Get seat PDA for a market and trader
pub fn get_seat_address_helper(market: &Pubkey, trader: &Pubkey) -> (Pubkey, u8)
```

## Complete Example

See [examples/cpi_example.rs](../examples/cpi_example.rs) for a full working example.

Here's a simplified version:

```rust
use phoenix::program::cpi::swap_signed;
use phoenix::state::{OrderPacket, Side};
use phoenix::quantities::WrapperU64;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

fn execute_trade(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    // Assume accounts are validated and extracted
    let authority = &accounts[0];
    let market = &accounts[1];
    let base_account = &accounts[2];
    let quote_account = &accounts[3];
    let base_vault = &accounts[4];
    let quote_vault = &accounts[5];
    let token_program = &accounts[6];
    let phoenix_program = &accounts[7];

    // Get PDA bump
    let (_, bump) = Pubkey::find_program_address(
        &[b"authority"],
        program_id,
    );

    // Create order
    let order_packet = OrderPacket::new_ioc_by_lots(
        Side::Bid,
        WrapperU64::new(1000),
        10,
        WrapperU64::new(u64::MAX),
        None,
        false,
    );

    // Execute via CPI
    let seeds = &[b"authority".as_ref(), &[bump]];
    swap_signed(
        phoenix_program.key,
        market,
        authority,
        base_account,
        quote_account,
        base_vault,
        quote_vault,
        token_program,
        &order_packet,
        &[seeds],
    )
}
```

## Common Patterns

### 1. PDA-Controlled Trading

Use PDAs to control trading on behalf of users:

```rust
// Derive your program's trading authority
let (authority, bump) = Pubkey::find_program_address(
    &[b"trading_authority"],
    program_id,
);

// Use swap_signed with the authority's seeds
let seeds = &[b"trading_authority".as_ref(), &[bump]];
swap_signed(
    phoenix_program_id,
    market,
    authority_account,
    // ... other accounts
    &order_packet,
    &[seeds],
)?;
```

### 2. Conditional Trading

Execute trades based on program logic:

```rust
// Check condition
if price_meets_threshold(current_price, threshold) {
    // Execute trade
    swap(/* ... */)?;
}
```

### 3. Multi-Market Strategies

Trade across multiple Phoenix markets:

```rust
// Market 1: SOL/USDC
swap(phoenix_program, market1, /* ... */)?;

// Market 2: BTC/USDC
swap(phoenix_program, market2, /* ... */)?;
```

### 4. Getting Market Addresses

Calculate PDAs for market accounts:

```rust
use phoenix::program::cpi::{get_vault_address_helper, get_seat_address_helper};

// Get vault addresses
let (base_vault, _) = get_vault_address_helper(&market_key, &base_mint);
let (quote_vault, _) = get_vault_address_helper(&market_key, &quote_mint);

// Get seat address (for limit orders)
let (seat, _) = get_seat_address_helper(&market_key, &trader_key);
```

## Account Layout

### For Swap Instructions

Required accounts in order:
1. `[]` Phoenix program
2. `[]` Log authority (Phoenix PDA)
3. `[writable]` Market
4. `[signer]` Trader
5. `[writable]` Trader's base token account
6. `[writable]` Trader's quote token account
7. `[writable]` Market's base vault
8. `[writable]` Market's quote vault
9. `[]` SPL Token program

### For Limit Order Instructions

Required accounts in order:
1. `[]` Phoenix program
2. `[]` Log authority (Phoenix PDA)
3. `[writable]` Market
4. `[signer]` Trader
5. `[]` Trader's seat
6. `[writable]` Trader's base token account
7. `[writable]` Trader's quote token account
8. `[writable]` Market's base vault
9. `[writable]` Market's quote vault
10. `[]` SPL Token program

## Order Types

### IOC (Immediate or Cancel)

Executes immediately and cancels any unfilled portion:

```rust
OrderPacket::new_ioc_by_lots(
    Side::Bid,
    price_in_ticks,
    num_base_lots,
    self_trade_behavior,
    match_limit,
    use_only_deposited_funds,
)
```

### Post-Only

Only places an order on the book, won't match existing orders:

```rust
OrderPacket::new_post_only_default(
    Side::Ask,
    price_in_ticks,
    num_base_lots,
    client_order_id,
)
```

### Fill or Kill (FOK)

Must fill completely or be canceled:

```rust
OrderPacket::new_fok_by_lots(
    Side::Bid,
    price_in_ticks,
    num_base_lots,
    self_trade_behavior,
    match_limit,
    use_only_deposited_funds,
)
```

## Troubleshooting

### Common Errors

**Error: Missing Required Signature**
- Ensure the trader account is marked as a signer
- For PDA traders, use `*_signed` functions with correct seeds

**Error: Invalid Account Data**
- Verify account addresses match expected PDAs
- Check vault addresses are derived correctly
- Ensure seat exists for limit orders

**Error: Insufficient Funds**
- Verify token accounts have sufficient balance
- Check token account ownership
- Ensure proper token account derivation

### Debugging Tips

1. **Log Account Keys**: Print account keys to verify correct accounts
   ```rust
   msg!("Market: {}", market.key);
   msg!("Trader: {}", trader.key);
   ```

2. **Verify PDAs**: Double-check PDA derivation
   ```rust
   let (vault, bump) = get_vault_address_helper(&market, &mint);
   msg!("Expected vault: {}, bump: {}", vault, bump);
   ```

3. **Check Feature Flags**: Ensure `cpi` feature is enabled
   ```toml
   phoenix = { version = "0.1.1", features = ["cpi", "no-entrypoint"] }
   ```

## Best Practices

1. **Error Handling**: Always handle CPI errors appropriately
2. **Account Validation**: Validate all accounts before CPI calls
3. **PDA Verification**: Verify PDAs match expected addresses
4. **Gas Optimization**: Minimize CPI calls when possible
5. **Testing**: Thoroughly test CPI integration in devnet

## Additional Resources

- [Phoenix Program Documentation](https://docs.phoenix.trade)
- [Solana CPI Documentation](https://docs.solana.com/developing/programming-model/calling-between-programs)
- [Example Program](../examples/cpi_example.rs)
- [Phoenix SDK](https://github.com/Ellipsis-Labs/phoenix-sdk)

## Support

For issues or questions:
- GitHub Issues: [Phoenix V1 Repository](https://github.com/Ellipsis-Labs/phoenix-v1)
- Discord: [Ellipsis Labs Community](https://discord.gg/ellipsis-labs)
- Email: maintainers@ellipsislabs.xyz
