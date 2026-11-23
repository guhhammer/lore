# doctl vpcs

Manage DigitalOcean VPC networks.

## Usage
- `doctl vpcs list` : list VPC networks
- `doctl vpcs create --name vpc_name --region nyc1 --ip-range 10.116.0.0/20` : create a VPC network with a specific name, region, and IP range
- `doctl vpcs get vpc_id` : retrieve details of a specific VPC network
- `doctl vpcs update vpc_id --name new_name` : update the name of a specific VPC network
- `doctl vpcs delete vpc_id` : delete a VPC network

## Categories
network

## Links
- More information: https://docs.digitalocean.com/reference/doctl/reference/vpcs/
- tldr page: https://tldr.inbrowser.app/pages/common/doctl-vpcs
