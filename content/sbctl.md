# sbctl

A user-friendly secure boot key manager. Note: Not enrolling Microsoft's certificates can brick your system. See <https://github.com/Foxboron/sbctl/wiki/FAQ#option-rom>.

## Usage
- `sbctl status` : show the current secure boot status
- `sbctl create-keys` : create custom secure boot keys (by default, everything is stored in `/var/lib/sbctl`)
- `sbctl enroll-keys -m` : enroll the custom secure boot keys and Microsoft's UEFI vendor certificates
- `sbctl setup --setup` : automatically run `create-keys` and `enroll-keys` based on the settings in `/etc/sbctl/sbctl.conf`
- `sbctl sign -s path/to/efi_binary` : sign an EFI binary with the created key and save the file to the database
- `sbctl sign-all` : re-sign all the saved files
- `sbctl verify` : verify that all EFI executables on the EFI system partition have been signed

## Categories
security, system, network

## Links
- More information: https://github.com/Foxboron/sbctl#usage
- tldr page: https://tldr.inbrowser.app/pages/linux/sbctl
