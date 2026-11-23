# sar

Monitor performance of various Linux subsystems.

## Usage
- `sar -b 1` : report I/O and transfer rate issued to physical devices, one per second (press `<Ctrl c>` to quit)
- `sar -n DEV 2 10` : report a total of 10 network device statistics, one per 2 seconds
- `sar -u ALL 2` : report CPU utilization, one per 2 seconds
- `sar -r ALL 1 20` : report a total of 20 memory utilization statistics, one per second
- `sar -q 1` : report the run queue length and load averages, one per second
- `sar -B 5` : report paging statistics, one per 5 seconds

## Categories
monitoring, hardware, network

## Links
- More information: https://manned.org/sar
- tldr page: https://tldr.inbrowser.app/pages/linux/sar
