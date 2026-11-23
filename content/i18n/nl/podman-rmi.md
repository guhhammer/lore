# podman rmi

Verwijder OCI/Docker-images.

## Usage
- `podman rmi image1:tag image2:tag ...` : verwijder een of meer images op basis van hun naam
- `podman rmi -f image` : forceer het verwijderen van een image
- `podman rmi --no-prune image` : verwijder een image zonder ongetagde ouders te verwijderen
- `podman rmi -h` : toon de help

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-rmi.1.html
