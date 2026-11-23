# az container

Manage Azure Container Instances. Part of `azure-cli` (also known as `az`).

## Usage
- `az container create -g resource_group -n name --image image_name -os windows|linux --cpu number_of_cpu_cores --memory memory_in_GB` : create a container in a container group
- `az container exec -g resource_group -n container_group_name --exec-command "command"` : execute a command from within a running container of a container group
- `az container logs -n name -g resource_group` : examine the logs of a container in a container group
- `az container show -n name -g resource_group` : get the details of a container group
- `az container start -n name -g resource_group` : start all containers in a container group
- `az container stop -n name -g resource_group` : stop all containers in a container group
- `az container delete -n name -g resource_group` : delete a container group

## Categories
containers, cloud, users

## Links
- More information: https://learn.microsoft.com/cli/azure/container
- tldr page: https://tldr.inbrowser.app/pages/common/az-container
