# aws s3 ls

List AWS S3 buckets, folders (prefixes), and files (objects).

## Usage
- `aws s3 ls` : list all buckets
- `aws s3 ls s3://bucket_name` : list files and folders in the root of a bucket (`s3://` is optional)
- `aws s3 ls bucket_name/path/to/directory/` : list files and folders directly inside a directory
- `aws s3 ls --recursive bucket_name` : list all files in a bucket
- `aws s3 ls --recursive bucket_name/path/to/directory/prefix` : list all files in a path with a given prefix
- `aws s3 ls help` : display help

## Categories
cloud, files

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/ls.html
- tldr page: https://tldr.inbrowser.app/pages/common/aws-s3-ls
