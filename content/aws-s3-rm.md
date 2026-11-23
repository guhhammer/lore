# aws s3 rm

Delete S3 objects.

## Usage
- `aws s3 rm s3://bucket_name/path/to/file` : delete a specific S3 object
- `aws s3 rm s3://bucket_name/path/to/file --dryrun` : simulate the deletion of a specific S3 object without deleting it
- `aws s3 rm s3://arn:aws:s3:region:account_id:access_point/access_point_name/object_key` : delete an object from a specific S3 access point
- `aws s3 rm s3://bucket_name --recursive` : remove all objects from a bucket (empty the bucket)
- `aws s3 rm help` : display help

## Categories
cloud, files

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/rm.html
- tldr page: https://tldr.inbrowser.app/pages/common/aws-s3-rm
