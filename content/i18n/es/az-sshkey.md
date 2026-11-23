# az sshkey

Administra claves públicas SSH con máquinas virtuales. Parte de `azure-cli` (también conocido como `az`).

## Usage
- `az sshkey create --name nombre -g grupo_de_recursos` : crea una nueva clave SSH
- `az sshkey create --name nombre -g grupo_de_recursos --public-key "@ruta/de/llave.pub"` : sube una clave SSH existente
- `az sshkey list` : lista todas las claves públicas SSH
- `az sshkey show --name nombre -g grupo_de_recursos` : muestra información sobre una clave pública SSH

## Links
- More information: https://learn.microsoft.com/cli/azure/sshkey
