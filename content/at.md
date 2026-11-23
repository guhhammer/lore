# at

Execute commands once at a later time. Results will be sent to the users mail.

## Usage
- `at now + 5 minutes` : create commands interactively and execute them in 5 minutes (press `<Ctrl d>` when done)
- `at hh:mm` : create commands interactively and execute them at a specific time
- `echo "command" | at 1000` : execute a command from `stdin` at 10:00 AM today
- `at -f path/to/file 9:30 PM Tue` : execute commands from a given file next Tuesday
- `at -l` : list all queued jobs for the current user (same as `atq`)
- `at -c job_number` : view a specified job

## Categories
users, files

## Links
- More information: https://manned.org/at
- tldr page: https://tldr.inbrowser.app/pages/common/at
