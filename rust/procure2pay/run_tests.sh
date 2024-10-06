set -e

# Build
cargo build

# Run
cargo run || echo "We expect non-zero exit code (no args)"

# Run Unit tests for both variants
cargo test --features sequential
cargo test --features parallel

# Run with samples
cargo  run --release sample_log.csv --with-names
# Evaluate timing (future consideration: performace counters in CI)
duration=$(cargo  run --release sample_log.csv | grep -oP 'Duration: \K\d+(?= milliseconds)')
if [ "$duration" -lt 50 ]; then
    echo "Passed: Duration ($duration) is below 50 ms"
else
    echo "Failed: Duration ($duration) is 50 ms or more!"
    exit 1;
fi

# Integration tests (verify that golden impl yields the same results as the optimized one)
cargo  run --release sample_log.csv --no-time-eval > parallel.txt
cargo  run --release sample_log.csv --gold --no-time-eval > sequential.txt
if ! diff parallel.txt sequential.txt; then
    echo "Output of the two variants differ!";
    cat parallel.txt;
    cat sequential.txt;
    exit 1;
fi

# Check again with decimated inputs
cargo  run --release sample_log.csv --decimate 10 --no-time-eval > parallel.txt
cargo  run --release sample_log.csv --decimate 10 --gold --no-time-eval > sequential.txt
if ! diff parallel.txt sequential.txt; then
    echo "Output of the two variants differ!";
    cat parallel.txt;
    cat sequential.txt;
    exit 1;
fi

echo "All checks passed!"
