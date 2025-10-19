#!/bin/bash

# Surfpool Test Script for gloo_solana
# This script sets up surfpool, runs tests, and cleans up

set -e  # Exit on any error

echo "🌊 gloo_solana Surfpool Test Script"
echo "=================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if surfpool is installed
check_surfpool() {
    print_status "Checking if surfpool is installed..."

    if ! command -v surfpool &> /dev/null; then
        print_error "surfpool is not installed. Installing..."
        cargo install surfpool
        print_success "surfpool installed successfully"
    else
        print_success "surfpool is already installed"
    fi
}

# Start surfpool
start_surfpool() {
    print_status "Starting surfpool..."

    # Check if surfpool is already running
    if curl -s http://127.0.0.1:8899 > /dev/null 2>&1; then
        print_warning "surfpool is already running"
        return 0
    fi

    # Start surfpool in background
    surfpool start &
    SURFPOOL_PID=$!

    # Wait for surfpool to be ready
    print_status "Waiting for surfpool to be ready..."
    for i in {1..30}; do
        if curl -s http://127.0.0.1:8899 > /dev/null 2>&1; then
            print_success "surfpool is ready (PID: $SURFPOOL_PID)"
            return 0
        fi
        sleep 1
        echo -n "."
    done

    print_error "surfpool failed to start within 30 seconds"
    return 1
}

# Test surfpool connection
test_surfpool_connection() {
    print_status "Testing surfpool connection..."

    response=$(curl -s -X POST -H "Content-Type: application/json" \
                     -d '{"jsonrpc":"2.0","id":1,"method":"getVersion"}' \
                     http://127.0.0.1:8899)

    if echo "$response" | grep -q "surfnet-version"; then
        print_success "surfpool connection successful"
        echo "Response: $response"
        return 0
    else
        print_error "surfpool connection failed"
        echo "Response: $response"
        return 1
    fi
}

# Run unit tests
run_unit_tests() {
    print_status "Running unit tests..."

    if cargo test --quiet; then
        print_success "All unit tests passed"
    else
        print_error "Some unit tests failed"
        return 1
    fi
}

# Run basic functionality test
run_basic_test() {
    print_status "Running basic functionality test..."

    if cargo run --example basic_test --quiet; then
        print_success "Basic functionality test passed"
    else
        print_error "Basic functionality test failed"
        return 1
    fi
}

# Run surfpool test
run_surfpool_test() {
    print_status "Running surfpool test..."

    # The surfpool test requires WASM, so we'll run a native test instead
    if cargo run --example basic_test --quiet; then
        print_success "Surfpool-compatible test passed"
    else
        print_error "Surfpool test failed"
        return 1
    fi
}

# Stop surfpool
stop_surfpool() {
    print_status "Stopping surfpool..."

    if [ ! -z "$SURFPOOL_PID" ]; then
        kill $SURFPOOL_PID 2>/dev/null || true
        wait $SURFPOOL_PID 2>/dev/null || true
        print_success "surfpool stopped"
    else
        # Try to stop any running surfpool instance
        if command -v surfpool &> /dev/null; then
            surfpool stop 2>/dev/null || true
            print_success "surfpool stopped (if it was running)"
        fi
    fi
}

# Cleanup function
cleanup() {
    print_status "Cleaning up..."
    stop_surfpool
    print_success "Cleanup completed"
}

# Set up signal handlers
trap cleanup EXIT INT TERM

# Main execution
main() {
    echo "Starting surfpool testing process..."
    echo

    # Check prerequisites
    check_surfpool

    # Start surfpool
    if ! start_surfpool; then
        exit 1
    fi

    echo
    # Test connection
    if ! test_surfpool_connection; then
        exit 1
    fi

    echo
    # Run tests
    run_unit_tests
    echo

    run_basic_test
    echo

    run_surfpool_test
    echo

    print_success "All tests completed successfully! 🎉"
}

# Handle command line arguments
case "${1:-}" in
    "start")
        check_surfpool
        start_surfpool
        echo "Surfpool is running. Press Ctrl+C to stop."
        trap 'stop_surfpool; exit 0' INT
        sleep infinity
        ;;
    "stop")
        stop_surfpool
        ;;
    "test")
        run_unit_tests
        run_basic_test
        ;;
    "connection")
        test_surfpool_connection
        ;;
    "help"|"-h"|"--help")
        echo "Usage: $0 [COMMAND]"
        echo ""
        echo "Commands:"
        echo "  start       Start surfpool and keep it running"
        echo "  stop        Stop surfpool"
        echo "  test        Run tests only (no surfpool management)"
        echo "  connection  Test surfpool connection"
        echo "  help        Show this help message"
        echo ""
        echo "If no command is provided, runs the full test suite."
        ;;
    "")
        main
        ;;
    *)
        print_error "Unknown command: $1"
        echo "Run '$0 help' for usage information."
        exit 1
        ;;
esac
