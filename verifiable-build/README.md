# Solana Verifiable Build CLI

A command-line tool for building and verifying Solana programs in a reproducible way using Docker.

## Features

- Build Solana programs from source in a reproducible Docker environment
- Verify that deployed programs match their source code
- Get hashes of deployed programs and buffer accounts
- Support for custom RPC endpoints with automatic network aliases
- Integration with Solana CLI configuration

## Installation

```bash
cd verifiable-build
cargo install --path .
```

## Usage

### Build a program

```bash
solana-verifiable-build build \
  --repo-url https://github.com/user/repo \
  --mount-path programs/my-program \
  --commit-hash abc123
```

### Verify a program from source

```bash
solana-verifiable-build verify-from-repo \
  --program-id <PROGRAM_ID> \
  --name-of-program my_program \
  --url mainnet
```

### Verify a program from Docker image

```bash
solana-verifiable-build verify-from-image \
  --executable-path target/deploy/program.so \
  --image my-build-image:latest \
  --program-id <PROGRAM_ID> \
  --url https://api.mainnet-beta.solana.com
```

### Get program hash

```bash
solana-verifiable-build get-program-hash \
  --program-id <PROGRAM_ID> \
  --url mainnet
```

### Get buffer hash

```bash
solana-verifiable-build get-buffer-hash \
  --buffer-address <BUFFER_ADDRESS> \
  --url devnet
```

## Network Aliases

The `--url` parameter supports convenient network aliases:

- `mainnet`, `main`, `m`, `mainnet-beta` → https://api.mainnet-beta.solana.com
- `devnet`, `dev`, `d` → https://api.devnet.solana.com
- `localnet`, `localhost`, `l`, `local` → http://localhost:8899

You can also provide a full RPC URL directly.

## Configuration

If no `--url` parameter is provided, the tool will use the RPC URL from your Solana CLI configuration file (`~/.config/solana/cli/config.yml`).

## Key Changes

This version includes important updates to URL handling:

- URL parameters are now optional (`Option<String>`) instead of required
- Defaults to user's Solana CLI configuration when no URL is specified
- Added `get_network()` helper function for convenient network aliases
- Added `get_client()` function that integrates with Solana CLI config

These changes make it easier to use the tool without repeatedly specifying URLs.
