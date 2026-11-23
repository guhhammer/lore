# s3cmd

Upload, retrieve, and manage data in S3 compatible object storage.

## Usage
- `s3cmd --configure` : invoke configuration/reconfiguration tool
- `s3cmd ls s3://bucket|path/to/file` : list Buckets/Folders/Objects
- `s3cmd mb s3://bucket` : create Bucket/Folder
- `s3cmd get s3://bucket_name/path/to/file path/to/local_file` : download a specific file from a bucket
- `s3cmd put local_file s3://bucket/file` : upload a file to a bucket
- `s3cmd mv s3://src_bucket/src_object s3://dst_bucket/dst_object` : move an object to a specific bucket location
- `s3cmd rm s3://bucket/object` : delete a specific object

## Categories
cloud, files, network

## Links
- More information: https://s3tools.org/s3cmd
- tldr page: https://tldr.inbrowser.app/pages/common/s3cmd
