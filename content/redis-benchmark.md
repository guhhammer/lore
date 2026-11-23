# redis-benchmark

Benchmark a Redis server.

## Usage
- `redis-benchmark` : run full benchmark
- `redis-benchmark -h host -p port -a password` : run benchmark on a specific Redis server
- `redis-benchmark -h host -p port -t set,lpush -n 100000` : run a subset of tests with default 100000 requests
- `redis-benchmark -n 100000 script load "redis.call('set', 'foo', 'bar')"` : run with a specific script
- `redis-benchmark -t set -r 100000` : run benchmark by using 100000 random keys
- `redis-benchmark -n 1000000 -t set,get -P 16` : run benchmark by using a Pipelining of 16 commands
- `redis-benchmark -q` : run benchmark quietly and only show query per seconds result

## Categories
monitoring, databases, network

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/redis-benchmark
