# kubectl debug

Debug cluster resources using interactive debugging containers.

## Usage
- `kubectl debug pod_name -it --image busybox` : create an interactive debugging session in a pod and immediately attach to it
- `kubectl debug --image image -c container_name pod_name` : create a debug container with a custom image and name
- `kubectl debug node/node_name -it --image busybox` : create an interactive debugging session on a node and immediately attach to it (the container will run in the host namespaces and the host's filesystem will be mounted at `/host`)
- `kubectl debug pod_name -it --image image --copy-to pod_copy_name` : create a copy of a pod and add a debug container to it
- `kubectl debug pod_name -it --copy-to pod_copy_name --container container_name -- command` : create a copy of a pod and change the command of a specific container
- `kubectl debug pod_name --copy-to pod_copy_name --set-image container_name=image` : create a copy of a pod and change the image of a specific container
- `kubectl debug pod_name --copy-to pod_copy_name --set-image '*=image'` : create a copy of a pod and change all container images
- `kubectl debug pod_name -it --image image --target target_container_name` : create an ephemeral debug container and target a specific container (useful for debugging distroless containers)

## Categories
containers, development, media

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_debug/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-debug
