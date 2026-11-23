# autorecon

A multi-threaded network reconnaissance tool which performs automated enumeration of services.

## Usage
- `sudo autorecon host_or_ip1,host_or_ip2,...` : perform reconnaissance on target host(s) (detailed scan results will be dumped in `results`)
- `sudo autorecon -t path/to/file` : perform reconnaissance on target(s) from a file
- `sudo autorecon -o path/to/results host_or_ip1,host_or_ip2,...` : output results to a different directory
- `sudo autorecon -p T:21-25,80,443,U:53,B:123 host_or_ip1,host_or_ip2,...` : limit scanning to specific ports and protocols (`T` for TCP, `U` for UDP, `B` for both)

## Categories
network, files, users

## Links
- More information: https://github.com/AutoRecon/AutoRecon
- tldr page: https://tldr.inbrowser.app/pages/linux/autorecon
