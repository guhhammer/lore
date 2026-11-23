# vboxmanage-startvm

Start a virtual machine.

## Usage
- `VBoxManage startvm vm_name|uuid` : start a virtual machine
- `VBoxManage startvm vm_name|uuid --type headless|gui|sdl|separate` : start a virtual machine with the specified UI mode
- `VBoxManage startvm vm_name|uuid --password path/to/password_file` : specify a password file to start an encrypted virtual machine
- `VBoxManage startvm vm_name|uuid --password-id password_id` : specify a password ID to start an encrypted virtual machine
- `VBoxManage startvm vm_name|uuid -E name=value` : start a virtual machine with an environment variable pair name value

## Categories
users

## Links
- More information: https://www.virtualbox.org/manual/ch08.html#vboxmanage-startvm
- tldr page: https://tldr.inbrowser.app/pages/common/vboxmanage-startvm
