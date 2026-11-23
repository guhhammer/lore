# ventoy2disk

A tool to create bootable USB drives using ISO files on Windows systems.

## Usage
- `Ventoy2Disk VTOYCLI /I /Drive:D:` : install Ventoy to drive D: with default settings (MBR, Secure Boot enabled)
- `Ventoy2Disk VTOYCLI /I /Drive:D: /GPT /NOSB` : install Ventoy with GPT partition style and disable Secure Boot
- `Ventoy2Disk VTOYCLI /I /Drive:D: /R:4096` : install Ventoy and reserve 4GB space at the end of the disk
- `Ventoy2Disk VTOYCLI /I /PhyDrive:1 /FS:NTFS` : install Ventoy using physical drive number 1 with NTFS filesystem
- `Ventoy2Disk VTOYCLI /I /Drive:D: /NOUSBCheck` : install Ventoy without USB type checking (for internal drives)
- `Ventoy2Disk VTOYCLI /U /Drive:D:` : update Ventoy on drive D: while keeping current settings
- `Ventoy2Disk VTOYCLI /I /Drive:D: /NonDest` : perform non-destructive installation (preserve existing data)
- `Ventoy2Disk VTOYCLI /I /Drive:D: /GPT /NOSB /R:2048 /FS:NTFS /NOUSBCheck` : install Ventoy with all options: GPT, no Secure Boot, 2GB reserved, NTFS, no USB check

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
disks, packages, hardware, windows

## Links
- More information: https://www.ventoy.net/en/doc_windows_cli.html
- tldr page: https://tldr.inbrowser.app/pages/windows/ventoy2disk
