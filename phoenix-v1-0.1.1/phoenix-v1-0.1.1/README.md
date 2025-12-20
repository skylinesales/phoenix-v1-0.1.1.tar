# phoenix

On-chain orderbook that operates without a crank

## Features

- **Atomic Settlement**: Trades settle instantly on-chain
- **No Crank Required**: Self-executing orderbook
- **CPI Support**: Integrate Phoenix into your Solana programs
- **Multiple Order Types**: IOC, Post-Only, Fill-or-Kill

## Using Phoenix in Your Solana Program

Phoenix supports Cross-Program Invocation (CPI), allowing you to integrate DEX functionality directly into your Solana programs.

### Quick Start

Add Phoenix to your `Cargo.toml`:

```toml
[dependencies]
phoenix = { version = "0.1.1", features = ["cpi", "no-entrypoint"] }
```

Example usage:

```rust
use phoenix::program::cpi::swap_signed;
use phoenix::state::{OrderPacket, Side};
use phoenix::quantities::WrapperU64;

// Execute a swap via CPI
let order_packet = OrderPacket::new_ioc_by_lots(
    Side::Bid,
    WrapperU64::new(1000),  // price
    10,                      // quantity
    WrapperU64::new(u64::MAX),
    None,
    false,
);

swap_signed(
    phoenix_program.key,
    market,
    trader,
    base_account,
    quote_account,
    base_vault,
    quote_vault,
    token_program,
    &order_packet,
    signer_seeds,
)?;
```

**For complete documentation**, see [CPI_INTEGRATION.md](CPI_INTEGRATION.md).

## Development

### Build

To build the contract, run:

```bash
./build.sh
```

### Testing

To test the contract, run:

```bash
./test.sh
```

## Resources

- [CPI Integration Guide](CPI_INTEGRATION.md)
- [Example Program](examples/cpi_example.rs)
- [Phoenix SDK](https://github.com/Ellipsis-Labs/phoenix-sdk)