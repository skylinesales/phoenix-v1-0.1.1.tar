# phoenix

On-chain orderbook that operates without a crank

### Build

To build the contract, run:

```bash
./build.sh
```

The build script supports both modern (`build-sbf`) and legacy (`build-bpf`) Solana build commands:

```bash
# Default: use cargo build-sbf (recommended)
./build.sh

# Legacy: use cargo build-bpf (for older Anchor versions <0.26)
./build.sh --bpf

# Or set via environment variable
SOLANA_BPF_BUILD=true ./build.sh
```

For more options, run:
```bash
./build.sh --help
```

### Testing

To test the contract, run:

```bash
./test.sh
```