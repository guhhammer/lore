# kubectl patch

Patch Kubernetes resources with new values.

## Usage
- `kubectl patch secrets secret_name -p '{"metadata":{"finalizers": []\}\}' --type merge` : partially patch a secret using a strategic merge JSON patch to remove the finalizer
- `kubectl patch secrets secret_name -p $'metadata:\n finalizers: []' --type merge` : partially patch a secret using a strategic merge YAML patch to remove the finalizer
- `kubectl patch po pod_name --type 'json' -p '[{"op": "replace", "path": "/spec/containers/0/image", "value":"new_image_value"}]'` : partially patch a pod's container using a JSON patch with positional arrays
- `kubectl patch deploy deployment_name --subresource 'scale' --type 'merge' -p '{"spec":{"replicas":number_of_replicas\}\}'` : update a deployment's replicas through the scale subresource using a strategic merge JSON patch

## Categories
version-control, containers, text

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_patch/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-patch
