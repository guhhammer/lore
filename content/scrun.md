# scrun

An OCI runtime proxy for Slurm that runs containers as jobs.

## Usage
- `scrun create container_id` : create a new container with a specific ID
- `scrun start container_id` : start a previously created container
- `scrun state container_id` : query the state of a container
- `scrun kill container_id` : send a signal to a container (default: `SIGTERM`)
- `scrun kill container_id SIGKILL` : send a specific signal to a container
- `scrun delete container_id` : delete a container and release its resources
- `scrun create|start|kill|delete container_id --debug` : enable debug logging
- `scrun --version` : display version

## Categories
containers, network, processes

## Links
- More information: https://slurm.schedmd.com/scrun.html
- tldr page: https://tldr.inbrowser.app/pages/linux/scrun
