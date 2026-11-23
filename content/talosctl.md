# talosctl

Interact with Talos Linux, a minimal and immutable Kubernetes distribution.

## Usage
- `talosctl apply-config -i -n control_plane_ip -f path/to/control_plane.yaml` : apply a config to a fresh node
- `talosctl bootstrap -n node_ip` : bootstrap the `etcd` cluster on a node
- `talosctl edit resource_to_edit -n node_ip` : edit an API resource
- `talosctl get resource_to_get -n node_ip` : get resources
- `talosctl kubeconfig -n node_ip` : download the admin kube configuration from a node
- `talosctl reset -n node_ip` : reset a node

## See also
- kubectl

## Categories
containers, network

## Links
- More information: https://docs.siderolabs.com/talos/latest/reference/cli
- tldr page: https://tldr.inbrowser.app/pages/common/talosctl
