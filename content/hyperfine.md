# hyperfine

A benchmarking tool.

## Usage
- `hyperfine 'make'` : run a basic benchmark, performing at least 10 runs
- `hyperfine 'make target1' 'make target2'` : run a comparative benchmark
- `hyperfine -m 7 'make'` : change minimum number of benchmarking runs
- `hyperfine -w 5 'make'` : perform benchmark with warmup
- `hyperfine -p 'make clean' 'make'` : run a command before each benchmark run (to clear caches, etc.)
- `hyperfine -p 'make clean' -P num_threads 1 10 'make --jobs {num_threads}'` : run a benchmark where a single parameter changes for each run

## Categories
monitoring

## Links
- More information: https://manned.org/hyperfine
- tldr page: https://tldr.inbrowser.app/pages/common/hyperfine
