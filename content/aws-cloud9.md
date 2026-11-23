# aws cloud9

Manage Cloud9 - a collection of tools to code, build, run, test, debug, and release software in the cloud.

## Usage
- `aws cloud9 list-environments` : list all Cloud9 development environment identifiers
- `aws cloud9 create-environment-ec2 --name name --instance-type instance_type` : create a Cloud9 development environment
- `aws cloud9 describe-environments --environment-ids environment_ids` : display information about Cloud9 development environments
- `aws cloud9 create-environment-membership --environment-id environment_id --user-arn user_arn --permissions permissions` : add an environment member to a Cloud9 development environment
- `aws cloud9 describe-environment-status --environment-id environment_id` : display status information for a Cloud9 development environment
- `aws cloud9 delete-environment --environment-id environment_id` : delete a Cloud9 environment
- `aws cloud9 delete-environment-membership --environment-id environment_id --user-arn user_arn` : delete an environment member from a development environment

## Categories
cloud, development

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/cloud9/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-cloud9
