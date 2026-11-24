# haproxy

Fast and reliable HTTP reverse proxy and load balancer.

## Usage
- `haproxy -c -f path/to/haproxy.cfg` : check configuration file for validity
- `haproxy -f path/to/haproxy.cfg` : start HAProxy with a configuration file
- `haproxy -D -f path/to/haproxy.cfg` : start in daemon mode
- `haproxy -W -f path/to/haproxy.cfg` : start in master-worker mode
- `haproxy -d -f path/to/haproxy.cfg` : start with debugging mode enabled (foreground, verbose output)
- `haproxy -f path/to/haproxy.cfg -sf process_id` : reload configuration with graceful shutdown of old process
- `haproxy -f path/to/haproxy.cfg -n maxconn` : set maximum number of simultaneous connections
- `haproxy -f path/to/haproxy.cfg -x path/to/haproxy.sock -sf process_id` : reload with zero downtime by reusing sockets from old process

## Categories
files, network, monitoring

## Links
- More information: https://manned.org/haproxy
- tldr page: https://tldr.inbrowser.app/pages/common/haproxy
- Wikipedia: https://en.wikipedia.org/wiki/HAProxy
