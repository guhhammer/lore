# dkms

Un framework che permette la compilazione dinamica di moduli del kernel.

## Usage
- `dkms status` : elenca i moduli attualmente installati
- `sudo dkms autoinstall` : ricompila tutti i moduli per il kernel in esecuzione
- `sudo dkms install -m acpi_call -v 1.2.1` : installa la versione 1.2.1 del modulo acpi_call per il kernel in esecuzione
- `sudo dkms remove -m acpi_call -v 1.2.1 --all` : rimuovi la versione 1.2.1 del modulo acpi_call da tutti i kernel

## Links
- More information: https://manned.org/dkms
