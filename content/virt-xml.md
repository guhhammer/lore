# virt-xml

Edit libvirt Domain XML files with explicit command-line options. Note: `domain` refers to the name, UUID, or ID for the existing VMs.

## Usage
- `virt-xml --option=?` : list all the suboptions for a specific option
- `virt-xml --disk=? --network=? --boot=?` : list all the suboptions for disk, network, and boot
- `virt-xml domain --edit --option suboption=new_value` : edit a value for a specific domain
- `virt-xml domain --edit --metadata description="new_description"` : change the description for a specific domain
- `virt-xml domain --edit --boot bootmenu=on|off` : enable/Disable the boot device menu for a specific domain
- `virt-xml domain --update --add-device --hostdev bus.device` : attach host USB hub to a running VM (run `lsusb` to see a list of USB devices with their IDs)

## See also
- virsh

## Categories
text, hardware, network

## Links
- More information: https://github.com/virt-manager/virt-manager/blob/main/man/virt-xml.rst
- tldr page: https://tldr.inbrowser.app/pages/linux/virt-xml
