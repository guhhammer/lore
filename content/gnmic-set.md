# gnmic set

Modify gnmi network device configuration.

## Usage
- `gnmic -a ip:port set --update-path gNMI_path --update-value value` : update the value of a path
- `gnmic -a ip:port set --update-path gNMI_path --update-file path/to/file` : update the value of a path to match the contents of a JSON file
- `gnmic -a ip:port set --replace-path gNMI_path --replace-file path/to/file` : replace the value of a path to match the contents of a JSON file
- `gnmic -a ip:port set --delete gNMI_path` : delete the node at a given path

## Categories
files, network, hardware

## Links
- More information: https://gnmic.openconfig.net/cmd/set/
- tldr page: https://tldr.inbrowser.app/pages/common/gnmic-set
