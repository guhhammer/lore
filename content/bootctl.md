# bootctl

Control EFI firmware boot settings and manage boot loader.

## Usage
- `bootctl` : show information about the system firmware and the bootloaders
- `bootctl list` : show all available bootloader entries
- `sudo bootctl set-default entry_id` : set a boot entry as the default to boot to
- `sudo bootctl reboot-to-firmware true` : set a flag to boot into the system firmware on the next boot (similar to `sudo systemctl reboot --firmware-setup`)
- `bootctl --esp-path /path/to/efi_system_partition/` : specify the path to the EFI system partition (defaults to `/efi/`, `/boot/`, or `/boot/efi`)
- `sudo bootctl install` : install `systemd-boot` into the EFI system partition
- `sudo bootctl remove` : remove all installed versions of `systemd-boot` from the EFI system partition

## See also
- kernel-install

## Categories
system, hardware, users

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/bootctl.html
- tldr page: https://tldr.inbrowser.app/pages/linux/bootctl
