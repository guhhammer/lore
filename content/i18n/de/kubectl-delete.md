# kubectl delete

Lösche Kubernetes-Ressourcen.

## Usage
- `kubectl delete po pod_name` : lösche einen bestimmten Pod
- `kubectl delete deploy deployment_name` : lösche ein bestimmtes Deployment
- `kubectl delete no node_name` : lösche eine bestimmte Node
- `kubectl delete po --all -n namespace` : lösche alle Pods in einem bestimmten Namespaces
- `kubectl delete deploy,svc --all -n namespace` : lösche alle Deployments und Services in einem bestimmten Namespace
- `kubectl delete no --all` : lösche alle Nodes
- `kubectl delete -f pfad/zu/manifest.yaml` : lösche Resourcen, die in einer YAML Datei definiert sind

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_delete/
