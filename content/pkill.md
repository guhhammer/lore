# pkill

Signal process by name. Mostly used for stopping processes.

## Usage
- `pkill "process_name"` : kill all processes which match
- `pkill -f "command_name"` : kill all processes which match their full command instead of just the process name
- `pkill -9 "process_name"` : force kill matching processes (can't be blocked)
- `pkill -USR1 "process_name"` : send `SIGUSR1` signal to processes which match
- `pkill -o "firefox"` : kill the main `firefox` process to close the browser

## Categories
danger, processes

## Links
- More information: https://manned.org/pkill
- tldr page: https://tldr.inbrowser.app/pages/common/pkill
