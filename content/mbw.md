# mbw

Memory Bandwidth Benchmark.

## Usage
- `mbw -n 3 512` : run 3 memory bandwidth tests with 512MB size
- `mbw -n 3 -q -a 512` : run 3 memory bandwidth tests with 512MB memory size, output only statistics, not averages
- `mbw -n 3 -q -t0 512` : run memcpy test 3 times with 512MB size, only display statistics
- `mbw -n 10 -q -t2 -b 1024 8192` : run the memcpy test 10 times with 1024 byte blocks allocated 8192MB of memory
- `mbw -n 0 -t1 -q 2048` : run dumb test with 2048MB size, output only statistics, run forever

## Categories
monitoring, development

## Links
- More information: https://manned.org/mbw
- tldr page: https://tldr.inbrowser.app/pages/linux/mbw
