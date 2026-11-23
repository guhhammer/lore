# doctl

Manage DigitalOcean resources.

## Usage
- `doctl auth init` : authenticate with DigitalOcean
- `doctl account get` : display account information
- `doctl compute droplet create name --size size --image image --region region` : create a new Droplet
- `doctl compute droplet list` : list all Droplets
- `doctl compute ssh droplet_id` : SSH into a Droplet
- `doctl kubernetes cluster create cluster_name` : create a Kubernetes cluster
- `doctl kubernetes cluster list` : list Kubernetes clusters
- `doctl databases list` : list managed databases

## Categories
containers, network

## Links
- More information: https://docs.digitalocean.com/reference/doctl/
- tldr page: https://tldr.inbrowser.app/pages/common/doctl
