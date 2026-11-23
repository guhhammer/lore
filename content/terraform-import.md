# terraform import

Import existing infrastructure into Terraform state.

## Usage
- `terraform import resource_type.resource_name resource_id` : import an existing resource into Terraform state
- `terraform import module.module_name.resource_type.resource_name resource_id` : import a resource into a module
- `terraform import -input=false resource_type.resource_name resource_id` : import without interactive input prompts (useful for automation)
- `terraform import -var 'name=value' resource_type.resource_name resource_id` : import with variable values
- `terraform import -var-file=path/to/variables.tfvars resource_type.resource_name resource_id` : import with a variable file
- `terraform import -lock=false resource_type.resource_name resource_id` : import without state locking (use with caution)

## Categories
files

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/import
- tldr page: https://tldr.inbrowser.app/pages/common/terraform-import
