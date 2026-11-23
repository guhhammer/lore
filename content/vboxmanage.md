# vboxmanage

Interface with VirtualBox. Includes all the functionality of the GUI and more. Some subcommands such as `startvm` have their own usage documentation.

## Usage
- `VBoxManage list vms` : list all VirtualBox virtual machines
- `VBoxManage showvminfo name|uuid` : show information about a particular virtual machine
- `VBoxManage startvm name|uuid` : start a virtual machine
- `VBoxManage startvm name|uuid --type headless` : start a virtual machine in headless mode
- `VBoxManage controlvm name|uuid savestate` : shut down the virtual machine and save its current state
- `VBoxManage controlvm name|uuid poweroff` : shut down the virtual machine without saving its state
- `VBoxManage extpack install --replace VboxExtensionPackFileName` : update VBox extension packs
- `VBoxManage --help` : display help

## Categories
network, documents, monitoring

## Links
- More information: https://www.virtualbox.org/manual/ch08.html#vboxmanage-intro
- tldr page: https://tldr.inbrowser.app/pages/common/vboxmanage
