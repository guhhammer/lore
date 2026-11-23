# woodpecker-cli

Manage, configure, and inspect Woodpecker CI servers, and run workflows locally.

## Usage
- `woodpecker-cli lint` : lint all workflows in the current project
- `woodpecker-cli exec path/to/workflow.yml` : execute a workflow locally
- `woodpecker-cli exec --env variable_name=variable_value path/to/workflow.yml` : execute a workflow locally with environment variables
- `woodpecker-cli exec --secrets name="value" path/to/workflow.yml` : execute a workflow locally with secrets
- `woodpecker-cli exec --secrets name1="value1",name2="value2",... path/to/workflow.yml` : execute a workflow locally with multiple secrets
- `woodpecker-cli setup` : set up the command-line client to manage a Woodpecker CI server
- `woodpecker-cli pipeline show` : show recent pipelines that have run on the remote Woodpecker CI server
- `woodpecker-cli update` : update woodpecker-cli to the latest version

## Categories
files

## Links
- More information: https://woodpecker-ci.org/docs/cli
- tldr page: https://tldr.inbrowser.app/pages/common/woodpecker-cli
