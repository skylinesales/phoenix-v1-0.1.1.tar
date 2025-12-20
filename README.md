# Phoenix v1

On-chain orderbook that operates without a crank - a Solana-based decentralized exchange.

## Features

- **Atomic Settlement**: Trades settle instantly on-chain
- **No Crank Required**: Self-executing orderbook
- **CPI Integration**: Use Phoenix in your Solana programs via Cross-Program Invocation
- **Multiple Order Types**: IOC, Post-Only, Fill-or-Kill
- **Event Logging**: Complete transaction history via event logs

## Integration with Solana Programs

Phoenix supports **Cross-Program Invocation (CPI)**, enabling seamless integration into other Solana programs. See the [CPI Integration Guide](phoenix-v1-0.1.1/phoenix-v1-0.1.1/CPI_INTEGRATION.md) for detailed instructions.

### Quick Example

```rust
use phoenix::program::cpi::swap_signed;
use phoenix::state::{OrderPacket, Side};

// Execute a swap from your Solana program
swap_signed(
    phoenix_program_id,
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

## Prerequisites

### Java Runtime
This project requires **Java 21 LTS** or later.

**Quick Check:**
```bash
java -version
```

**Upgrade to Java 21:**
If you need to upgrade, see [JAVA_UPGRADE.md](JAVA_UPGRADE.md) for detailed instructions.

Quick upgrade:
```bash
./upgrade-java.sh
```

### Other Requirements
- Rust and Cargo (for building the Solana program)
- Node.js and Yarn (for IDL generation)
- Solana CLI tools

## Project Structure

- `phoenix-v1-0.1.1/phoenix-v1-0.1.1/` - Main Phoenix smart contract project
  - Rust-based Solana program
  - IDL generation scripts
  - Test suite

## Build

To build the contract, run:

```bash
cd phoenix-v1-0.1.1/phoenix-v1-0.1.1
./build.sh
```

## Testing

To test the contract, run:

```bash
cd phoenix-v1-0.1.1/phoenix-v1-0.1.1
./test.sh
```

## Java Version Management

This repository includes tools for managing Java versions:

- `.java-version` - Specifies the required Java version (21)
- `upgrade-java.sh` - Automated script to upgrade to Java 21 using SDKMAN
- `install-sdkman.sh` - Install SDKMAN (Software Development Kit Manager)

For complete Java upgrade instructions, see [JAVA_UPGRADE.md](JAVA_UPGRADE.md).

## License

See [LICENSE](phoenix-v1-0.1.1/phoenix-v1-0.1.1/LICENSE) for details.

## Additional Resources

- [Phoenix Documentation](https://github.com/Ellipsis-Labs/phoenix-v1)
- [Java Upgrade Guide](JAVA_UPGRADE.md)
