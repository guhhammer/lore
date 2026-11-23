# sbpctl

Manage the systemd-boot-password boot manager, a systemd-boot fork with a password-protected editor.

## Usage
- `sudo sbpctl install /path/to/efi_system_partition` : install the boot manager into the EFI system partition
- `sudo sbpctl install -d /path/to/efi_system_partition` : install the boot manager only as the default EFI loader (`/EFI/BOOT/BOOT*.EFI`)
- `sudo sbpctl install -i /path/to/efi_system_partition` : install the boot manager with `/etc/sbp/loader.conf` included in the EFI binary
- `sudo sbpctl install -s /path/to/efi_system_partition` : install the boot manager and sign it for Secure Boot with the keys in `/etc/sbp`
- `sbpctl generate` : generate a SHA-512 hash of a password for the `password` option in `loader.conf`
- `sudo sbpctl standalone -i path/to/initramfs.img path/to/vmlinuz path/to/output.efi` : create a standalone EFI application from a Linux EFI application and an initramfs

## See also
- bootctl

## Categories
system, users, packages

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/sbpctl
