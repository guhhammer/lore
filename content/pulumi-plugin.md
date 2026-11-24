# pulumi plugin

Manage language and resource provider plugins manually. Other commands manage these automatically.

## Usage
- `pulumi plugin ls` : list all plugins on the downloaded cache
- `pulumi plugin -p -j` : list plugins being used by the current project in JSON format
- `pulumi plugin install kind name version` : install a plugin kind (e.g. resource) with the latest version or a specific one
- `pulumi plugin rm kind name version` : remove a plugin kind (e.g. resource) and interactively pick a version or provide a specific one
- `pulumi plugin -h` : display help

## Links
- More information: https://www.pulumi.com/docs/iac/cli/commands/pulumi_plugin/
- tldr page: https://tldr.inbrowser.app/pages/common/pulumi-plugin
- Wikipedia: https://en.wikipedia.org/wiki/Pulumi
