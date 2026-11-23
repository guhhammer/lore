# rotatelogs

Piped logging program to rotate Apache logs.

## Usage
- `rotatelogs -t` : causes the logfile to be truncated instead of rotated
- `rotatelogs -T` : causes  all  but the initial logfile to be truncated when opened
- `rotatelogs -e` : echo logs through to stdout
- `rotatelogs -c` : create log file for each interval, even if empty
