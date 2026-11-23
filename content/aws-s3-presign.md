# aws s3 presign

Generate pre-signed URLs for Amazon S3 objects.

## Usage
- `aws s3 presign s3://bucket_name/path/to/file` : generate a pre-signed URL for a specific S3 object that is valid for one hour
- `aws s3 presign s3://bucket_name/path/to/file --expires-in duration_in_seconds` : generate a pre-signed URL valid for a specific lifetime
- `aws s3 presign help` : display help

## Categories
cloud, files, network

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/presign.html
- tldr page: https://tldr.inbrowser.app/pages/common/aws-s3-presign
