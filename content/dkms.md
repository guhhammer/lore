# dkms

A framework that allows for dynamic building of kernel modules.

## Usage
- `dkms status` : list currently installed modules
- `sudo dkms autoinstall` : rebuild all modules for the currently running kernel
- `sudo dkms install -m acpi_call -v 1.2.1` : install version 1.2.1 of the acpi_call module for the currently running kernel
- `sudo dkms remove -m acpi_call -v 1.2.1 --all` : remove version 1.2.1 of the acpi_call module from all kernels

## Categories
system, users

## Links
- More information: https://manned.org/dkms
- tldr page: https://tldr.inbrowser.app/pages/linux/dkms
