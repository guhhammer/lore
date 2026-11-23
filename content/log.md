# log

View, export, and configure logging systems.

## Usage
- `log stream` : stream live system logs
- `log stream --process process_id` : stream logs sent to `syslog` from the process with a specific PID
- `log show --predicate "process == 'process_name'"` : show logs sent to syslog from a process with a specific name
- `sudo log collect --last 1h --output path/to/file.logarchive` : export all logs to disk for the past hour

## Categories
monitoring, processes, media, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/log.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/log
