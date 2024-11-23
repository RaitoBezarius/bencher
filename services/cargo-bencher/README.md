# Bencher for Cargo

[Bencher](https://bencher.dev) is a suite of
[continuous benchmarking](https://bencher.dev/docs/explanation/continuous-benchmarking/) tools.
Have you ever had a performance regression impact your users?
Bencher could have prevented that from happening.
Bencher allows you to detect and prevent performance regressions _before_ they hit production.

- **Run**: Run your benchmarks locally or in CI using your favorite benchmarking tools. The `bencher` CLI simply wraps your existing benchmark harness and stores its results.
- **Track**: Track the results of your benchmarks over time. Monitor, query, and graph the results using the Bencher web console based on the source branch, testbed, benchmark, and measure.
- **Catch**: Catch performance regressions in CI. Bencher uses state of the art, customizable analytics to detect performance regressions before they make it to production.

For the same reasons that unit tests are run in CI to prevent feature regressions, benchmarks should be run in CI with Bencher to prevent performance regressions. Performance bugs are bugs!

## Supported Benchmark Harnesses

- [libtest bench](https://bencher.dev/docs/explanation/adapters/#-rust-bench)
- [Criterion](https://bencher.dev/docs/explanation/adapters/#-rust-criterion)
- [Iai](https://bencher.dev/docs/explanation/adapters/#-rust-iai)
- [Iai-Callgrind](https://bencher.dev/docs/explanation/adapters/#-rust-iai-callgrind)
