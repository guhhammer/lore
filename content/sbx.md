# sbx

Create and manage isolated microVM sandboxes for running AI coding agents.

## Usage
- `sbx` : open the interactive TUI dashboard
- `sbx run agent path/to/directory` : create and run an agent session inside a sandbox
- `sbx ls` : list all sandboxes
- `sbx exec -it sandbox_name bash` : open an interactive shell inside a running sandbox
- `sbx stop sandbox_name1 sandbox_name2 ...` : stop one or more running sandboxes
- `sbx rm sandbox_name1 sandbox_name2 ...` : remove one or more sandboxes
- `sbx policy allow network hostname|ip_address` : allow network access to a specific hostname or IP address for all sandboxes
- `sbx secret set secret_name` : store a secret globally for all sandboxes

## Categories
network

## Links
- More information: https://docs.docker.com/reference/cli/sbx/
- tldr page: https://tldr.inbrowser.app/pages/common/sbx
