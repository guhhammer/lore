# limine-scan

This command is an alias of `limine-entry-tool --scan`.

## Usage
- `limine-entry-tool --scan` : scan for other active UEFI boot entries and add them to the Limine menu
- `limine-entry-tool --add "kernel_name" "path/to/initramfs" "path/to/vmlinuz"` : add a new kernel boot entry with an initramfs and a kernel file
- `limine-entry-tool --add-uki "kernel_name" "path/to/uki.efi"` : add a new Unified Kernel Image (UKI) boot entry
- `limine-entry-tool --remove "kernel_name"` : remove a kernel boot entry and its associated files from the ESP
- `limine-entry-tool --remove-os "os_name|machine_id"` : remove an entire OS entry by its name or machine ID
- `limine-entry-tool --add-efi "efi_entry_name" "path/to/loader.efi"` : add an EFI boot entry for an alternative bootloader (e.g., Windows)

## Categories
system, files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/limine-scan
