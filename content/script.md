# script

Record all terminal output to a typescript file.

## Usage
- `script` : record a new session to a file named `typescript` in the current directory
- `exit` : stop recording
- `script path/to/session.out` : record a new session to a custom filepath
- `script -a logfile.log` : append to an existing file
- `script -t 2>path/to/timing_file` : record timing information (data is outputted to `stderr`)
- `script -f path/to/file` : write out data as soon as it happens
- `script -q logfile.log` : execute quietly without start and done messages
- `script -h` : display help

## Categories
development, files, media

## Links
- More information: https://manned.org/script
- tldr page: https://tldr.inbrowser.app/pages/linux/script
