# aws sts

Security Token Service (STS) allows IAM users or federated users to request temporary credentials.

## Usage
- `aws sts assume-role --role-arn aws_role_arn` : get temporary security credentials to access specific AWS resources
- `aws sts get-caller-identity` : get an IAM user or role whose credentials are used to call the operation

## Categories
cloud, users, system

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/sts/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-sts
