# docker container update

Update configuration of Docker containers. Note: This command is not supported for Windows containers.

## Usage
- `docker update --restart always|no|on-failure|unless-stopped container_name` : update restart policy to apply when a specific container exits
- `docker update --restart on-failure:3 container_name` : update the policy to restart up to three times a specific container when it exits with non-zero exit status
- `docker update --cpus count container_name` : update the number of CPUs available to a specific container
- `docker update -m limitM container_name` : update the memory limit in Megabytes for a specific container
- `docker update --pids-limit count container_name` : update the maximum number of process IDs allowed inside a specific container (use `-1` for unlimited)
- `docker update --memory-swap limitM container_name` : update the amount of memory in Megabytes a specific container can swap to disk (use `-1` for unlimited)

## Categories
containers, disks, monitoring

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/update/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-container-update
