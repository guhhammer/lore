# cupsctl

Update or query a server's `cupsd.conf`.

## Usage
- `cupsctl` : display the current configuration values
- `cupsctl -h server:port` : display the configuration values of a specific server
- `cupsctl -E` : enable encryption on the connection to the scheduler
- `cupsctl --debug-logging|--no-debug-logging` : enable or disable debug logging to the `error_log` file
- `cupsctl --remote-admin|--no-remote-admin` : enable or disable remote administration
- `cupsctl | grep '^_debug_logging' | awk -F= '{print $2}'` : parse the current debug logging state

## Categories
development, databases, network

## Links
- More information: https://openprinting.github.io/cups/doc/man-cupsctl.html
- tldr page: https://tldr.inbrowser.app/pages/common/cupsctl
