# psping

A ping tool that includes TCP ping, latency, and bandwidth measurement.

## Usage
- `psping hostname` : ping a host using ICMP
- `psping hostname:port` : ping a host over a TCP port
- `psping hostname -n pings -q` : specify the number of pings and perform it quietly
- `psping hostname:port -q -n 50 -h` : ping the target over TCP 50 times and produce a histogram of the results
- `psping /?` : display help

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
network, windows

## Links
- More information: https://learn.microsoft.com/sysinternals/downloads/psping
- tldr page: https://tldr.inbrowser.app/pages/windows/psping
