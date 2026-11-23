# podman machine

Crea y gestiona máquinas virtuales ejecutando Podman. Desde la versión 4 o superior de Podman.

## Usage
- `podman machine ls` : lista máquinas existentes
- `podman machine init` : crea una nueva máquina predeterminada
- `podman machine init nombre` : crea una nueva máquina con un nombre específico
- `podman machine init --cpus 4 --memory 4096 --disk-size 50` : crea una nueva máquina con diferentes recursos
- `podman machine start|stop nombre` : inicia o detiene una máquina
- `podman machine ssh nombre` : conecta a una máquina en ejecución a través de SSH
- `podman machine inspect nombre` : inspecciona información sobre una máquina

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-machine.1.html
