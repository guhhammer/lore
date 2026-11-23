# swtpm_bios

BIOS simulation tool for swtpm.

## Usage
- `swtpm_bios [OPTIONS]` : the general form, as the manual gives it
- `swtpm_bios --unix <path>` : connect to the given UnixIO path
- `swtpm_bios -tpm2` : the device is a TPM 2
- `swtpm_bios -n` : don't send a TPM_Startup command
- `swtpm_bios -o` : only send the startup command and nothing else

## Categories
hardware
