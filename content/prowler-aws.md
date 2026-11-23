# prowler aws

Assess AWS security best practices, perform audits, compliance checks, and generate reports.

## Usage
- `prowler aws` : run the default set of checks on the AWS account
- `prowler aws -p custom_profile -f us-east-1 eu-south-2 ...` : use a custom AWS profile and filter audited regions
- `prowler aws -s s3|ec2|...` : run checks for selected AWS services
- `prowler aws -c s3_bucket_public_access` : run a specific AWS check
- `prowler aws -e s3_bucket_public_access --exclude-services s3|ec2|...` : exclude specific checks or services

## See also
- prowler azure
- prowler gcp
- prowler kubernetes
- prowler m365
- prowler github

## Categories
cloud, system

## Links
- More information: https://docs.prowler.com/user-guide/cli/tutorials/misc
- tldr page: https://tldr.inbrowser.app/pages/common/prowler-aws
