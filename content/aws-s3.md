# aws s3

CLI for AWS S3 - provides storage through web services interfaces. Some subcommands such as `cp` have their own usage documentation.

## Usage
- `aws s3 ls bucket_name` : show files in a bucket
- `aws s3 sync path/to/directory s3://bucket_name` : sync files in a directory from local to bucket
- `aws s3 sync s3://bucket_name path/to/target` : sync files and directories from bucket to local
- `aws s3 sync path/to/directory s3://bucket_name --exclude path/to/file --exclude path/to/directory/*` : sync files in a directory with exclusions
- `aws s3 rm s3://bucket/path/to/file` : remove file from bucket
- `aws s3 any_command --dryrun` : simulate a command without making any changes

## Categories
cloud, files, documents

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-s3
