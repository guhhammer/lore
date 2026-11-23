# tlsx

Fast and configurable TLS grabber focused on TLS based data collection and analysis.

## Usage
- `tlsx -u host1,host2,... -tv -cipher` : display TLS version and cipher of one or more hosts
- `tlsx -u host -ve -ce` : enumerate and display supported TLS versions and ciphers of a host
- `tlsx -l path/to/hosts.txt -ex -ss -mm -re -un` : scan a list of hosts from a file and mark hosts with expired/self-signed/mismatched/revoked/untrusted certificates
- `tlsx -l path/to/hosts.txt -c 300 -timeout 5 -retry 3 -delay 200ms -wc` : adjust per host concurrency, timeout, retry, and delay parameters when scanning a list of hosts for wildcard SSL certificates
- `tlsx -u host -dns` : display unique hostname(s) from SSL certificate response
- `tlsx -u host -san -j -o path/to/file.json` : display Subject Alternative Names (SANs) from the TLS certificate of a host, with JSON output written to a file
- `tlsx -hc` : perform a self health-check of `tlsx` itself

## Categories
security, network, files

## Links
- More information: https://github.com/projectdiscovery/tlsx#usage
- tldr page: https://tldr.inbrowser.app/pages/common/tlsx
