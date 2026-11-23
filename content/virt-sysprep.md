# virt-sysprep

Reset, unconfigure, or customize a virtual machine image. This command can be used on a virtual machine or directly on a virtual machine disk image. Note: You may need to pass `--connect URI` to the commands or set up the URI in `$XDG_CONFIG_HOME/libvirt/libvirt.conf`.

## Usage
- `virt-sysprep --list-operations` : list all supported operations (default enabled operations are indicated with asterisks)
- `sudo virt-sysprep -d vm_name --operations operation1,operation2,...` : run only the specified operations
- `sudo virt-sysprep -a path/to/image.qcow2` : remove sensitive system data from a virtual machine image (operations marked as default)
- `sudo virt-sysprep -d vm_name -n` : simulate running all enabled operations on a virtual machine specified by its name
- `sudo virt-sysprep -d vm_name --operations machine-id,net-hwaddr,net-hostname,net-nmconn,customize --hostname new_hostname` : reset NetworkManager network configurations, persistent MAC mappings, and the hostname to avoid network conflicts
- `sudo virt-sysprep -a path/to/image.qcow2 --operations customize --root-password password:new_password` : set the root password for a disk image
- `sudo virt-sysprep -a path/to/image.qcow2 --run-command 'useradd -m username && echo username:password | chpasswd && usermod -aG sudo username'` : inject a new user with a defined password and add them to the sudo group
- `sudo virt-sysprep -a path/to/image.qcow2 --operations customize --network --install package_name1,package_name2,...` : install specific package(s) on a disk image (use `--update` to upgrade all installed packages to latest)

## Categories
users, containers, media

## Links
- More information: https://libguestfs.org/virt-sysprep.1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/virt-sysprep
