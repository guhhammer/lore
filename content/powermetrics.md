# powermetrics

Gather and display CPU usage, power, and interrupt wakeup statistics.

## Usage
- `sudo powermetrics` : display CPU, power, and wakeup metrics using the default 5-second sample interval
- `sudo powermetrics -i 2000 -n 10` : sample metrics every 2000 milliseconds taking 10 samples then exit
- `sudo powermetrics -o path/to/output.txt` : save output to a file instead of `stdout`
- `sudo powermetrics -r pid|wakeups|cputime|composite` : order the process list by the specified method (default: `composite`)
- `sudo powermetrics -f plist --show-usage-summary` : display output in machine-readable property list format and print a usage summary on exit

## Categories
monitoring, desktop, users, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/powermetrics.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/powermetrics
