# dkms

Um framework que permite recompilação dinâmica de modulos do kernel.

## Usage
- `dkms status` : lista os módulos instalados atualmente
- `sudo dkms autoinstall` : recompila todos os módulos para o kernel que está rodando atualmente
- `sudo dkms install -m acpi_call -v 1.2.1` : instala a versão 1.2.1 do módulo acpi_call para o kernel que está rodando atualmente
- `sudo dkms remove -m acpi_call -v 1.2.1 --all` : remove a versão 1.2.1 do módulo acpi_call de todos os kernels

## Links
- More information: https://manned.org/dkms
