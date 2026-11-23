# aws eks

Gestisce addon, cluster e gruppi di nodi di Amazon Elastic Kubernetes Service (EKS). Amazon EKS è un servizio per eseguire facilmente Kubernetes su AWS.

## Usage
- `aws eks create-cluster --name nome_cluster --role-arn arn_eks_service_role --resources-vpc-config subnetIds=subnet_ids,securityGroupIds=security_group_ids` : crea un cluster EKS
- `aws eks update-kubeconfig --name nome_cluster` : aggiorna kubeconfig per connettersi al cluster EKS
- `aws eks list-clusters` : elenca i cluster EKS disponibili
- `aws eks describe-cluster --name nome_cluster` : descrivi i dettagli del cluster EKS
- `aws eks delete-cluster --name nome_cluster` : elimina un cluster EKS
- `aws eks list-nodegroups --cluster-name nome_cluster` : elenca i nodegroup in un cluster EKS
- `aws eks describe-nodegroup --cluster-name nome_cluster --nodegroup-name nome_nodegroup` : descrivi i dettagli del nodegroup

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/eks/
