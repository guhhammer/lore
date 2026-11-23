# terraform providers

Manage provider dependencies for Terraform configurations.

## Usage
- `terraform providers` : show providers required by the configuration
- `terraform providers lock` : update the dependency lock file for all providers
- `terraform providers lock -platform=linux_amd64 -platform=darwin_arm64` : update the lock file for multiple target platforms
- `terraform providers mirror path/to/mirror_directory` : mirror all required providers to a local directory
- `terraform providers mirror -platform=linux_amd64 path/to/mirror_directory` : mirror providers for a specific platform
- `terraform providers schema -json` : print provider schemas in JSON format
- `terraform providers lock -fs-mirror=path/to/mirror` : generate lock file entries using a filesystem mirror
- `terraform providers lock -net-mirror=https://mirror.example.com/providers/` : generate lock file entries using a network mirror

## Categories
files, network, packages

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/providers
- tldr page: https://tldr.inbrowser.app/pages/common/terraform-providers
