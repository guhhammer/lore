# docker context

Switch between contexts to manage multiple Docker environments.

## Usage
- `docker context create context_name --docker "host=tcp://remote-host:2375"` : create a context using a specific Docker endpoint
- `docker context create context_name` : create a context based on the `$DOCKER_HOST` environment variable
- `docker context use context_name` : switch to a context
- `docker context ls` : list all contexts

## Categories
containers, network

## Links
- More information: https://docs.docker.com/reference/cli/docker/context/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-context
