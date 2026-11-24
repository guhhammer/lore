# pulumi destroy

Destroy all existing resources in a stack.

## Usage
- `pulumi destroy` : destroy all resources in the current stack
- `pulumi destroy -s stack` : destroy all resources in a specific stack
- `pulumi destroy -y` : automatically approve and destroy resources after previewing
- `pulumi destroy --exclude-protected` : exclude protected resources from being destroyed
- `pulumi destroy --remove` : remove the stack and its configuration file after all resources in the stack are deleted
- `pulumi destroy --continue-on-error` : continue destroying the resources, even if an error is encountered

## Links
- More information: https://www.pulumi.com/docs/iac/cli/commands/pulumi_destroy/
- tldr page: https://tldr.inbrowser.app/pages/common/pulumi-destroy
- Wikipedia: https://en.wikipedia.org/wiki/Pulumi
