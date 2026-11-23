# prstat

Report active process statistics.

## Usage
- `prstat` : examine all processes and report statistics sorted by CPU usage
- `prstat -s rss` : examine all processes and report statistics sorted by memory usage
- `prstat -t` : report total usage summary for each user
- `prstat -m` : report microstate process accounting information
- `prstat -c -n 5 -s cpu 1` : print out a list of top 5 CPU using processes every second

## Notes
A SunOS (Solaris) command: this page is for reference, it does not run on Linux or macOS.

## Categories
monitoring, processes, hardware, sunos

## Links
- More information: https://www.unix.com/man-page/sunos/1m/prstat
- tldr page: https://tldr.inbrowser.app/pages/sunos/prstat
