# pulumi schema

Check a Pulumi package schema for errors. Schema reference: <https://www.pulumi.com/docs/iac/extending-pulumi/schema/>.

## Usage
- `pulumi schema check path/to/file` : check a package schema
- `pulumi schema check --allow-dangling-references path/to/file` : check a package schema without failing if the reference to a type is missing
- `pulumi schema check -h` : display help

## Categories
network, packages, files

## Links
- More information: https://www.pulumi.com/docs/iac/cli/commands/pulumi_schema/
- tldr page: https://tldr.inbrowser.app/pages/common/pulumi-schema
- Wikipedia: https://en.wikipedia.org/wiki/Pulumi
