# podman ps

List Podman containers.

## Usage
- `podman ps` : list currently running Podman containers
- `podman ps -a` : list all Podman containers (running and stopped)
- `podman ps -l` : show the latest created container (includes all states)
- `podman ps -f "name=name"` : filter containers that contain a substring in their name
- `podman ps -f "ancestor=image:tag"` : filter containers that share a given image as an ancestor
- `podman ps -a -f "exited=code"` : filter containers by exit status code
- `podman ps -f "status=created|running|removing|paused|exited|dead"` : filter containers by status
- `podman ps -f "volume=path/to/directory" --format "table \{\{.ID\}\}\t\{\{.Image\}\}\t\{\{.Names\}\}\t\{\{.Mounts\}\}"` : filter containers that mount a specific volume or have a volume mounted in a specific path

## Categories
containers, disks, files

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-ps.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/podman-ps
- Wikipedia: https://en.wikipedia.org/wiki/Podman
