# wrk

HTTP benchmarking tool.

## Usage
- `wrk -t 12 -c 400 -d 30s "http://127.0.0.1:8080/index.html"` : run a benchmark for `30` seconds, using `12` threads, and keeping `400` HTTP connections open
- `wrk -t 2 -c 5 -d 5s -H "Host: example.com" "http://example.com/index.html"` : run a benchmark with a custom header
- `wrk -t 2 -c 5 -d 5s --timeout 2s "http://example.com/index.html"` : run a benchmark with a request timeout of `2` seconds

## Categories
network, monitoring

## Links
- More information: https://github.com/wg/wrk#basic-usage
- tldr page: https://tldr.inbrowser.app/pages/common/wrk
