# conda export

Export environment details.

## Usage
- `conda export` : export the current environment details to `stdout`
- `conda export -f path/to/environment.yaml` : export the current environment details to a `YAML` file
- `conda export --format environment-json|environment-yaml|explicit|json|reqs|requirements|txt|yaml|yml` : export details in a specific format
- `conda export -n environment_name` : target an environment by name
- `conda export -p path/to/environment` : target an environment by its path
- `conda export -c channel_name` : include a specific channel

## Categories
text, files

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/export.html
- tldr page: https://tldr.inbrowser.app/pages/common/conda-export
