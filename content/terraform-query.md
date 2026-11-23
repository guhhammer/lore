# terraform query

Query remote infrastructure for resources matching criteria in `.tfquery.hcl` files.

## Usage
- `terraform query` : query infrastructure using `.tfquery.hcl` files in the current directory
- `terraform query -generate-config-out path/to/generated.tf` : query and generate import blocks and resource configuration
- `terraform query -var 'name1=value1' -var 'name2=value2'` : specify values for input variables defined in the query file
- `terraform query -var-file path/to/file.tfvars` : specify values for input variables from a file
- `terraform query -json` : output results in JSON format

## Categories
databases, files

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/query
- tldr page: https://tldr.inbrowser.app/pages/common/terraform-query
