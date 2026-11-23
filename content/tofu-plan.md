# tofu plan

Generate and show OpenTofu execution plans.

## Usage
- `tofu plan` : generate and show the execution plan in the currently directory
- `tofu plan -destroy` : show a plan to destroy all remote objects that currently exist
- `tofu plan -refresh-only` : show a plan to update the Tofu state and output values
- `tofu plan -var 'name1=value1' -var 'name2=value2'` : specify values for input variables
- `tofu plan -target resource_type.resource_name` : focus Tofu's attention on only a subset of resources
- `tofu plan -json` : output a plan as JSON
- `tofu plan -no-color >path/to/file` : write a plan to a specific file

## Categories
files

## Links
- More information: https://opentofu.org/docs/cli/commands/plan/
- tldr page: https://tldr.inbrowser.app/pages/common/tofu-plan
