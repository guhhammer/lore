# wondershaper

Allow the user to limit the bandwidth of network adapters.

## Usage
- `wondershaper -s -a adapter_name` : show the current status of a specific adapter
- `wondershaper -c -a adapter_name` : clear limits from a specific adapter
- `wondershaper -a adapter_name -d 1024` : set a specific maximum download rate (in Kbps)
- `wondershaper -a adapter_name -u 512` : set a specific maximum upload rate (in Kbps)
- `wondershaper -a adapter_name -d 1024 -u 512` : set a specific maximum download rate and upload rate (in Kbps)
- `wondershaper -h` : display help

## Categories
network, users

## Links
- More information: https://github.com/magnific0/wondershaper#usage
- tldr page: https://tldr.inbrowser.app/pages/common/wondershaper
