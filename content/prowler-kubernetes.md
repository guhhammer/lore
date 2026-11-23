# prowler kubernetes

Assess Kubernetes cluster security best practices and configurations.

## Usage
- `prowler kubernetes` : run the default checks using the default kubeconfig location
- `prowler kubernetes --kubeconfig-file path/to/kubeconfig` : specify a custom kubeconfig file for scanning
- `prowler kubernetes --context my-context` : specify a specific Kubernetes context to scan
- `prowler kubernetes --namespaces default kube-system` : scan specific namespaces only
- `prowler kubernetes -s ietcd|apiserver|...` : run checks for selected Kubernetes services
- `prowler kubernetes -c etcd_encryption` : run a specific Kubernetes check
- `prowler kubernetes -e etcd_encryption --exclude-services ietcd|apiserver|...` : exclude specific checks or services

## See also
- prowler aws
- prowler azure
- prowler gcp
- prowler m365
- prowler github

## Categories
containers, files, system

## Links
- More information: https://docs.prowler.com/user-guide/cli/tutorials/misc
- tldr page: https://tldr.inbrowser.app/pages/common/prowler-kubernetes
