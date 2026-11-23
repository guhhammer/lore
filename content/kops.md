# kops

Create, destroy, upgrade, and maintain Kubernetes clusters.

## Usage
- `kops create cluster -f cluster_name.yaml` : create a cluster from the configuration specification
- `kops create sshpublickey key_name -i ~/.ssh/id_rsa.pub` : create a new SSH public key
- `kops export kubecfg cluster_name` : export the cluster configuration to the `~/.kube/config` file
- `kops get cluster cluster_name -o yaml` : get the cluster configuration as YAML
- `kops delete cluster cluster_name -y` : delete a cluster
- `kops validate cluster cluster_name --wait wait_time_until_ready --count num_required_validations` : validate a cluster

## Categories
containers, network, packages

## Links
- More information: https://kops.sigs.k8s.io/cli/kops/
- tldr page: https://tldr.inbrowser.app/pages/common/kops
