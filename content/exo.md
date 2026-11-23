# exo

Manage the Exoscale services. Some subcommands such as `compute` have their own usage documentation.

## Usage
- `exo config` : configure the exo command-line
- `exo completion zsh` : generate the exo autocompletion script for a specified shell
- `exo zone -O json` : list all of the available zones and output them as JSON
- `exo compute instance create instance_name --zone zone -Q` : quietly create a Compute instance in a specific zone (disables the non-essential command output)
- `exo storage list --output-template '\{\{ .Name \}\}'` : list just the name of all of the buckets in the Organization
- `exo iam -h` : display help for a specific sub-command

## Categories
documents, monitoring, system

## Links
- More information: https://community.exoscale.com/reference/cli/exo/storage/list/index.md
- tldr page: https://tldr.inbrowser.app/pages/common/exo
