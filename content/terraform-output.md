# terraform output

Export structured data about your Terraform resources.

## Usage
- `terraform output` : display all outputs for the root module
- `terraform output name` : output only a value with specific name
- `terraform output -raw` : convert the output value to a raw string (useful for shell scripts)
- `terraform output -json` : format the outputs as a JSON object, with a key per output (useful with jq)

## Categories
text

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/output
- tldr page: https://tldr.inbrowser.app/pages/common/terraform-output
