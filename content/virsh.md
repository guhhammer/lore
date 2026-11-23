# virsh

Manage `virsh` guest domains. Note: Some of the commands below may require specifying `virsh --connect URI` explicitly. Some subcommands such as `list` have their own usage documentation.

## Usage
- `virsh -c qemu:///system|qemu:///session|xen:///system|lxc:///system|...` : connect to a hypervisor session interactively
- `virsh -c uri list --all` : list all domains
- `sudo virsh net-start default` : activate a network named `default`
- `sudo virsh create path/to/config_file.xml` : create a domain from a configuration file
- `sudo virsh edit domain` : edit a domain's configuration file (editor can be changed with `$EDITOR` or `$VISUAL`)
- `sudo virsh start|reboot|reset|shutdown|destroy|suspend|resume domain` : start/reboot/reset/shutdown/destroy/suspend/resume a domain
- `sudo virsh save domain path/to/state_file` : save the current running state of a domain (RAM, but not disk state) to a state file (domain will be powered off)
- `sudo virsh undefine domain --remove-all-storage --snapshots-metadata` : remove storage and snapshots of a stopped domain

## Categories
users, files, system

## Links
- More information: https://libvirt.org/manpages/virsh.html
- tldr page: https://tldr.inbrowser.app/pages/common/virsh
