# squid

Cache and forward HTTP requests through a proxy server.

## Usage
- `sudo squid` : start Squid in the background
- `sudo squid -N` : start Squid in the foreground
- `sudo squid -f path/to/squid.conf` : start Squid with a specific configuration file
- `sudo squid -k parse` : test the configuration file for errors
- `sudo squid -k reconfigure` : reload the configuration file
- `sudo squid -k shutdown` : shut down Squid gracefully
- `sudo squid -k rotate` : rotate the log files

## Categories
network, users, files

## Links
- More information: https://manned.org/squid
- tldr page: https://tldr.inbrowser.app/pages/common/squid
