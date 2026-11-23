# vboxmanage-controlvm

Change the state and the settings of a currently running virtual machine.

## Usage
- `VBoxManage controlvm uuid|vm_name pause` : temporarily stop the execution of a virtual machine
- `VBoxManage controlvm uuid|vm_name resume` : resume the execution of a paused virtual machine
- `VBoxManage controlvm uuid|vm_name reset` : perform a cold reset on the virtual machine
- `VBoxManage controlvm uuid|vm_name poweroff` : power off a virtual machine with the same effect as pulling the power cable of a computer
- `VBoxManage controlvm uuid|vm_name savestate` : shut down the virtual machine and save its current state
- `VBoxManage controlvm uuid|vm_name acpipowerbutton` : send an ACPI (Advanced Configuration and Power Interface) shutdown signal to the virtual machine
- `VBoxManage controlvm uuid|vm_name reboot` : send command to reboot itself to the guest OS
- `VBoxManage controlvm uuid|vm_name shutdown` : shut down the virtual machine without saving its state

## Categories
system, network

## Links
- More information: https://www.virtualbox.org/manual/ch08.html#vboxmanage-controlvm
- tldr page: https://tldr.inbrowser.app/pages/common/vboxmanage-controlvm
