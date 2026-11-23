# stress-ng

Load and stress a Linux system in various ways (CPU, memory, I/O, etc.). Useful for benchmarking, hardware validation, and stability testing.

## Usage
- `stress-ng -c 4 -t 60s` : stress all CPUs with 4 workers for 60 seconds
- `stress-ng -m 2 --vm-bytes 512M -t 30s` : stress virtual memory with 2 workers for 30 seconds
- `stress-ng -i 3 -t 45s` : stress the I/O subsystem with 3 workers for 45 seconds
- `stress-ng -a 1 -t 2m` : run all stress tests for 2 minutes

## Categories
monitoring, hardware

## Links
- More information: https://manned.org/stress-ng
- tldr page: https://tldr.inbrowser.app/pages/common/stress-ng
