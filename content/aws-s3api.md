# aws s3api

Create and delete Amazon S3 buckets and edit bucket properties.

## Usage
- `aws s3api create-bucket --bucket bucket_name --region region --create-bucket-configuration LocationConstraint=region` : create bucket in a specific region
- `aws s3api delete-bucket --bucket bucket_name` : delete a bucket
- `aws s3api list-buckets` : list buckets
- `aws s3api list-objects --bucket bucket_name --query 'Contents[].{Key: Key, Size: Size}'` : list the objects inside of a bucket and only show each object's key and size
- `aws s3api put-object --bucket bucket_name --key object_key --body path/to/file` : add an object to a bucket
- `aws s3api get-object --bucket bucket_name --key object_key path/to/output_file` : download object from a bucket (The output file is always the last argument)
- `aws s3api put-bucket-policy --bucket bucket_name --policy file://path/to/bucket_policy.json` : apply an Amazon S3 bucket policy to a specified bucket
- `aws s3api get-bucket-policy --bucket bucket_name --query Policy --output json|table|text|yaml|yaml-stream >path/to/bucket_policy` : download the Amazon S3 bucket policy from a specified bucket

## Categories
cloud, files, text

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3api/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-s3api
