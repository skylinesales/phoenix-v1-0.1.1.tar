# Java Runtime Upgrade Guide

This repository now includes tools to upgrade the Java runtime to the latest LTS (Long-Term Support) version.

## Current Status
- **Target Version**: Java 21 (Latest LTS as of 2025)
- **Version Manager**: SDKMAN

## Quick Start

### Option 1: Automatic Upgrade using SDKMAN (Recommended)

Run the upgrade script which will install SDKMAN and Java 21:

```bash
./upgrade-java.sh
```

After running the script, activate Java 21 in your current shell:

```bash
source $HOME/.sdkman/bin/sdkman-init.sh
sdk use java 21.0.5-tem
```

### Option 2: APT-based Upgrade (Debian/Ubuntu)

For systems using APT package manager:

```bash
./upgrade-java-apt.sh
```

This will install Java 21 from the Eclipse Temurin repository and set it as the default.

### Option 3: Manual Installation with SDKMAN

1. Install SDKMAN (if not already installed):
   ```bash
   ./install-sdkman.sh
   source $HOME/.sdkman/bin/sdkman-init.sh
   ```

2. Install Java 21:
   ```bash
   sdk install java 21.0.5-tem
   ```

3. Set Java 21 as default:
   ```bash
   sdk default java 21.0.5-tem
   ```

## Verification

Check your Java version:

```bash
java -version
```

Expected output:
```
openjdk version "21.0.5" 2024-10-15 LTS
OpenJDK Runtime Environment Temurin-21.0.5+11 (build 21.0.5+11-LTS)
OpenJDK 64-Bit Server VM Temurin-21.0.5+11 (build 21.0.5+11-LTS, mixed mode, sharing)
```

## Java Version File

The repository includes a `.java-version` file that specifies the required Java version (21). This file is recognized by various Java version managers including:
- SDKMAN
- jenv
- asdf

## About SDKMAN

SDKMAN (Software Development Kit Manager) is a tool for managing parallel versions of multiple SDKs on Unix-based systems. It makes it easy to:
- Install multiple Java versions
- Switch between Java versions
- Set default Java versions
- Manage other SDK tools

For more information, visit: https://sdkman.io/

## Troubleshooting

### SDKMAN not found in new shell

After installation, you need to source SDKMAN in each new shell session. Add this to your `~/.bashrc` or `~/.zshrc`:

```bash
export SDKMAN_DIR="$HOME/.sdkman"
[[ -s "$HOME/.sdkman/bin/sdkman-init.sh" ]] && source "$HOME/.sdkman/bin/sdkman-init.sh"
```

### Permission denied when running scripts

Make sure the scripts are executable:

```bash
chmod +x install-sdkman.sh upgrade-java.sh
```

## Alternative Java Version Managers

While this guide uses SDKMAN, you can also use other tools:
- **jenv**: Lightweight Java version manager
- **asdf**: Universal version manager with Java plugin
- **Jabba**: Cross-platform Java version manager
