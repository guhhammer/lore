# pct create

Create LXC containers in Proxmox.

## Usage
- `pct cr 100 local:vztmpl/distro-name.tar.zst --rootfs local-lvm:4` : create a container from a template with 4GB size, give it 512MiB of memory, and unlimited access to CPU
- `pct cr 100 local:vztmpl/distro-name.tar.zst --rootfs local-lvm:4 --memory 8192` : give a container a specific memory limit in megabytes
- `pct cr 100 local:vztmpl/distro-name.tar.zst --rootfs local-lvm:4 --hostname hostname --password password` : give a container a hostname and a password
- `pct cr 100 local:vztmpl/distro-name.tar.zst --rootfs local-lvm:4 --net0 name=eth0,bridge=vmbr0,ip=dhcp|manual|10.0.0.1/24 --features nesting=1` : give a container network access
- `pct cr 100 local:vztmpl/distro-name.tar.zst --rootfs local-lvm:4 --start` : start a container immediately after creation
- `pct cr 100 local:vztmpl/distro-name.tar.zst --rootfs local-lvm:4 --onboot` : set the container to start on system boot

## Categories
containers, archives, disks

## Links
- More information: https://pve.proxmox.com/pve-docs/pct.1.html#cli_pct_create
- tldr page: https://tldr.inbrowser.app/pages/linux/pct-create
