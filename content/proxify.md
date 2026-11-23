# proxify

A versatile and portable proxy for capturing, manipulating, and replaying HTTP/HTTPS traffic on the go.

## Usage
- `proxify` : start an HTTP proxy (on the loopback network interface `127.0.0.1` and port `8888`)
- `proxify -ha "ip_address:port_number"` : start an HTTP proxy on a custom network interface and port (may require `sudo` for a port number lower than `1024`)
- `proxify -of jsonl|yaml -o path/to/file` : specify output format and output file
- `proxify -h` : display help

## See also
- mitmproxy

## Categories
network, files

## Links
- More information: https://github.com/projectdiscovery/proxify#usage
- tldr page: https://tldr.inbrowser.app/pages/common/proxify
