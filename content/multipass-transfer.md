# multipass transfer

Transfer files between the host and the virtual machine. Note: Relative paths on an instance start from the user's home directory.

## Usage
- `multipass transfer path/to/local_file instance_name:path/to/remote_directory` : transfer a file from the host to an instance
- `multipass transfer instance_name:path/to/remote_file path/to/local_directory` : transfer a file from an instance to the host
- `multipass transfer -r path/to/local_directory instance_name:path/to/remote_directory` : transfer a directory from the host to an instance

## Categories
files, network, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/multipass-transfer
