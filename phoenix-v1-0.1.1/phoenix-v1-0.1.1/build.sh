#!/bin/bash -e

# Parse command line arguments
USE_BPF=false

while [[ $# -gt 0 ]]; do
  case $1 in
    --bpf|--build-bpf)
      USE_BPF=true
      shift
      ;;
    --sbf|--build-sbf)
      USE_BPF=false
      shift
      ;;
    -h|--help)
      echo "Usage: $0 [OPTIONS]"
      echo ""
      echo "Build the Phoenix Solana program"
      echo ""
      echo "Options:"
      echo "  --bpf, --build-bpf    Use cargo build-bpf (older, for Anchor <0.26)"
      echo "  --sbf, --build-sbf    Use cargo build-sbf (newer, default)"
      echo "  -h, --help            Show this help message"
      echo ""
      echo "Environment Variables:"
      echo "  SOLANA_BPF_BUILD      Set to 'true' to use build-bpf"
      exit 0
      ;;
    *)
      echo "Unknown option: $1"
      echo "Usage: $0 [--bpf|--build-bpf] [--sbf|--build-sbf]"
      echo "Use --help for more information"
      exit 1
      ;;
  esac
done

ROOT=$(git rev-parse --show-toplevel)

# Choose build command based on flag or environment variable
if [ "${SOLANA_BPF_BUILD:-false}" = "true" ] || [ "$USE_BPF" = "true" ]; then
  CARGO_BUILD_CMD="cargo build-bpf"
  echo "Using cargo build-bpf (legacy mode)"
else
  CARGO_BUILD_CMD="cargo build-sbf"
  echo "Using cargo build-sbf (default)"
fi

# Build the Solana program
echo "Building with: $CARGO_BUILD_CMD"
$CARGO_BUILD_CMD

# Generate IDL
(cd $ROOT/idl && yarn && node generateIdl.js)
