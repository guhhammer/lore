# bore local

Start a local proxy to a remote server using Bore.

## Usage
- `bore local -t remote_server_address local_port` : expose a local port to a remote Bore server
- `bore local -l host -t remote_server_address local_port` : expose a specific local host instead of `localhost`
- `bore local -t remote_server_address -p remote_port local_port` : specify a remote server port explicitly
- `bore local -t remote_server_address -s your_secret local_port` : use a secret for authentication
- `bore local -h` : display help

## Categories
network

## Links
- More information: https://github.com/ekzhang/bore#detailed-usage
- tldr page: https://tldr.inbrowser.app/pages/linux/bore-local
