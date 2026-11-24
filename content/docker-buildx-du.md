# docker buildx du

See disk usage for a builder.

## Usage
- `docker buildx du` : show disk usage
- `docker buildx du --filter "description~=golang"` : filter output based on a specific condition
- `docker buildx du --verbose` : show verbose output
- `docker buildx du --format "table \{\{.ID\}\}    \{\{.Description\}\}"` : format the output using a Go template
- `docker buildx du --format json | jq .` : pretty print output as JSON with `jq` command
- `docker buildx du --builder builder_name` : inspect the disk usage of a specific builder

## Categories
containers, disks, monitoring

## Links
- More information: https://docs.docker.com/reference/cli/docker/buildx/du/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-buildx-du
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
