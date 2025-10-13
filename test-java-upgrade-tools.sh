#!/bin/bash

# Test script to verify Java upgrade tools
# This script performs dry-run tests and validation

echo "========================================="
echo "Java Upgrade Tools - Test Suite"
echo "========================================="
echo ""

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counters
TESTS_PASSED=0
TESTS_FAILED=0

# Function to print test result
print_result() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓ PASS${NC}: $2"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}✗ FAIL${NC}: $2"
        ((TESTS_FAILED++))
    fi
}

echo "Test 1: Verify .java-version file exists"
if [ -f .java-version ]; then
    VERSION=$(cat .java-version)
    if [ "$VERSION" = "21" ]; then
        print_result 0 ".java-version contains correct version (21)"
    else
        print_result 1 ".java-version contains unexpected version ($VERSION)"
    fi
else
    print_result 1 ".java-version file not found"
fi

echo ""
echo "Test 2: Verify upgrade scripts exist and are executable"

scripts=("upgrade-java.sh" "upgrade-java-apt.sh" "install-sdkman.sh")
for script in "${scripts[@]}"; do
    if [ -f "$script" ]; then
        if [ -x "$script" ]; then
            print_result 0 "$script exists and is executable"
        else
            print_result 1 "$script exists but is not executable"
        fi
    else
        print_result 1 "$script not found"
    fi
done

echo ""
echo "Test 3: Verify script syntax"

for script in "${scripts[@]}"; do
    if bash -n "$script" 2>/dev/null; then
        print_result 0 "$script has valid syntax"
    else
        print_result 1 "$script has syntax errors"
    fi
done

echo ""
echo "Test 4: Verify documentation files"

docs=("README.md" "JAVA_UPGRADE.md")
for doc in "${docs[@]}"; do
    if [ -f "$doc" ]; then
        # Check if file is not empty
        if [ -s "$doc" ]; then
            print_result 0 "$doc exists and is not empty"
        else
            print_result 1 "$doc exists but is empty"
        fi
    else
        print_result 1 "$doc not found"
    fi
done

echo ""
echo "Test 5: Verify GitHub Actions workflow"

workflow=".github/workflows/java-version-check.yml"
if [ -f "$workflow" ]; then
    # Check if workflow contains required steps
    if grep -q "setup-java" "$workflow" && grep -q "java-version: '21'" "$workflow"; then
        print_result 0 "GitHub Actions workflow is properly configured"
    else
        print_result 1 "GitHub Actions workflow is missing required configuration"
    fi
else
    print_result 1 "GitHub Actions workflow not found"
fi

echo ""
echo "Test 6: Check current Java version"

if command -v java &> /dev/null; then
    # Use portable version extraction (compatible with macOS, Linux, BSD)
    CURRENT_JAVA_VERSION=$(java -version 2>&1 | head -1 | sed -E 's/.*version "?([0-9]+).*/\1/')
    
    # Validate version was extracted successfully
    if [ -n "$CURRENT_JAVA_VERSION" ] && [ "$CURRENT_JAVA_VERSION" -eq "$CURRENT_JAVA_VERSION" ] 2>/dev/null; then
        echo "Current Java version: $CURRENT_JAVA_VERSION"
        
        if [ "$CURRENT_JAVA_VERSION" -ge 21 ]; then
            print_result 0 "Java version is 21 or higher (upgrade may not be needed)"
        else
            echo -e "${YELLOW}INFO:${NC} Java version is $CURRENT_JAVA_VERSION (upgrade recommended)"
            print_result 0 "Java is installed (version $CURRENT_JAVA_VERSION)"
        fi
    else
        echo -e "${YELLOW}WARN:${NC} Could not determine Java version"
        print_result 0 "Java is installed but version could not be determined"
    fi
else
    echo -e "${YELLOW}WARN:${NC} Java is not currently installed"
    print_result 0 "Java not found (upgrade scripts will install it)"
fi

echo ""
echo "Test 7: Verify documentation references"

# Check if README.md references JAVA_UPGRADE.md
if grep -q "JAVA_UPGRADE.md" README.md; then
    print_result 0 "README.md references JAVA_UPGRADE.md"
else
    print_result 1 "README.md does not reference JAVA_UPGRADE.md"
fi

# Check if JAVA_UPGRADE.md mentions upgrade scripts
if grep -q "upgrade-java.sh" JAVA_UPGRADE.md; then
    print_result 0 "JAVA_UPGRADE.md mentions upgrade-java.sh"
else
    print_result 1 "JAVA_UPGRADE.md does not mention upgrade-java.sh"
fi

echo ""
echo "========================================="
echo "Test Summary"
echo "========================================="
echo -e "Tests Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Tests Failed: ${RED}$TESTS_FAILED${NC}"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    echo ""
    echo "The Java upgrade tools are ready to use."
    echo "To upgrade Java to version 21, run: ./upgrade-java.sh"
    exit 0
else
    echo -e "${RED}Some tests failed!${NC}"
    echo "Please review the failures above."
    exit 1
fi
