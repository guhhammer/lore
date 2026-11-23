# kubetail

Utility to tail multiple Kubernetes pod logs at the same time.

## Usage
- `kubetail app_name` : tail the logs of multiple pods (whose name starts with `app_name`) in one go
- `kubetail app_name -c container_name` : tail only a specific container from multiple pods
- `kubetail app_name -c container_name_1 -c container_name_2` : tail multiple containers from multiple pods
- `kubetail app_name_1,app_name_2,...` : tail multiple applications at the same time separate them by comma

## Categories
containers

## Links
- More information: https://github.com/johanhaleby/kubetail
- tldr page: https://tldr.inbrowser.app/pages/common/kubetail
