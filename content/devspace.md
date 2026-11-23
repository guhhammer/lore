# devspace

Develop, deploy, and debug applications inside Kubernetes.

## Usage
- `devspace init` : initialize a new DevSpace project in the current directory
- `devspace dev` : start development mode with port forwarding, file synchronization, and terminal access
- `devspace dev -n namespace` : start development mode in a specific namespace
- `devspace deploy` : deploy the project to Kubernetes
- `devspace deploy -p profile_name` : deploy the project with a specific profile
- `devspace build` : build all defined images
- `devspace logs -f` : follow logs from a pod
- `devspace ui` : open the DevSpace UI in the browser

## Categories
development, containers, network

## Links
- More information: https://www.devspace.sh/docs/cli
- tldr page: https://tldr.inbrowser.app/pages/common/devspace
