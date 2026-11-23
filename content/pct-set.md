# pct set

Set container options.

## Usage
- `pct se 100 --onboot` : set container to start automatically on boot
- `pct se 100 --net0 name=eth0,bridge=vmbr0,ip=10.0.0.100/24,gw=10.0.0.1` : set a container to have a static IP
- `pct se 100 --memory 8192 --cpulimit 4` : set container memory and CPU limit
- `pct se 100 --hostname new_name` : set the hostname of a container
- `pct se 100 --mp0 /path/to/host_directory,mp=/path/to/guest_mount_point` : mount a host file location in a guest
- `pct se 100 --tags tag1,tag2,...` : set container tags
- `pct se 100 --delete unused0,net0,mp0,mp1,...` : remove an option

## Categories
containers, files, network

## Links
- More information: https://pve.proxmox.com/pve-docs/pct.1.html#cli_pct_set
- tldr page: https://tldr.inbrowser.app/pages/linux/pct-set
