# valkey-cli

Open a connection to a Valkey server.

## Usage
- `valkey-cli` : connect to the local server
- `valkey-cli -h host` : connect to a remote server on the default port (6379)
- `valkey-cli -h host -p port` : connect to a remote server specifying a port number
- `valkey-cli -u uri` : connect to a remote server specifying a URI
- `valkey-cli -a password` : specify a password
- `valkey-cli valkey_command` : execute valkey command
- `valkey-cli -c` : connect to the local cluster

## Categories
network

## Links
- More information: https://valkey.io/topics/cli/
- tldr page: https://tldr.inbrowser.app/pages/common/valkey-cli
