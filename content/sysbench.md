# sysbench

Benchmark a System's CPU, IO, and memory.

## Usage
- `sysbench cpu run` : run a CPU benchmark with 1 thread for 10 seconds
- `sysbench --threads=number_of_threads --time=seconds` : run a CPU benchmark with multiple threads for a specified time
- `sysbench memory run` : run a memory benchmark with 1 thread for 10 seconds
- `sysbench fileio prepare` : prepare a filesystem-level read benchmark
- `sysbench --file-test-mode=rndrd|rndrw|rndwr|seqrd|seqrewr|seqwr fileio run` : run a filesystem-level benchmark

## Categories
monitoring, hardware

## Links
- More information: https://github.com/akopytov/sysbench#usage
- tldr page: https://tldr.inbrowser.app/pages/common/sysbench
- Wikipedia: https://en.wikipedia.org/wiki/Sysbench
