# aws logs

Interact with log files from various AWS services.

## Usage
- `aws logs list-log-groups` : list log groups
- `aws logs tail log_group_name --follow` : continuously poll logs of a CloudWatch log group
- `aws logs tail log_group_name --filter-pattern pattern` : tail the logs of a CloudWatch log group based on a filter
- `aws logs start-live-tail --log-group-identifiers log_group_name` : stream near real-time logs from a log group
- `aws logs create-export-task --log-group-name log_group_name --from start_time --to end_time --destination s3_bucket_name` : export logs to an S3 bucket

## Categories
cloud, monitoring, users

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/logs/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-logs
