# kubectl cp

Copy files and directories between a local filesystem and a container in a pod.

## Usage
- `kubectl cp path/to/local_file pod_name:/path/to/file_in_pod` : copy a local file or directory to a pod
- `kubectl cp path/to/local_file namespace/pod_name:/path/to/file_in_pod` : copy a local file or directory to a pod in a specific namespace
- `kubectl cp namespace/pod_name:/path/to/file_in_pod path/to/local_file` : copy a file or directory from a pod to the local machine
- `kubectl cp path/to/local_file pod_name:/path/to/file_in_pod -c container_name` : copy a file or directory to a specific container in a pod
- `kubectl cp path/to/local_file namespace/pod_name:/path/to/file_in_pod --no-preserve` : copy a file or directory to a pod without preserving ownership and permissions
- `kubectl cp path/to/local_file pod_name:/path/to/file_in_pod --retries 3` : copy a local file or directory to a pod with retries on failure

## Categories
files, containers, disks

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_cp/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-cp
