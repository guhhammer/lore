# docker cannot connect to the daemon

"Cannot connect to the Docker daemon" or "permission denied" on docker.sock: start the service, or let your user use it.

## Steps
- `systemctl status docker` : is the service running?
- `sudo systemctl enable --now docker` : start it, and at every boot
- `ls -l /var/run/docker.sock` : who may use the socket: root and the docker group
- `sudo usermod -aG docker $USER` : join the docker group, then log out and in
- `docker info` : check it works

## Notes
- The docker group is as good as root: only add users you trust.
- macOS and Windows: start Docker Desktop.

## See also
- docker
- systemctl
- usermod

## Categories
containers
