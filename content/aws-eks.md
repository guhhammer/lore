# aws eks

Manage Amazon Elastic Kubernetes Service (EKS) addons, clusters, and node groups. Amazon EKS is a service for easily running Kubernetes on AWS.

## Usage
- `aws eks create-cluster --name cluster_name --role-arn eks_service_role_arn --resources-vpc-config subnetIds=subnet_ids,securityGroupIds=security_group_ids` : create an EKS Cluster
- `aws eks update-kubeconfig --name cluster_name` : update kubeconfig to connect to the EKS Cluster
- `aws eks list-clusters` : list available EKS clusters
- `aws eks describe-cluster --name cluster_name` : describe EKS cluster details
- `aws eks delete-cluster --name cluster_name` : delete an EKS Cluster
- `aws eks list-nodegroups --cluster-name cluster_name` : list nodegroups in an EKS cluster
- `aws eks describe-nodegroup --cluster-name cluster_name --nodegroup-name nodegroup_name` : describe nodegroup details

## Categories
containers, cloud, system

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/eks/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-eks
