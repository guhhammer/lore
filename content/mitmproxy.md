# mitmproxy

An interactive man-in-the-middle HTTP proxy.

## Usage
- `mitmproxy` : start `mitmproxy` with default settings (will listen on port `8080`)
- `mitmproxy --listen-host ip_address -p port` : start `mitmproxy` bound to a custom address and port
- `mitmproxy -s path/to/script.py` : start `mitmproxy` using a script to process traffic
- `SSLKEYLOGFILE="path/to/file" mitmproxy` : export the logs with SSL/TLS master keys to external programs (wireshark, etc.)
- `mitmproxy -m regular|transparent|socks5|...` : specify mode of operation of the proxy server (`regular` is the default)
- `mitmproxy --console-layout horizontal|single|vertical` : set the console layout
- `mitmproxy -w path/to/dump.mitm` : save all proxied traffic to a file for later analysis
- `mitmproxy -nr path/to/dump.mitm` : replay a previously saved HTTP flow file

## See also
- mitmweb
- mitmdump

## Categories
network, files

## Links
- More information: https://docs.mitmproxy.org/stable/
- tldr page: https://tldr.inbrowser.app/pages/common/mitmproxy
