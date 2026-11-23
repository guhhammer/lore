# dbiproxy

A proxy server for the DBD::Proxy driver.

## Usage
- `dbiproxy <options> --localport=<port>` : the general form, as the manual gives it
- `dbiproxy --debug` : turn debugging mode on
- `dbiproxy --facility=mode` : (UNIX only) Facility to use for Sys::Syslog
- `dbiproxy --group=gid` : after doing a bind(), change the real and effective GID to the given
- `dbiproxy --localaddr=ip` : by default a daemon is listening to any IP number that a machine has

## Categories
network
