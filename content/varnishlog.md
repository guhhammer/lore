# varnishlog

Display Varnish logs.

## Usage
- `varnishlog` : display logs in real time
- `varnishlog -q 'ReqHeader eq "Host: example.com"'` : only display requests to a specific domain
- `varnishlog -q 'ReqMethod eq "POST"'` : only display POST requests
- `varnishlog -q 'ReqURL eq "/path"'` : only display requests to a specific path
- `varnishlog -q 'ReqURL ~ "regex"'` : only display requests to paths matching a `regex`

## Categories
desktop, network

## Links
- More information: https://vinyl-cache.org/docs/trunk/reference/varnishlog.html
- tldr page: https://tldr.inbrowser.app/pages/common/varnishlog
