# whatwaf

Detect and bypass web application firewalls and protection systems.

## Usage
- `whatwaf -u https://example.com --verbose` : detect protection on a single URL, optionally use verbose output
- `whatwaf -t number -l path/to/file` : detect protection on a list of URLs in parallel from a file (one URL per line)
- `whatwaf --proxy http://127.0.0.1:8080 --pl path/to/file -u https://example.com` : send requests through a proxy and use custom payload list from a file (one payload per line)
- `whatwaf --tor -p 'payload1,payload2,...' -u https://example.com` : send requests through Tor (Tor must be installed) using custom payloads (comma-separated)
- `whatwaf --ra --throttle seconds --timeout seconds -P --force-ssl -u http://example.com` : use a random user-agent, set throttling and timeout, send a POST request, and force HTTPS connection
- `whatwaf --wafs` : list all WAFs that can be detected
- `whatwaf --tampers` : list all available tamper scripts

## Categories
network, files

## Links
- More information: https://github.com/Ekultek/WhatWaf#basic-help-menu
- tldr page: https://tldr.inbrowser.app/pages/common/whatwaf
