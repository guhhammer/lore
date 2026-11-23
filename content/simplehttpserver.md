# simplehttpserver

A simple HTTP/S server that supports file upload, basic authentication, and YAML rules for custom responses. A Go alternative to Python's `http.server`.

## Usage
- `simplehttpserver -verbose` : start the HTTP server serving the current directory with verbose output (listen on all interfaces and port 8000 by default)
- `sudo simplehttpserver -basic-auth username:password -path /var/www/html -listen 0.0.0.0:80` : start the HTTP server with basic authentication serving a specific path over port 80 on all interfaces
- `sudo simplehttpserver -https -domain *.selfsigned.com -listen 0.0.0.0:443` : start the HTTP server, enabling HTTPS using a self-signed certificate with custom SAN on all interfaces
- `simplehttpserver -upload -header 'X-Powered-By: Go' -header 'Server: SimpleHTTPServer'` : start the HTTP server with custom response headers and upload capability
- `simplehttpserver -rules rules.yaml` : start the HTTP server with customizable rules in YAML (see documentation for DSL)

## Categories
network, files, text

## Links
- More information: https://github.com/projectdiscovery/simplehttpserver
- tldr page: https://tldr.inbrowser.app/pages/common/simplehttpserver
