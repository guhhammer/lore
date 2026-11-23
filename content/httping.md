# httping

Measure the latency and throughput of a web server.

## Usage
- `httping -g url` : ping the specified URL
- `httping -h host -p port` : ping the web server on `host` and `port`
- `httping -l -g https://host` : ping the web server on `host` using a TLS connection
- `httping -g http://host -U username -P password` : ping the web server on `host` using HTTP basic authentication

## Categories
network

## Links
- More information: https://manned.org/httping
- tldr page: https://tldr.inbrowser.app/pages/common/httping
