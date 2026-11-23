# systemctl list-machines

List the host and all running local virtual machines or containers with their state.

## Usage
- `systemctl list-machines` : show all machines (host and running containers/VMs)
- `systemctl list-machines machine` : list a specific machine
- `systemctl list-machines machine_1 machine_2 ...` : list multiple matching machines
- `systemctl list-machines pattern` : filter machines using wildcard patterns i.e. a `glob`

## Categories
network, containers

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-list-machines
