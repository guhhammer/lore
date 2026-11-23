# gnmic get

Get a snapshot of a gnmi network device operation data.

## Usage
- `gnmic -a ip:port get --path path` : get a snapshot of the device state at a specific path
- `gnmic -a ip:port get --path path/to/file_or_directory1 --path path/to/file_or_directory2` : query the device state at multiple paths
- `gnmic -a ip:port get --prefix prefix --path path/to/file_or_directory1 --path path/to/file_or_directory2` : query the device state at multiple paths with a common prefix
- `gnmic -a ip:port get --path path -e json_ietf` : query the device state and specify response encoding (json_ietf)

## Categories
files, network, hardware

## Links
- More information: https://gnmic.openconfig.net/cmd/get/
- tldr page: https://tldr.inbrowser.app/pages/common/gnmic-get
