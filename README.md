# Phoenix v1

On-chain orderbook that operates without a crank - a Solana-based decentralized exchange.

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
