# pulumi refresh

Refresh the resources in a stack.

## Usage
- `pulumi refresh` : compare the current stack's state with the state in the cloud provider and adopt any changes into the current stack
- `pulumi refresh --diff` : refresh resources in the current stack and show the operation as a rich diff
- `pulumi refresh --expect-no-changes` : refresh resources in the current stack and return an error if any changes occur during the refresh
- `pulumi refresh --preview-only` : only show a preview of the refresh, but don't perform the refresh itself
- `pulumi refresh -s stack_name` : specify the name of the stack to operate on (defaults to the current stack)
- `pulumi refresh -h` : display help

## Links
- More information: https://www.pulumi.com/docs/iac/cli/commands/pulumi_refresh/
- tldr page: https://tldr.inbrowser.app/pages/common/pulumi-refresh
