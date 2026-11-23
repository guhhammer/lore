# kernelstub

Automatically manage Linux kernel loading on (U)EFI.

## Usage
- `sudo kernelstub` : configure the current kernel with default options
- `sudo kernelstub -o "quiet splash mitigations=off"` : add custom kernel options
- `sudo kernelstub -v -l` : run in verbose mode with `systemd-boot` configuration
- `sudo kernelstub -m` : copy kernel only to ESP without NVRAM changes
- `sudo kernelstub -c` : simulate the operation without making any changes
- `sudo kernelstub -k /path/to/vmlinuz -i /path/to/initrd.img -e /path/to/efi_partition` : specify paths manually
- `sudo kernelstub -p` : show current configuration

## Categories
system, users, files

## Links
- More information: https://github.com/isantop/kernelstub#usage
- tldr page: https://tldr.inbrowser.app/pages/linux/kernelstub
