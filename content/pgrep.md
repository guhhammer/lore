# pgrep

Find processes by name.

## Usage
- `pgrep process_name` : return PIDs of any running processes with a matching command string
- `pgrep -l process_name` : print the process name in addition to the PID (long output)
- `pgrep -f "process_name parameter"` : match processes against the full argument list instead of just the process name
- `pgrep -fl "process_name"` : print the PID together with the full argument list for each match
- `pgrep -u username process_name` : search for processes run by a specific user (effective UID)
- `pgrep -n process_name` : select only the newest (most recently started) matching process

## See also
- pidof

## Categories
processes

## Links
- More information: https://keith.github.io/xcode-man-pages/pgrep.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/pgrep
