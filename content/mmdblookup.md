# mmdblookup

A utility to look up an IP address in a MaxMind DB file.

## Usage
- `mmdblookup -f path/to/dataset.mmdb -i 8.8.8.8` : look up an IP address in the specified MaxMind DB file
- `mmdblookup -f path/to/dataset.mmdb -i 8.8.8.8 key1 key2 ...` : look up a subset of data for an IP address in the specified MaxMind DB file
- `mmdblookup -f path/to/dataset.mmdb -i 8.8.8.8 country iso_code` : look up the country ISO code for an IP address in the specified MaxMind DB file
- `mmdblookup -h` : display help
- `mmdblookup --version` : display version

## Categories
files, network

## Links
- More information: https://maxmind.github.io/libmaxminddb/mmdblookup.html
- tldr page: https://tldr.inbrowser.app/pages/common/mmdblookup
