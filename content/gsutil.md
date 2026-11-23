# gsutil

Access Google Cloud Storage. You can use `gsutil` to do a wide range of bucket and object management tasks.

## Usage
- `gsutil ls` : list all buckets in a project you are logged into
- `gsutil ls -r 'gs://bucket_name/prefix**'` : list the objects in a bucket
- `gsutil cp gs://bucket_name/object_name path/to/save_location` : download an object from a bucket
- `gsutil cp object_location gs://destination_bucket_name/` : upload an object to a bucket
- `gsutil mv gs://bucket_name/old_object_name gs://bucket_name/new_object_name` : rename or move objects in a bucket
- `gsutil mb gs://bucket_name` : create a new bucket in the project you are logged into
- `gsutil rm -r gs://bucket_name` : delete a bucket and remove all the objects in it

## Categories
cloud, network, files

## Links
- More information: https://docs.cloud.google.com/storage/docs/gsutil
- tldr page: https://tldr.inbrowser.app/pages/common/gsutil
