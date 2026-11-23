# pct

Manage LXC containers in Proxmox.

## Usage
- `pct list` : list all containers
- `pct start|stop|reboot 100` : start/Stop/Reboot a specific container
- `pct en 100` : access a specific container's shell
- `pct cr 100 local:vztmpl/distro-name.tar.zst --rootfs local-lvm:4` : create a container from template with 4GB size
- `pct resi 100 rootfs|mpX 20G` : resize the container's disk to 20G
- `pct conf 100` : show the configuration of a container, specifying its ID
- `pct sn 100 my-snapshot --description My snapshot description` : snapshot a specific container with description
- `pct des 100 --purge` : destroy a container and remove all related resources

## Categories
containers

## Links
- More information: https://pve.proxmox.com/pve-docs/pct.1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/pct
