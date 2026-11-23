# httpflow

A utility to capture and dump HTTP streams.

## Usage
- `httpflow -i any` : capture traffic on all interfaces
- `httpflow host httpbin.org or host baidu.com` : use a bpf-style capture to filter the results
- `httpflow -u 'regex'` : use a `regex` to filter requests by URLs
- `httpflow -r out.cap` : read packets from PCAP format binary file
- `httpflow -w path/to/directory` : write the output to a directory

## Categories
network, files

## Links
- More information: https://github.com/six-ddc/httpflow
- tldr page: https://tldr.inbrowser.app/pages/common/httpflow
