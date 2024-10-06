# Procure to pay analysis

## Log Processing in procure2pay

This project processes and analyzes activity logs to determine the most frequent sequences of activities in the data.
Initially considered implementing the solution in Java, but chose Rust in the end, as had *some* recent experience with it.

## Data

Before starting, the data was checked for formatting issues, duplicates, and ordering. 
Assumed some transitions between activities happen instantly, while others might be automated, like those triggered by a cron job (typically those events by vendor all happen at "00:00:00.000").
Edge cases, like simultaneous entries due to automated syncing, were taken into account.

## Performance

Checked with rust tools and visualized by flame-chart: [!profile](profiler.png) (note the highlights on malloc/free which makes ~45% of processing time, more about it in future considerations).

Optimized the initial data sorting by using parallel processing by case IDs, timestamp and activity name (due to duplicates and auto-transitions, described above). An important step was splitting the dataset by case_id for each thread, ensuring that each thread could process its portion without any overlap. This minimized the need for expensive merging operations later on. We also reduced memory usage by converting activities into more compact formats (which also helped while comparing).
The sequential implementation served as a baseline, while the parallel version was fine-tuned for speed and efficiency.


## Scope

I didn’t dive deep into analyzing potential transitions between activities or handling incomplete transitions. Also assumed activities are static constants, that won't change runtime (If we encounter unexpected activity name, we throw an exception).

Future improvements could include integrating real workflow rules to make the analysis more relevant to specific business processes. Replacing CSV parsing with a database connection. Focus on further performance optimizations (C++ engineers often optimize heap processing, I intentionally left this aspect aside, but could gain additional ~30% based on profiling). Another idea is to focus on top-10 variants when counting occurrences, so we don't need to sort the final vector of variants, but this sort is quite cheap so we won't gain much (this depends on data, though; could be significant with another dataset)

## Correctness

Unit test with some edge cases, like invalid or empty input data, duplicated data. Some unit tests also cover smaller sets of actual correct data and the data that were generated (from the expected variants) in a reverse order.
Integration tests compared the outputs of the sequential and parallel implementations to ensure consistency, also trying to reduce (decimate) input data and compare the outputs.
There's always room for improvement, especially in expanding edge case testing and enhancing error handling.

Example of test script output:
```bash
rust/procure2pay (dev/sketch_in_rust)$ ./run_tests.sh 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/procure2pay`
error: the following required arguments were not provided:
  <file>

Usage: procure2pay <file>

For more information, try '--help'.
We expect non-zero exit code (no args)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests src/main.rs (target/debug/deps/procure2pay-a8af0c3d2cf9c88d)

running 9 tests
test tests::tests::test_common_set ... ok
test tests::tests::test_duplicate_activities_in_case ... ok
test tests::tests::test_activities_with_the_same_timestamp ... ok
test tests::tests::test_invalid_activity ... ok
test tests::tests::test_no_input ... ok
test tests::tests::test_generated_from_expected_variants ... ok
test tests::tests::test_more_cases_with_one_variants ... ok
test tests::tests::test_one_valid_activity ... ok
test tests::tests::test_long_variant ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests src/main.rs (target/debug/deps/procure2pay-f0b721b6f3b4b2d4)

running 9 tests
test tests::tests::test_no_input ... ok
test tests::tests::test_one_valid_activity ... ok
test tests::tests::test_common_set ... ok
test tests::tests::test_activities_with_the_same_timestamp ... ok
test tests::tests::test_generated_from_expected_variants ... ok
test tests::tests::test_duplicate_activities_in_case ... ok
test tests::tests::test_long_variant ... ok
test tests::tests::test_invalid_activity ... ok
test tests::tests::test_more_cases_with_one_variants ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/procure2pay ../../samples/Activity_Log_2004_to_2014.csv --with-names`
Duration: 8 milliseconds
[[["Create FI invoice by vendor","Post invoice in FI","Clear open item"],3201],[["Create purchase order item","Change purchase order item","Record order confirmation","Create MM invoice by vendor","Enter goods receipt","Post invoice in MM","Clear open item"],824],[["Create purchase order item","Create MM invoice by vendor","Change purchase order item","Increase purchase order item net value","Increase purchase order item quantity","Enter goods receipt","Post invoice in MM","Remove payment block","Clear open item"],480],[["Create MM invoice by vendor","Create purchase order item","Enter goods receipt","Post invoice in MM","Clear open item"],448],[["Create purchase order item","Create MM invoice by vendor","Change purchase order item","Increase purchase order item net value","Increase purchase order item quantity","Enter goods receipt","Post invoice in MM","Clear open item"],374],[["Create purchase order item","Create MM invoice by vendor","Change purchase order item","Reduce purchase order item net value","Reduce purchase order item quantity","Enter goods receipt","Post invoice in MM","Remove payment block","Clear open item"],347],[["Create MM invoice by vendor","Post invoice in MM","Clear open item"],309],[["Create purchase order item","Create MM invoice by vendor","Change purchase order item","Reduce purchase order item net value","Reduce purchase order item quantity","Enter goods receipt","Post invoice in MM","Clear open item"],198],[["Create purchase order item","Change purchase order item","Record order confirmation","Enter goods receipt","Create MM invoice by vendor","Post invoice in MM","Clear open item"],190],[["Create purchase order item","Create MM invoice by vendor","Enter goods receipt","Post invoice in MM","Clear open item"],83]]
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/procure2pay ../../samples/Activity_Log_2004_to_2014.csv`
Passed: Duration (10) is below 50 ms
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/procure2pay ../../samples/Activity_Log_2004_to_2014.csv --no-time-eval`
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/procure2pay ../../samples/Activity_Log_2004_to_2014.csv --gold --no-time-eval`
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/procure2pay ../../samples/Activity_Log_2004_to_2014.csv --decimate 10 --no-time-eval`
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/procure2pay ../../samples/Activity_Log_2004_to_2014.csv --decimate 10 --gold --no-time-eval`
All checks passed!
```
