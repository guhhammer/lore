# systemd-resolve

Resolve domain names, IPv4 and IPv6 addresses, DNS resource records, and services. Note: This tool has been renamed to `resolvectl` in new versions of `systemd`.

## Usage
- `resolvectl` : show DNS settings
- `resolvectl query domain1 domain2 ...` : resolve the IPv4 and IPv6 addresses for one or more domains
- `resolvectl query ip_address` : retrieve the domain of a specified IP address
- `resolvectl flush-caches` : flush all local DNS caches
- `resolvectl statistics` : display DNS statistics (transactions, cache, and DNSSEC verdicts)
- `resolvectl --legend no -t MX query domain` : retrieve an MX record of a domain
- `resolvectl service _service._protocol name` : resolve an SRV record, for example `_xmpp-server._tcp gmail.com`
- `resolvectl tlsa tcp domain:443` : retrieve a TLS key

## Categories
system, network, databases

## Links
- More information: https://manned.org/systemd-resolve
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-resolve
