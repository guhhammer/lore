# abrt

This command is an alias of `abrt-cli`.

## Usage
- `abrt-cli list` : list detected problems
- `abrt-cli info problem_id` : show details of a specific problem
- `abrt-cli remove problem_id` : remove a crash report
- `abrt-cli report problem_id` : report a problem to the configured bug tracker (e.g. Bugzilla)
- `abrt-watch-log -F error_string /var/log/myapp.log notify-send "Crash detected"` : monitor a log file and trigger a program when a match is found
- `abrt-cli report -a problem_id` : generate a report for debugging manually

## Categories
monitoring, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/abrt
