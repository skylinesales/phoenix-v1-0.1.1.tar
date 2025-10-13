#!/bin/bash -e

# Alternative Java upgrade script using APT package manager (for Debian/Ubuntu systems)
# This script upgrades Java to version 21 using Eclipse Temurin packages

echo "Java Runtime Upgrade Script (APT-based)"
echo "========================================"
echo ""

echo "Current Java version:"
java -version 2>&1 || echo "Java not found"

echo ""
echo "Adding Eclipse Temurin repository..."

# Install prerequisites
sudo apt-get update
sudo apt-get install -y wget apt-transport-https gnupg

# Add Temurin GPG key
wget -O - https://packages.adoptium.net/artifactory/api/gpg/key/public | sudo apt-key add -

# Add Temurin repository
echo "deb https://packages.adoptium.net/artifactory/deb $(awk -F= '/^VERSION_CODENAME/{print$2}' /etc/os-release) main" | sudo tee /etc/apt/sources.list.d/adoptium.list

# Update package list
sudo apt-get update

echo ""
echo "Installing Java 21 (Temurin)..."

# Install Java 21
sudo apt-get install -y temurin-21-jdk

echo ""
echo "Setting Java 21 as default..."

# Update alternatives to use Java 21
sudo update-alternatives --set java /usr/lib/jvm/temurin-21-jdk-amd64/bin/java
sudo update-alternatives --set javac /usr/lib/jvm/temurin-21-jdk-amd64/bin/javac

echo ""
echo "Java upgrade complete!"
echo ""
echo "New Java version:"
java -version 2>&1

echo ""
echo "Note: If you have JAVA_HOME set, update it to:"
echo "  export JAVA_HOME=/usr/lib/jvm/temurin-21-jdk-amd64"
