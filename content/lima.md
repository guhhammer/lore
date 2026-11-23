# lima

This command is an alias of `limactl shell` for the default VM instance. You can also set the `$LIMA_INSTANCE` environment variable to work on a different instance.

## Usage
- `limactl list` : list VMs
- `limactl create --name vm_name template:debian|fedora|ubuntu|...` : create a VM using the default settings and optionally provide a name and/or a template (see `limactl create --list-templates` for available templates)
- `limactl start vm_name` : start a VM (this might install some dependencies in it and take a few minutes)
- `limactl shell vm_name` : open a remote shell inside a VM
- `limactl shell vm_name command` : run a command inside a VM
- `limactl stop vm_name` : stop/shut down a VM
- `limactl remove vm_name` : delete a VM

## Categories
shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/lima
