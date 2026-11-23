# aws s3 sync

Recursively sync files and directories between your local system and an S3 bucket, or between S3 buckets.

## Usage
- `aws s3 sync path/to/directory s3://bucket_name/path/to/remote_location` : sync files in a directory from local to a bucket
- `aws s3 sync s3://bucket_name/path/to/remote_location path/to/directory` : sync files in a directory from a bucket to local
- `aws s3 sync s3://bucket_source_name/path/to/remote_location s3://bucket_target_name/path/to/remote_location` : sync objects between two buckets
- `aws s3 sync path/to/directory s3://bucket_name/path/to/remote_location --exclude path/to/file --exclude path/to/directory/*` : sync local files to S3 while excluding specific files or directories
- `aws s3 sync s3://bucket_source_name/path/to/remote_location s3://bucket_target_name/path/to/remote_location --delete` : sync objects between buckets and delete destination files not in source
- `aws s3 sync path/to/local_directory s3://bucket_name/path/to/remote_location --acl private|public-read --storage-class STANDARD_IA|GLACIER` : sync to S3 with advanced options (set ACL and storage class)
- `aws s3 sync path/to/directory s3://bucket_name/path/to/remote_location --size-only` : sync files to S3 and skip unchanged ones (compare size and modification time)
- `aws s3 sync help` : display help

## Categories
cloud, files

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/sync.html
- tldr page: https://tldr.inbrowser.app/pages/common/aws-s3-sync
