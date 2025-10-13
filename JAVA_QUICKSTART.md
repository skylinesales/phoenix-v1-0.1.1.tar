# Java Upgrade - Quick Reference Card

## Current Status
- **Current Java Version**: 17
- **Target Java Version**: 21 (Latest LTS)
- **Upgrade Required**: Yes

## Quick Commands

### Check Current Version
```bash
java -version
```

### Upgrade Java (Recommended Method)
```bash
./upgrade-java.sh
```

After upgrade, activate in current shell:
```bash
source $HOME/.sdkman/bin/sdkman-init.sh
sdk use java 21.0.5-tem
```

### Upgrade Java (APT Method - Debian/Ubuntu)
```bash
./upgrade-java-apt.sh
```

### Test Upgrade Tools
```bash
./test-java-upgrade-tools.sh
```

## Files Added

| File | Purpose |
|------|---------|
| `.java-version` | Specifies required Java version (21) |
| `upgrade-java.sh` | Main upgrade script using SDKMAN |
| `upgrade-java-apt.sh` | Alternative upgrade using APT package manager |
| `install-sdkman.sh` | Standalone SDKMAN installer |
| `test-java-upgrade-tools.sh` | Test suite for upgrade tools |
| `JAVA_UPGRADE.md` | Comprehensive upgrade documentation |
| `README.md` | Updated project README with Java requirements |
| `.github/workflows/java-version-check.yml` | CI/CD workflow for Java version verification |

## What Was Changed

1. **Added Java version specification** - `.java-version` file marks Java 21 as required
2. **Automated upgrade scripts** - Two methods (SDKMAN and APT) for different preferences
3. **Documentation** - Complete guides for upgrading and managing Java versions
4. **CI/CD integration** - GitHub Actions workflow to verify Java version in automated builds
5. **Testing** - Comprehensive test suite to validate all upgrade tools

## Next Steps

1. Run `./test-java-upgrade-tools.sh` to verify tools are ready
2. Run `./upgrade-java.sh` to upgrade to Java 21
3. Verify upgrade with `java -version`
4. Continue with your development work

## Support

For detailed instructions, see [JAVA_UPGRADE.md](JAVA_UPGRADE.md)

For troubleshooting, refer to the "Troubleshooting" section in JAVA_UPGRADE.md
