# faketime

Fake the system time for a command.

## Usage
- `faketime 'today 23:30' date` : fake the time to this evening, before printing the result of `date`
- `faketime 'yesterday' bash` : open a new Bash shell, which uses yesterday as the current date
- `faketime 'next Friday 1 am' path/to/program` : simulate how a program would act next Friday night
- `faketime '2020-01-01 12:00:00' program` : run a program on a specific date and time

## Links
- More information: https://manned.org/faketime
- tldr page: https://tldr.inbrowser.app/pages/linux/faketime
