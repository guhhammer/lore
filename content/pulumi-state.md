# pulumi state

Edit the current stack's state.

## Usage
- `pulumi state delete` : delete a resource from the current stack's state
- `pulumi state move resource_urn --dest stack_name` : move a resource from the current stack to another
- `pulumi state rename` : rename a resource in the current stack's state
- `pulumi state repair` : repair an invalid state
- `pulumi state edit --stack stack_name` : edit a stack's state in the editor specified by the `$EDITOR` environment variable
- `pulumi state -h` : display help

## Categories
files

## Links
- More information: https://www.pulumi.com/docs/iac/cli/commands/pulumi_state/
- tldr page: https://tldr.inbrowser.app/pages/common/pulumi-state
