# multipass

Manage Ubuntu virtual machines using native hypervisors.

## Usage
- `multipass find` : list the aliases that can be used to launch an instance
- `multipass launch -n instance_name --cloud-init configuration_file` : launch a new instance, set its name, and use a cloud-init configuration file
- `multipass list` : list all the created instances and some of their properties
- `multipass start instance_name` : start a specific instance by name
- `multipass info instance_name` : show the properties of an instance
- `multipass shell instance_name` : open a shell prompt on a specific instance by name
- `multipass delete instance_name` : delete an instance by name
- `multipass mount path/to/local_directory instance_name:path/to/target_directory` : mount a directory into a specific instance

## Categories
files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/multipass
