#!/bin/bash -e

# Script to upgrade Java to the latest LTS version (Java 21)
# This script uses SDKMAN for Java version management

echo "Java Runtime Upgrade Script"
echo "============================"
echo ""

# Check if SDKMAN is installed
if [ ! -d "$HOME/.sdkman" ]; then
    echo "SDKMAN is not installed. Installing SDKMAN first..."
    curl -s "https://get.sdkman.io" | bash
    source "$HOME/.sdkman/bin/sdkman-init.sh"
else
    echo "SDKMAN is already installed."
    source "$HOME/.sdkman/bin/sdkman-init.sh"
fi

echo ""
echo "Current Java version:"
java -version 2>&1 || echo "Java not found"

echo ""
echo "Installing Java 21 (Latest LTS)..."

# Install Java 21 (Temurin distribution - formerly AdoptOpenJDK)
sdk install java 21.0.5-tem || echo "Java 21 may already be installed"

# Set Java 21 as default
sdk default java 21.0.5-tem

echo ""
echo "Java upgrade complete!"
echo ""
echo "New Java version:"
java -version 2>&1

echo ""
echo "To use Java 21 in your current shell, run:"
echo "  source \$HOME/.sdkman/bin/sdkman-init.sh"
echo "  sdk use java 21.0.5-tem"
