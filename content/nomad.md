# nomad

Distributed, highly available, datacenter-aware scheduler.

## Usage
- `nomad node status` : show the status of nodes in the cluster
- `nomad job validate path/to/file.nomad` : validate a job file
- `nomad job plan path/to/file.nomad` : plan a job for execution on the cluster
- `nomad job run path/to/file.nomad` : run a job on the cluster
- `nomad job status` : show the status of jobs currently running on the cluster
- `nomad job status job_name` : show the detailed status information about a specific job
- `nomad alloc logs alloc_id` : follow the logs of a specific allocation
- `nomad volume status` : show the status of storage volumes

## Categories
processes, files, containers

## Links
- More information: https://developer.hashicorp.com/nomad/commands
- tldr page: https://tldr.inbrowser.app/pages/common/nomad
