# aws s3 mv

Move local files or S3 objects to another location locally or in S3.

## Usage
- `aws s3 mv path/to/local_file s3://bucket_name/path/to/remote_file` : move a file from local to a specified bucket
- `aws s3 mv s3://bucket_name1/path/to/file s3://bucket_name2/path/to/target` : move a specific S3 object into another bucket
- `aws s3 mv s3://bucket_name1/path/to/file s3://bucket_name2` : move a specific S3 object into another bucket keeping the original name
- `aws s3 mv help` : display help

## Categories
cloud, files

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/mv.html
- tldr page: https://tldr.inbrowser.app/pages/common/aws-s3-mv
