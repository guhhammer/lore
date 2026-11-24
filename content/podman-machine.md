# podman machine

Create and manage virtual machines running Podman. Included with Podman version 4 or greater.

## Usage
- `podman machine ls` : list existing machines
- `podman machine init` : create a new default machine
- `podman machine init name` : create a new machine with a specific name
- `podman machine init --cpus 4 --memory 4096 --disk-size 50` : create a new machine with different resources
- `podman machine start|stop name` : start or stop a machine
- `podman machine ssh name` : connect to a running machine via SSH
- `podman machine inspect name` : inspect information about a machine

## Categories
containers, network

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-machine.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/podman-machine
- Wikipedia: https://en.wikipedia.org/wiki/Podman
