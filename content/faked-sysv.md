# faked-sysv

Daemon that remembers fake ownership/permissions of files manipulated by fakeroot processes.

## Usage
- `faked [--debug] [--foreground] [--cleanup] [--key msg-key] [--load] [--save-file save-file] [--port TCP-port]` : the general form, as the manual gives it
- `faked-sysv --debug` : print debugging information on stderr
- `faked-sysv --foreground` : don't fork into the background
- `faked-sysv --cleanup number` : cleanup the semaphores
- `faked-sysv --save-file save-file` : save the environment to save-file on exit

## Categories
files, processes, system
