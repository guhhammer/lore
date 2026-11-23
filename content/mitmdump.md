# mitmdump

View, record, and programmatically transform HTTP traffic. The command-line counterpart to mitmproxy.

## Usage
- `mitmdump -w path/to/file` : start a proxy and save all output to a file
- `mitmdump -nr input_filename -w output_filename "~m post"` : filter a saved traffic file to just POST requests
- `mitmdump -nc path/to/file` : replay a saved traffic file
- `sudo mitmdump -m dns` : intercept DNS traffic (starts an intercepting DNS server on 127.0.0.1:53)

## Categories
network, files, media

## Links
- More information: https://docs.mitmproxy.org/stable/#mitmdump
- tldr page: https://tldr.inbrowser.app/pages/common/mitmdump
