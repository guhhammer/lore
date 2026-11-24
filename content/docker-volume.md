# docker volume

Manage Docker volumes.

## Usage
- `docker volume create volume_name` : create a volume
- `docker volume create --label label volume_name` : create a volume with a specific label
- `docker volume create -o type=tmpfs -o device=tmpfs -o o=size=100m,uid=1000 volume_name` : create a `tmpfs` volume a size of 100 MiB and a uid of 1000
- `docker volume ls` : list all volumes
- `docker volume rm volume_name` : remove a volume
- `docker volume inspect volume_name` : display information about a volume
- `docker volume prune` : remove all unused local volumes
- `docker volume subcommand --help` : display help for a subcommand

## Categories
disks, containers

## Links
- More information: https://docs.docker.com/reference/cli/docker/volume/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-volume
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
