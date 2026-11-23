# bk

Manage Buildkite builds, pipelines, and agents.

## Usage
- `bk configure` : configure the API token and organization
- `bk use organization_slug` : select an organization to use
- `bk init` : initialize a `pipeline.yaml` file
- `bk pipeline list` : list all pipelines in the current organization
- `bk build create pipeline_slug` : trigger a build for a pipeline
- `bk build view build_number` : view the status of a specific build
- `bk agent list` : list all agents in the current organization
- `bk -h` : display help

## Categories
development

## Links
- More information: https://github.com/buildkite/cli#usage
- tldr page: https://tldr.inbrowser.app/pages/common/bk
