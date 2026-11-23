# podman machine

Maak virtuele machines aan waarop Podman draait en beheer deze. Inbegrepen vanaf Podman versie 4.

## Usage
- `podman machine ls` : toon bestaande machines
- `podman machine init` : maak een nieuwe standaardmachine aan
- `podman machine init naam` : maak een nieuwe machine aan met een specifieke naam
- `podman machine init --cpus 4 --memory 4096 --disk-size 50` : maak een nieuwe machine aan met andere resources
- `podman machine start|stop naam` : start of stop een machine
- `podman machine ssh naam` : maak verbinding met een draaiende machine via SSH
- `podman machine inspect naam` : bekijk informatie over een machine

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-machine.1.html
