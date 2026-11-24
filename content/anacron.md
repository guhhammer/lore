# anacron

Runs commands periodically.

## Usage
- `anacron [-s] [-f] [-n] [-d] [-q] [-t anacrontab] [-S spooldir] [job] ...` : the general form, as the manual gives it
- `anacron [-S spooldir] -u [-t anacrontab] [job] ...` : another form of the same command
- `anacron [-V|-h]` : another form of the same command
- `anacron -T [-t anacrontab]` : another form of the same command
- `anacron -f` : force execution of the jobs, ignoring the timestamps
- `anacron -s` : serialize execution of jobs
- `anacron -n` : run jobs now

## Categories
processes

## Links
- Wikipedia: https://en.wikipedia.org/wiki/Anacron
