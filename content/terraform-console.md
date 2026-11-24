# terraform console

Start an interactive console for evaluating Terraform expressions.

## Usage
- `terraform console` : start an interactive console to evaluate expressions
- `terraform console -plan` : evaluate expressions against the planned state instead of current state
- `echo "expression" | terraform console` : evaluate a specific expression non-interactively
- `terraform console -var 'name1=value1' -var 'name2=value2'` : specify values for input variables
- `terraform console -var-file path/to/file.tfvars` : specify values for input variables from a file

## Categories
files

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/console
- tldr page: https://tldr.inbrowser.app/pages/common/terraform-console
- Wikipedia: https://en.wikipedia.org/wiki/Terraform_(software)
