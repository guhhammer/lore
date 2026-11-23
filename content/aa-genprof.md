# aa-genprof

Generate AppArmor security profiles by monitoring program behavior.

## Usage
- `sudo aa-genprof path/to/program` : start generating a profile for a program
- `sudo aa-genprof -d /path/to/profiles path/to/program` : specify a custom directory for profiles
- `sudo aa-genprof -f /path/to/logfile path/to/program` : specify a custom logfile for profiling
- `aa-genprof -h` : display help

## Categories
files, users

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-genprof.8
- tldr page: https://tldr.inbrowser.app/pages/linux/aa-genprof
