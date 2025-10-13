#!/bin/bash -e

# Script to install SDKMAN (Software Development Kit Manager)
# SDKMAN is a popular tool for managing parallel versions of multiple SDKs including Java

echo "Installing SDKMAN..."

# Install SDKMAN
curl -s "https://get.sdkman.io" | bash

# Source SDKMAN to make it available in current shell
source "$HOME/.sdkman/bin/sdkman-init.sh"

echo "SDKMAN installed successfully!"
echo "Please run 'source \$HOME/.sdkman/bin/sdkman-init.sh' to use SDKMAN in your current shell"
