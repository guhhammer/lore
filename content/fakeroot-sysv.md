# fakeroot-sysv

Run a command in an environment faking root privileges for file manipulation.

## Usage
- `fakeroot-sysv -l library` : specify an alternative wrapper library
- `fakeroot-sysv --faked binary` : specify an alternative binary to use as faked
- `fakeroot-sysv -s save-file` : save the fakeroot environment to save-file on exit
- `fakeroot-sysv -i load-file` : load a fakeroot environment previously saved using -s from load-file

## Categories
files, monitoring
