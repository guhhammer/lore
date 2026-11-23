# docker container exec

Execute a command on an already running Docker container.

## Usage
- `docker exec -it container_name /bin/bash` : enter an interactive shell session on an already-running container
- `docker exec -d container_name command` : run a command in the background (detached) on a running container
- `docker exec -it -w path/to/directory container_name command` : select the working directory for a given command to execute into
- `docker exec -i -d container_name command` : run a command in background on existing container but keep `stdin` open
- `docker exec -it -e variable_name=value container_name /bin/bash` : set an environment variable in a running Bash session
- `docker exec -u user container_name command` : run a command as a specific user

## Categories
containers, files

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/exec/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-container-exec
