# bosh

Deploy and manage the BOSH director.

## Usage
- `bosh alias-env environment_name -e ip_address|url --ca-cert ca_certificate` : create local alias for director in a specific environment
- `bosh environments` : list environments
- `bosh login -e environment` : log in to the director
- `bosh -e environment deployments` : list deployments
- `bosh -e environment vms -d deployment` : list environment virtual machines in a deployment
- `bosh -e environment ssh virtual_machine -d deployment` : SSH into virtual machine
- `bosh -e environment upload-stemcell stemcell_file|url` : upload stemcell
- `bosh -e environment cloud-config` : show current cloud config

## Categories
network

## Links
- More information: https://bosh.io/docs/cli-v2/
- tldr page: https://tldr.inbrowser.app/pages/common/bosh
