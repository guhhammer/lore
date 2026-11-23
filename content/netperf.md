# netperf

Client-side command for `netperf`, the benchmarking application that measures network throughput. Similar to `iperf`.

## Usage
- `netperf address` : connect to server on a specific IP address via default port (12865)
- `netperf address -p port` : specify port
- `netperf address -l seconds` : specify the sampling length in seconds (default is 10)
- `netperf address -4|6` : force IPv[4] or IPv[6]

## See also
- netserver

## Categories
network

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/netperf
