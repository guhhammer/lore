# aws s3 cp

Copy local files or S3 objects to another location locally or in S3.

## Usage
- `aws s3 cp path/to/file s3://bucket_name/path/to/remote_file` : copy a file from local to a specific bucket
- `aws s3 cp s3://bucket_name1/path/to/file s3://bucket_name2/path/to/target` : copy a specific S3 object into another bucket
- `aws s3 cp s3://bucket_name1/path/to/file s3://bucket_name2` : copy a specific S3 object into another bucket keeping the original name
- `aws s3 cp s3://bucket_name . --recursive` : copy S3 objects to a local directory recursively
- `aws s3 cp help` : display help

## Categories
cloud, files

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/cp.html
- tldr page: https://tldr.inbrowser.app/pages/common/aws-s3-cp
