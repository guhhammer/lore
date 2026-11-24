# terraform validate

Validate the configuration files in a directory.

## Usage
- `terraform validate` : validate the configuration in the current directory
- `terraform validate -json` : output the validation result in JSON format
- `terraform validate -no-tests` : validate the configuration without validating test files
- `terraform validate path/to/directory` : validate the configuration in a specific directory
- `terraform validate -test-directory path/to/test_directory` : validate the configuration using a custom test directory

## Categories
files, development

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/validate
- tldr page: https://tldr.inbrowser.app/pages/common/terraform-validate
- Wikipedia: https://en.wikipedia.org/wiki/Terraform_(software)
