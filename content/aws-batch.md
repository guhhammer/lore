# aws batch

Run batch computing workloads through the AWS Batch service.

## Usage
- `aws batch list-jobs --job-queue queue_name` : list running batch jobs
- `aws batch create-compute-environment --compute-environment-name compute_environment_name --type type` : create compute environment
- `aws batch create-job-queue --job-queue-name queue_name --priority priority --compute-environment-order compute_environment` : create batch job queue
- `aws batch submit-job --job-name job_name --job-queue job_queue --job-definition job_definition` : submit job
- `aws batch describe-jobs --jobs jobs` : describe the list of batch jobs
- `aws batch cancel-job --job-id job_id --reason reason` : cancel job

## Categories
processes, cloud, system

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/batch/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-batch
