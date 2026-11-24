# cargo bench

Compile and execute benchmarks.

## Usage
- `cargo bench` : execute all benchmarks of a package
- `cargo bench --no-fail-fast` : don't stop when a benchmark fails
- `cargo bench --no-run` : compile, but don't run benchmarks
- `cargo bench --bench benchmark` : benchmark the specified benchmark
- `cargo bench --profile profile` : benchmark with the given profile (default: `bench`)
- `cargo bench --examples` : benchmark all example targets
- `cargo bench --bins` : benchmark all binary targets
- `cargo bench --lib` : benchmark the package's library

## Categories
monitoring, development

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-bench.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-bench
- Wikipedia: https://en.wikipedia.org/wiki/Rust_(programming_language)
