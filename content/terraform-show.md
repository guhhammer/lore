# terraform show

Read and output a Terraform state or plan file in human-readable form.

## Usage
- `terraform show` : show the current state
- `terraform show path/to/terraform.tfstate` : show a specific state file
- `terraform show path/to/file.tfplan` : show a specific plan file
- `terraform show -json` : output in JSON format
- `terraform show -json path/to/file.tfplan` : output a plan file in JSON format
- `terraform show -no-color >path/to/file` : write to a file without color codes

## Categories
files, text

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/show
- tldr page: https://tldr.inbrowser.app/pages/common/terraform-show
- Wikipedia: https://en.wikipedia.org/wiki/Terraform_(software)
