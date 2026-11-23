# aws s3 rb

Delete an empty S3 bucket.

## Usage
- `aws s3 rb s3://bucket_name` : delete an empty S3 bucket
- `aws s3 rb s3://bucket_name --force` : force delete an S3 bucket and its unversioned objects (will fail if versioned objects are present)

## Categories
cloud

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/rb.html
- tldr page: https://tldr.inbrowser.app/pages/common/aws-s3-rb
