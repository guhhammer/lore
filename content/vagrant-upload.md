# vagrant upload

Upload files and directories from the host to the guest machine.

## Usage
- `vagrant upload path/to/source_file_or_directory path/to/destination_file_or_directory name|id` : upload file or directory from the host to the guest machine
- `vagrant upload --compress path/to/source_file_or_directory path/to/destination_file_or_directory name|id` : compress the file or directory before uploading to guest machine
- `vagrant upload --compression-type tgz|zip path/to/source_file_or_directory path/to/destination_file_or_directory name|id` : specify which type of compression to use. Default type is `zip`
- `vagrant upload --temporary path/to/source_file_or_directory path/to/destination_file_or_directory name|id` : create a temporary location on the guest machine and upload files to that location

## Categories
network, files, archives

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli/upload
- tldr page: https://tldr.inbrowser.app/pages/common/vagrant-upload
- Wikipedia: https://en.wikipedia.org/wiki/Vagrant_(software)
