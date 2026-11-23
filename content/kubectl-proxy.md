# kubectl proxy

Create a proxy server or application-level gateway between localhost and the Kubernetes API server.

## Usage
- `kubectl proxy` : run a proxy using the default settings on port 8001 and listen on localhost
- `kubectl proxy -w path/to/static_dir -P /static_prefix/ --api-prefix /api_subset/` : proxy part of the Kubernetes API while serving static files from a local directory
- `kubectl proxy --api-prefix /custom_prefix/` : proxy the entire Kubernetes API under a custom prefix
- `kubectl proxy -p port -w path/to/static_dir` : serve the Kubernetes API on a specific port while also serving static content
- `kubectl proxy -p 0` : run a proxy on a random local port, printing the chosen port to `stdout`
- `kubectl proxy -u path/to/socket` : run the proxy on a Unix domain socket instead of a TCP port
- `kubectl proxy --address 0.0.0.0 --accept-hosts '.*'` : accept connections from remote hosts by listening on all interfaces (use caution when exposing the proxy publicly)
- `kubectl proxy --accept-paths '^/api/v1/namespaces/default/.*' --reject-paths '^/api/.*/pods/.*/exec'` : allow only selected API paths while rejecting sensitive endpoints

## Categories
network, containers, files

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_proxy/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-proxy
