# qm set

Set virtual machine options.

## Usage
- `qm se 100 --name vm_name` : set a name for a VM in the GUI
- `qm se 100 --autostart 0|1` : set a VM to autostart on boot
- `qm se 100 --cores 4` : set the allotted core count of a VM
- `qm se 100 --memory 8192` : set the allotted amount of memory
- `qm se 100 --net0 virtio|e1000|rtl8139|vmxnet3,bridge=vmbr0` : give a VM a network device and bridge it to the host network
- `qm se 100 --delete device_name0,device_name1,...` : delete a device
- `qm se 100 --hostpci0 0000:00:02,x-vga=1 --bios ovmf` : passthrough a GPU device to the guest

## Categories
hardware, network

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_set
- tldr page: https://tldr.inbrowser.app/pages/linux/qm-set
