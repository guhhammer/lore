# limactl

Virtual machine manager for Linux guests, with multiple VM templates available. Can be used to run containers on macOS, but also for generic virtual machine use cases on macOS and Linux hosts.

## Usage
- `limactl list` : list VMs
- `limactl create --name vm_name template:debian|fedora|ubuntu|...` : create a VM using the default settings and optionally provide a name and/or a template (see `limactl create --list-templates` for available templates)
- `limactl start vm_name` : start a VM (this might install some dependencies in it and take a few minutes)
- `limactl shell vm_name` : open a remote shell inside a VM
- `limactl shell vm_name command` : run a command inside a VM
- `limactl stop vm_name` : stop/shut down a VM
- `limactl remove vm_name` : delete a VM

## Categories
containers, shell

## Links
- More information: https://github.com/lima-vm/lima
- tldr page: https://tldr.inbrowser.app/pages/common/limactl
