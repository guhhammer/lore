# aws glue

CLI for AWS Glue. Define the public endpoint for the AWS Glue service.

## Usage
- `aws glue list-jobs` : list jobs
- `aws glue start-job-run --job-name job_name` : start a job
- `aws glue start-workflow-run --name workflow_name` : start running a workflow
- `aws glue list-triggers` : list triggers
- `aws glue start-trigger --name trigger_name` : start a trigger
- `aws glue create-dev-endpoint --endpoint-name name --role-arn role_arn_used_by_endpoint` : create a dev endpoint

## Categories
cloud, processes, system

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/glue/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-glue
