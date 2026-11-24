# terraform init

Initialize a new or existing Terraform working directory.

## Usage
- `terraform init` : initialize the current working directory
- `terraform init -upgrade` : initialize and upgrade modules and providers to the latest allowed versions
- `terraform init -reconfigure` : initialize and reconfigure the backend, ignoring any saved configuration
- `terraform init -migrate-state` : initialize and reconfigure the backend, attempting to migrate any existing state
- `terraform init -backend-config 'key=value'` : initialize with additional backend configuration
- `terraform init -backend=false` : initialize without backend or HCP Terraform initialization
- `terraform init -input=false` : initialize without interactive prompts (useful for automation)
- `terraform init -lockfile readonly` : initialize with the dependency lockfile mode set to readonly

## Categories
files

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/init
- tldr page: https://tldr.inbrowser.app/pages/common/terraform-init
- Wikipedia: https://en.wikipedia.org/wiki/Terraform_(software)
