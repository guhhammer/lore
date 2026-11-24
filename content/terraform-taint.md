# terraform taint

Mark a resource instance as not fully functional to force replacement on next apply. Note: This command is deprecated, use `terraform apply -replace` instead.

## Usage
- `terraform taint resource_type.resource_name[instance_index]` : mark a resource as tainted
- `terraform taint -allow-missing resource_type.resource_name[instance_index]` : mark a resource as tainted, even if the resource is missing
- `terraform taint -lock=false resource_type.resource_name[instance_index]` : mark a resource as tainted without acquiring a state lock
- `terraform taint -lock-timeout=duration resource_type.resource_name[instance_index]` : mark a resource as tainted with a custom lock timeout

## See also
- terraform apply
- terraform untaint

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/taint
- tldr page: https://tldr.inbrowser.app/pages/common/terraform-taint
- Wikipedia: https://en.wikipedia.org/wiki/Terraform_(software)
