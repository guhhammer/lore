# xhost

Manage access control lists for X server connections.

## Usage
- `xhost` : display the current access control list
- `xhost +hostname` : allow a specific host to connect to the X server
- `xhost -hostname` : deny a specific host from connecting to the X server
- `xhost +` : allow all hosts to connect (disable access control - insecure)
- `xhost -` : deny all hosts except those explicitly allowed (enable access control)
- `xhost -family:name` : remove a specific user or address using a family prefix (like `inet:hostname` or `si:localuser:username`)

## Categories
network

## Links
- More information: https://manned.org/xhost
- tldr page: https://tldr.inbrowser.app/pages/linux/xhost
