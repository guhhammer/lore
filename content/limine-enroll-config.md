# limine-enroll-config

Embed or reset the BLAKE2B hash of `limine.conf` in the Limine EFI executable. Used to ensure the configuration file has not been tampered with when Secure Boot is enabled.

## Usage
- `limine-enroll-config path/to/BOOTX64.EFI blake2b_hash` : embed a config file's BLAKE2B hash into the Limine EFI executable
- `limine-enroll-config --reset path/to/BOOTX64.EFI` : remove the enrolled hash from the executable, disabling the config integrity check
- `limine-enroll-config --help` : display help

## Categories
security, files, system

## Links
- More information: https://codeberg.org/Limine/Limine/src/branch/trunk/USAGE.md#secure-boot
- tldr page: https://tldr.inbrowser.app/pages/linux/limine-enroll-config
