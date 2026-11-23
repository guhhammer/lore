# linode-cli nodebalancers

Manage Linode NodeBalancers.

## Usage
- `linode-cli nodebalancers list` : list all NodeBalancers
- `linode-cli nodebalancers create --region region` : create a new NodeBalancer
- `linode-cli nodebalancers view nodebalancer_id` : view details of a specific NodeBalancer
- `linode-cli nodebalancers update nodebalancer_id --label new_label` : update an existing NodeBalancer
- `linode-cli nodebalancers delete nodebalancer_id` : delete a NodeBalancer
- `linode-cli nodebalancers configs list nodebalancer_id` : list configurations for a NodeBalancer
- `linode-cli nodebalancers configs create nodebalancer_id --port port --protocol protocol` : add a new configuration to a NodeBalancer

## Categories
network

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/linode-cli-nodebalancers
