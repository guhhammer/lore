# pulumi policy

Manage resource policies on Pulumi Cloud (Business Critical) or locally (no organization sub-commands).

## Usage
- `pulumi policy new --dir path/to/directory template|url` : create a new Pulumi Policy Pack from a template or URL
- `pulumi policy validate-config organization_name/policy_pack_name version` : validate syntax of a policy. To check a policy against a project use `pulumi preview`
- `pulumi policy ls -j organization_name` : list all policies for an organization
- `pulumi policy publish organization_name` : publish a policy to the Pulumi Cloud
- `pulumi policy enable organization_name/policy_pack_name latest|version` : enable a policy with a specific version
- `pulumi policy disable organization_name/policy_pack_name --version version` : disable a policy with a specific version (defaults to all versions)
- `pulumi policy -h` : display help

## Categories
cloud, network

## Links
- More information: https://www.pulumi.com/docs/iac/cli/commands/pulumi_policy/
- tldr page: https://tldr.inbrowser.app/pages/common/pulumi-policy
