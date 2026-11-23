# rexec

Execute a command on a remote host. Note: Use `rexec` with caution, as it transmits data in plain text. Consider secure alternatives like SSH for encrypted communication.

## Usage
- `rexec -h remote_host ls -l` : execute a command on a remote host
- `rexec -u username -h remote_host ps aux` : specify the remote username on a remote host
- `rexec -n -h remote_host ls -l` : redirect `stdin` from `/dev/null` on a remote host
- `rexec -P 1234 -h remote_host ls -l` : specify the remote port on a remote host

## Categories
network, security, text

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/rexec
