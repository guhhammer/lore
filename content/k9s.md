# k9s

View and manage Kubernetes clusters.

## Usage
- `k9s --context kubeconfig_context_name` : manage a cluster using a kubeconfig context
- `k9s --readonly --cluster cluster_name` : manage a cluster in read-only mode (disabling all commands that may cause modifications)
- `k9s -n kubernetes_namespace --cluster cluster_name` : manage a cluster using a given Kubernetes namespace
- `k9s -c pod -l debug --cluster cluster_name` : manage a cluster launching k9s in the pod view and enable debug logging

## Categories
containers

## Links
- More information: https://k9scli.io/topics/commands/
- tldr page: https://tldr.inbrowser.app/pages/common/k9s
