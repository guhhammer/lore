# fakeroot-tcp

Run a command in an environment faking root privileges for file manipulation.

## Usage
- `fakeroot-tcp -l library` : specify an alternative wrapper library
- `fakeroot-tcp --faked binary` : specify an alternative binary to use as faked
- `fakeroot-tcp -s save-file` : save the fakeroot environment to save-file on exit
- `fakeroot-tcp -i load-file` : load a fakeroot environment previously saved using -s from load-file

## Categories
files, monitoring
