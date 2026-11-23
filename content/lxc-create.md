# lxc-create

Create linux containers.

## Usage
- `sudo lxc-create -n container_name -t download` : create a container interactively in `/var/lib/lxc/`
- `sudo lxc-create -P /path/to/directory/ -n container_name -t download` : create a container in a target directory
- `sudo lxc-create -n container_name -t download -- -d distro_name -r release_version -a architecture` : create a container passing options to a template
- `lxc-create -?` : display help

## Categories
containers, network, files

## Links
- More information: https://linuxcontainers.org/lxc/getting-started/
- tldr page: https://tldr.inbrowser.app/pages/linux/lxc-create
