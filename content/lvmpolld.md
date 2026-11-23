# lvmpolld

LVM poll daemon that supervises long-running LVM operations.

## Usage
- `lvmpolld -f` : start the daemon in the foreground
- `lvmpolld -f -l debug` : start in the foreground with debug logging
- `lvmpolld -t 300` : set the idle shutdown timeout (seconds)
- `lvmpolld -s /tmp/lvmpolld.socket` : use a custom socket path
- `lvmpolld -p /tmp/lvmpolld.pid` : use a custom PID file
- `lvmpolld --dump` : dump the current state

## Categories
disks, system, network

## Links
- More information: https://manned.org/lvmpolld
- tldr page: https://tldr.inbrowser.app/pages/linux/lvmpolld
