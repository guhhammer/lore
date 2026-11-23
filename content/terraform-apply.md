# terraform apply

Create or update infrastructure according to Terraform configuration files.

## Usage
- `terraform apply` : create or update infrastructure
- `terraform apply -auto-approve` : create or update infrastructure, skipping interactive approval
- `terraform apply path/to/file.tfplan` : apply a plan file
- `terraform apply -var 'name1=value1' -var 'name2=value2'` : specify values for input variables
- `terraform apply -var-file path/to/file.tfvars` : specify values for input variables from a file
- `terraform apply -target resource_type.resource_name[instance_index]` : apply changes to a specific resource
- `terraform apply -replace resource_type.resource_name[instance_index]` : replace a specific resource
- `terraform apply -destroy` : destroy Terraform-managed infrastructure

## Categories
files

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/apply
- tldr page: https://tldr.inbrowser.app/pages/common/terraform-apply
