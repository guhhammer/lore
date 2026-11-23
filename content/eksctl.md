# eksctl

The official CLI for Amazon EKS.

## Usage
- `eksctl create cluster` : create a basic cluster
- `eksctl get cluster --name=name --region=region` : list the details about a cluster or all of the clusters
- `eksctl create cluster --config-file=path/to/file` : create a cluster passing all configuration information in a file
- `eksctl create cluster --config-file=path/to/file --without-nodegroup` : create a cluster using a configuration file and skip creating nodegroups until later
- `eksctl delete cluster --name=name --region=region` : delete a cluster
- `eksctl create cluster --name=name --nodes=4 --kubeconfig=path/to/config.yaml` : create cluster and write cluster credentials to a file other than the default
- `eksctl create cluster --name=name --nodes=4 --write-kubeconfig=false` : create a cluster and prevent storing cluster credentials locally
- `eksctl create cluster --name=name --nodes=4 --auto-kubeconfig` : create a cluster and let `eksctl` manage cluster credentials under the `~/.kube/eksctl/clusters` directory

## Categories
containers, files

## Links
- More information: https://docs.aws.amazon.com/eks/latest/eksctl/what-is-eksctl.html
- tldr page: https://tldr.inbrowser.app/pages/common/eksctl
