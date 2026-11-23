# rsh

Execute commands on a remote host.

## Usage
- `rsh remote_host ls -l` : execute a command on a remote host
- `rsh remote_host -l username ls -l` : execute a command on a remote host with a specific username
- `rsh remote_host --no-err ls -l` : redirect `stdin` to `/dev/null` when executing a command on a remote host

## Categories
network

## Links
- More information: https://www.gnu.org/software/inetutils/manual/inetutils.html#rsh-invocation
- tldr page: https://tldr.inbrowser.app/pages/linux/rsh
