# kubectl run

Run pods in Kubernetes. Specifies pod generator to avoid deprecation error in some K8S versions.

## Usage
- `kubectl run nginx-dev --image nginx --port 80` : run an `nginx` pod and expose port 80
- `kubectl run nginx-dev --image nginx --env "TEST_VAR=testing"` : run an `nginx` pod, setting the `$TEST_VAR` environment variable
- `kubectl run nginx-dev --image nginx --dry-run=none|server|client` : simulate creating an `nginx` container, showing the API calls that would be made
- `kubectl run temp-ubuntu --image ubuntu:22.04 --restart Never --rm -- /bin/bash` : run an Ubuntu pod interactively, never restart it, and remove it when it exits
- `kubectl run temp-ubuntu --image ubuntu:22.04 --command -- echo argument1 argument2 ...` : run an Ubuntu pod, overriding the default command with echo, and specifying custom arguments

## Categories
containers, media, network

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_run/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-run
