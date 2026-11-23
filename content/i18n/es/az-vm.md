# az vm

Gestiona máquinas virtuales en Azure. Parte de `azure-cli` (también conocido como `az`).

## Usage
- `az vm list --output table` : muestra una tabla de Máquinas Virtuales disponibles
- `az vm create -g rg -n nombre_vm --image UbuntuLTS --admin-user usuarioazure --generate-ssh-keys` : crea una máquina virtual usando la imagen por defecto de Ubuntu y genera las claves SSH
- `az vm stop -g rg -n nombre_de_la_máquina_virtual` : detiene una Máquina Virtual
- `az vm deallocate -g rg -n nombre_de_la_máquina virtual` : desasigna una máquina virtual
- `az vm start -g rg -n nombre_de_la_máquina_virtual` : inicia una Máquina Virtual
- `az vm restart -g rg -n nombre_de_la_máquina_virtual` : reinicia una Máquina Virtual
- `az vm image list` : lista de imágenes de máquinas virtuales disponibles en Azure Marketplace

## Links
- More information: https://learn.microsoft.com/cli/azure/vm
