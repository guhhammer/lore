# devcontainer

Use a Docker container as a development environment.

## Usage
- `devcontainer up` : create and run a Dev Container
- `devcontainer templates apply -t template_id -a template_args -w path/to/workspace` : apply a Dev Container Template to a workspace
- `devcontainer exec command` : execute a command on a running Dev Container in the current workspace
- `devcontainer build path/to/workspace` : build a Dev Container image from `devcontainer.json`
- `devcontainer read-configuration` : read and print the configuration of a Dev Container from `devcontainer.json`

## Categories
containers

## Links
- More information: https://containers.dev/implementors/reference/
- tldr page: https://tldr.inbrowser.app/pages/common/devcontainer
