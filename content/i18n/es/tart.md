# tart

Crea, ejecuta y gestiona máquinas virtuales (VM) macOS y Linux en Apple Silicon.

## Usage
- `tart pull acme.io/org/name:tag` : extrae una imagen de VM remota
- `tart clone source-vm vm-name` : clona una VM desde una fuente de imagen local o remota
- `tart create --from-ipsw latest|ruta/al/archivo.ipsw vm-name` : crea una nueva Mac VM a partir de un archivo ipsw específico
- `tart run vm-name` : ejecuta una máquina virtual existente
- `tart run --dir ruta/al/directorio:/ruta/al/directorio_local vm-name` : ejecuta una máquina virtual existente con un directorio específico montado
- `tart list` : lista máquinas virtuales
- `tart ip vm-name` : obtén la dirección IP de una máquina virtual en ejecución
- `tart set vm-name --display 640x400` : cambia la resolución de pantalla de una máquina virtual

## Links
- More information: https://github.com/cirruslabs/tart
