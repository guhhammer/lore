# aa-logprof

Interactively update AppArmor security profiles based on logged violations.

## Usage
- `sudo aa-logprof` : interactively review and update profiles based on system logs
- `sudo aa-logprof -d /path/to/profiles` : use a specific directory for AppArmor profiles
- `sudo aa-logprof -f /path/to/logfile` : use a specific log file instead of the default
- `sudo aa-logprof -m "log_marker_text"` : ignore all log entries before the specified mark
- `aa-logprof -h` : display help

## Categories
files, users

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-logprof.8
- tldr page: https://tldr.inbrowser.app/pages/linux/aa-logprof
