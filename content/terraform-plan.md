# terraform plan

Generate and show Terraform execution plans.

## Usage
- `terraform plan` : generate and show the execution plan in the current directory
- `terraform plan -destroy` : show a plan to destroy all remote objects that currently exist
- `terraform plan -refresh-only` : show a plan to update the Terraform state and output values
- `terraform plan -var 'name1=value1' -var 'name2=value2'` : specify values for input variables
- `terraform plan -target resource_type.resource_name[instance_index]` : focus Terraform's attention on only a subset of resources
- `terraform plan -json` : output a plan as JSON
- `terraform plan -no-color >path/to/file` : write a plan to a specific file

## Categories
files

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/plan
- tldr page: https://tldr.inbrowser.app/pages/common/terraform-plan
