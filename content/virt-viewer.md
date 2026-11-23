# virt-viewer

Minimal graphical interface for a virtual machine (VM). Note: `domain` refers to the name, UUID, or ID for the existing VMs.

## Usage
- `virt-viewer -c qemu:///system|qemu:///session|...` : launch `virt-viewer` with a prompt to select running virtual machines
- `virt-viewer -c uri domain` : launch `virt-viewer` for a specific virtual machine by ID, UUID, or name
- `virt-viewer -c uri -r -w domain` : wait for a virtual machine to start and automatically reconnect if it shuts down and restarts
- `virt-viewer -c qemu+tls://host/system domain` : connect to a specific remote virtual machine over TLS (requires pre-configured TLS certificates)
- `virt-viewer -c xen+ssh://username@host/system domain` : connect to a specific remote virtual machine over SSH (requires SSH access to the host)

## See also
- virsh

## Categories
network, security

## Links
- More information: https://manned.org/virt-viewer
- tldr page: https://tldr.inbrowser.app/pages/common/virt-viewer
