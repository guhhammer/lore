# pulumi stack

Manage stacks and view stack state.

## Usage
- `pulumi stack init stack_name` : create a new stack
- `pulumi stack -u` : show the stack state along with resource URNs
- `pulumi stack ls` : list stacks in the current project
- `pulumi stack ls -a` : list stacks across all projects
- `pulumi stack select stack_name` : select an active stack
- `pulumi stack rm stack_name` : delete a stack
- `pulumi stack output --show-secrets` : show stack outputs, including secrets, in plaintext
- `pulumi stack export --file path/to/file.json` : export the stack state to a JSON file

## Categories
files

## Links
- More information: https://www.pulumi.com/docs/iac/cli/commands/pulumi_stack/
- tldr page: https://tldr.inbrowser.app/pages/common/pulumi-stack
- Wikipedia: https://en.wikipedia.org/wiki/Pulumi
