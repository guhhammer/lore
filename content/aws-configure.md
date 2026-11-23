# aws configure

Manage configuration for the AWS CLI.

## Usage
- `aws configure` : configure AWS CLI interactively (creates a new configuration or updates the default)
- `aws configure --profile profile_name` : configure a named profile for AWS CLI interactively (creates a new profile or updates an existing one)
- `aws configure get name` : display the value from a specific configuration variable
- `aws configure get name --profile profile_name` : display the value for a configuration variable in a specific profile
- `aws configure set name value` : set the value of a specific configuration variable
- `aws configure set name value --profile profile_name` : set the value of a configuration variable in a specific profile
- `aws configure list` : list the configuration entries
- `aws configure list --profile profile_name` : list the configuration entries for a specific profile

## Categories
cloud

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/configure/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-configure
