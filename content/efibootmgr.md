# efibootmgr

Manipulate the UEFI Boot Manager.

## Usage
- `efibootmgr -u` : list all boot options with their numbers
- `sudo efibootmgr -c -d /dev/sda -p 1 -l "path\to\executable.efi" -L "boot_entry_name"` : add a UEFI executable (e.g. UEFI shell, Unified Kernel Image, etc.) as a boot option
- `sudo efibootmgr -c -d /dev/sda -p 1 -l "path\to\vmlinuz" -u "root=UUID=partition_uuid rw loglevel=3 initrd=path\to\initramfs-linux.img" -L "Linux"` : add Linux as a boot option
- `sudo efibootmgr -o 0002,0008,0001,001A,...` : change the current boot order
- `sudo efibootmgr -B -b 0008` : delete a boot option

## Categories
system, users, files

## Links
- More information: https://manned.org/efibootmgr
- tldr page: https://tldr.inbrowser.app/pages/linux/efibootmgr
