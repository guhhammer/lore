# croc

Send and receive files easily and securely over any network.

## Usage
- `croc send path/to/file_or_directory` : send a file or directory
- `croc send -c passphrase path/to/file_or_directory` : send a file or directory with a specific passphrase
- `croc passphrase` : receive a file or directory on receiving machine
- `croc --relay ip_to_relay send path/to/file_or_directory` : send and connect over a custom relay
- `croc --relay ip_to_relay passphrase` : receive and connect over a custom relay
- `croc relay` : host a croc relay on the default ports
- `croc send|relay --help` : display parameters and options for a croc command

## Categories
files, network

## Links
- More information: https://github.com/schollz/croc
- tldr page: https://tldr.inbrowser.app/pages/common/croc
