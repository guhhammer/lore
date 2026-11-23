# vagrant port

List mappings between guest and host ports.

## Usage
- `vagrant port` : list all port mappings of machine(s) running in the current directory
- `vagrant port machine_name` : list mappings for a specific machine (if `Vagrantfile` is multi-machine)
- `vagrant port --guest port_number` : display info for a specific guest port
- `vagrant port --machine-readable` : display machine-readable output

## Categories
network

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli/port
- tldr page: https://tldr.inbrowser.app/pages/common/vagrant-port
