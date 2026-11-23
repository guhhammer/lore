# krunvm

Utilidad basada en CLI para crear micro máquinas virtuales utilizando imágenes OCI.

## Usage
- `krunvm create docker.io/fedora --cpus número_de_vcpus --mem memoria_en_megabytes --name "nombre"` : crea una micro máquina virtual basada en Fedora
- `krunvm start "nombre"` : inicia una imagen específica
- `krunvm list` : lista las imágenes existentes
- `krunvm changevm --cpus número_de_vcpus --mem memoria_en_megabytes --name "nuevo_nombre" "nombre"` : cambia una imagen específica
- `krunvm delete "nombre"` : borra una imagen específica

## Links
- More information: https://github.com/containers/krunvm/blob/main/docs/krunvm.1.txt
