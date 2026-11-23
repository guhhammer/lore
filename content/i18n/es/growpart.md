# growpart

Extiende una partición en un disco o imagen de disco para llenar el espacio disponible.

## Usage
- `growpart /dev/sdX n` : extiende la partición `n` desde `sdX` para llenar el espacio vacío hasta el final del disco o el principio de la siguiente partición
- `growpart -N /ruta/a/disco.img n` : muestra qué modificaciones se harían al hacer crecer la partición `n` en una imagen de disco

## Links
- More information: https://github.com/canonical/cloud-utils
