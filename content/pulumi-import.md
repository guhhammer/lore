# pulumi import

Import resources into an existing stack. Read syntax for your cloud provider: <https://www.pulumi.com/registry/>.

## Usage
- `pulumi import type_token name resource_id` : generate the resource definition with a given name for an existing provider resource
- `pulumi import aws:iam/user:User my_user_resource resouce_id` : import an existing AWS user as a `pulumi` resource
- `pulumi import cloudflare:index/workersScript:WorkersScript my_worker_script account_id/script_name` : import an existing Cloudflare worker
- `pulumi import --file path/to/file.json --out path/to/file` : import from a JSON file for bulk import operations and output to a file instead of `stdout`

## Categories
files, cloud, network

## Links
- More information: https://www.pulumi.com/docs/iac/cli/commands/pulumi_import/
- tldr page: https://tldr.inbrowser.app/pages/common/pulumi-import
- Wikipedia: https://en.wikipedia.org/wiki/Pulumi
