# terraform workspace

Manage Terraform workspaces for multiple environments.

## Usage
- `terraform workspace list` : list all workspaces
- `terraform workspace show` : show the name of the current workspace
- `terraform workspace new workspace_name` : create a new workspace
- `terraform workspace select workspace_name` : select an existing workspace
- `terraform workspace select -or-create workspace_name` : select a workspace, creating it if it doesn't exist
- `terraform workspace delete workspace_name` : delete a workspace
- `terraform workspace delete -force workspace_name` : delete a workspace, even if it is tracking resources

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/workspace
- tldr page: https://tldr.inbrowser.app/pages/common/terraform-workspace
