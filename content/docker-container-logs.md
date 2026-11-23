# docker container logs

Print container logs.

## Usage
- `docker logs name|id` : print logs from a container
- `docker logs name|id -f` : print logs and follow them
- `docker logs name|id -n 5` : print last 5 lines
- `docker logs name|id -t` : print logs and append them with timestamps
- `docker logs name|id --until time` : print logs from a certain point in time of container execution (i.e. 23m, 10s, 2013-01-02T13:23:37)

## Categories
containers, documents

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/logs/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-container-logs
