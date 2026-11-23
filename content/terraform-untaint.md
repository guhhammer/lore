# terraform untaint

Remove the tainted status from a resource instance.

## Usage
- `terraform untaint resource_type.resource_name[instance_index]` : remove tainted status from a resource
- `terraform untaint -allow-missing resource_type.resource_name[instance_index]` : remove tainted status, even if the resource is missing
- `terraform untaint -lock=false resource_type.resource_name[instance_index]` : remove tainted status without acquiring a state lock
- `terraform untaint -lock-timeout=duration resource_type.resource_name[instance_index]` : remove tainted status with a custom lock timeout

## See also
- terraform taint

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/untaint
- tldr page: https://tldr.inbrowser.app/pages/common/terraform-untaint
