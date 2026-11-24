# terraform destroy

Destroy all objects managed by a Terraform configuration.

## Usage
- `terraform destroy` : destroy all infrastructure in the current directory
- `terraform destroy -auto-approve` : destroy infrastructure, skipping interactive approval
- `terraform destroy -target resource_type.resource_name[instance_index]` : destroy a specific resource
- `terraform destroy -var 'name1=value1' -var 'name2=value2'` : specify values for input variables
- `terraform destroy -var-file path/to/file.tfvars` : specify values for input variables from a file
- `terraform destroy -compact-warnings` : destroy infrastructure with compact warnings

## Categories
files

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/destroy
- tldr page: https://tldr.inbrowser.app/pages/common/terraform-destroy
- Wikipedia: https://en.wikipedia.org/wiki/Terraform_(software)
