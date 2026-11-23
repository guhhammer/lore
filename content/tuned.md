# tuned

Dynamic adaptive system tuning daemon.

## Usage
- `sudo tuned` : start the `tuned` daemon in the foreground
- `sudo tuned -d` : start the `tuned` daemon in the background
- `sudo tuned -p profile_name` : start the `tuned` daemon with a specific profile
- `sudo tuned -l path/to/log_file.log` : start the `tuned` daemon logging to a custom log file
- `sudo tuned -P path/to/process_id_file.pid` : start the `tuned` daemon writing its process ID to a custom PID file
- `sudo tuned -D` : start the `tuned` daemon with debugging messages

## See also
- tuned-adm

## Categories
system, users, files

## Links
- More information: https://manned.org/tuned
- tldr page: https://tldr.inbrowser.app/pages/linux/tuned
