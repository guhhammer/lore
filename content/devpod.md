# devpod

Launch reproducible development environments using Docker, Kubernetes, or SSH.

## Usage
- `devpod provider add provider_name` : add a provider such as Docker or Kubernetes
- `devpod provider list-available` : list all available providers
- `devpod up github.com/user/repo -i vscode` : start a workspace from a GitHub repository with a specific IDE
- `devpod up path/to/project` : start a workspace from a local directory
- `devpod up workspace_name -r` : recreate an existing workspace
- `devpod up workspace_name -x` : reset a workspace to a clean state
- `devpod provider add org/provider-repo` : add a custom provider from a GitHub repository

## Categories
containers, network

## Links
- More information: https://devpod.sh/docs/quickstart/devpod-cli/
- tldr page: https://tldr.inbrowser.app/pages/common/devpod
