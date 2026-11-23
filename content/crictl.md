# crictl

Manage CRI-compatible container runtimes.

## Usage
- `crictl pods` : list all Kubernetes pods (Ready and NotReady)
- `crictl ps -a` : list all containers (Running and Exited)
- `crictl images` : list all images
- `crictl inspect container_id1 container_id2 ...` : print information about specific containers
- `crictl exec -it container_id sh` : open a specific shell inside a running container
- `crictl pull image:tag` : pull a specific image from a registry
- `crictl logs -f container_id` : print and follow logs of a specific container
- `crictl rmi image_id1 image_id2 ...` : remove one or more images

## Categories
containers, media

## Links
- More information: https://github.com/kubernetes-sigs/cri-tools/blob/master/docs/crictl.md
- tldr page: https://tldr.inbrowser.app/pages/common/crictl
