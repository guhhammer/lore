# aws

The official CLI tool for Amazon Web Services. Some subcommands such as `s3` have their own usage documentation.

## Usage
- `aws configure wizard` : configure the AWS Command-line
- `aws configure sso` : configure the AWS Command-line using SSO
- `aws sts get-caller-identity` : get the caller identity (used to troubleshoot permissions)
- `aws dynamodb list-tables --region us-east-1 --output yaml` : list AWS resources in a region and output in YAML
- `aws iam create-user --cli-auto-prompt` : use auto prompt to help with a command
- `aws dynamodb wizard new_table` : get an interactive wizard for an AWS resource
- `aws dynamodb update-table --generate-cli-skeleton` : generate a JSON CLI Skeleton (useful for infrastructure as code)
- `aws command help` : display help for a specific command

## Categories
cloud

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/
- tldr page: https://tldr.inbrowser.app/pages/common/aws
