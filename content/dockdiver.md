# dockdiver

A tool for interacting with Docker registries, including listing and dumping repositories.

## Usage
- `dockdiver -url https://example.com -list` : list all repositories in a Docker registry
- `dockdiver -url https://example.com -dump repository_name` : dump a specific repository to the default output directory (docker_dump)
- `dockdiver -url https://example.com -dump-all -username username -password password` : dump all repositories with basic authentication
- `dockdiver -url https://example.com -dump repository_name -port port -rate requests_per_second -dir path/to/output_directory` : dump a repository with a rate limit and a custom port (the default port is `5000`)
- `dockdiver -url https://example.com -dump-all -bearer bearer_token` : dump all repositories with bearer token for authorization
- `dockdiver -url https://example.com -list -headers '{"X-Custom": "Value"}'` : add custom headers as JSON (e.g., '{"X-Custom": "Value"}')

## Categories
network, containers

## Links
- More information: https://github.com/MachiavelliII/dockdiver#instructions
- tldr page: https://tldr.inbrowser.app/pages/common/dockdiver
