# kubectl explain

Display the documentation of a Kubernetes API resource, including available fields and descriptions.

## Usage
- `kubectl explain pods|nodes|deployments|...` : show documentation for a resource
- `kubectl explain pod.spec.containers` : show documentation for a sub-resource or field of an object
- `kubectl explain ingress.v1.networking.k8s.io` : show documentation for a specific versioned resource
- `kubectl explain po --recursive` : show all fields recursively for a resource
- `kubectl explain --help` : display help

## Categories
containers, documents, desktop

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_explain/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-explain
