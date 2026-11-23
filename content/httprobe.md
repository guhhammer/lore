# httprobe

Take a list of domains and probe for working HTTP and HTTPS servers.

## Usage
- `cat input_file | httprobe` : probe a list of domains from a text file
- `cat input_file | httprobe --prefer-https` : only check for HTTP if HTTPS is not working
- `cat input_file | httprobe -p https:2222` : probe additional ports with a given protocol
- `httprobe --help` : display help

## Categories
network

## Links
- More information: https://github.com/tomnomnom/httprobe
- tldr page: https://tldr.inbrowser.app/pages/common/httprobe
