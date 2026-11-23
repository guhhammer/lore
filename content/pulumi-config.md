# pulumi config

Manage configuration of a Pulumi stack.

## Usage
- `pulumi config -j` : view current configuration in JSON format
- `pulumi config -s stack_name` : view configuration for a specified stack
- `pulumi config get key` : get the value of a configuration key
- `pulumi config rm key` : remove a configuration value
- `cat path/to/file | pulumi config set key` : set a value for a configuration key from a file
- `pulumi config set --secret key S3cr37_value` : set a secret value (e.g. API key) for a configuration key and store/display as ciphertext
- `pulumi config --config-file path/to/file rm-all key1 key2 ...` : remove multiple configuration values from a specified configuration file

## Categories
files

## Links
- More information: https://www.pulumi.com/docs/iac/cli/commands/pulumi_config/
- tldr page: https://tldr.inbrowser.app/pages/common/pulumi-config
