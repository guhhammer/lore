# exo storage

Manage the Exoscale Simple Object Storage (SOS) service.

## Usage
- `exo storage mb bucket_name` : create a new SOS bucket
- `exo storage put path/to/file bucket_name/prefix/` : upload a file to a bucket
- `exo storage ls bucket_name` : list the objects within a bucket
- `exo storage get bucket_name/object_key path/to/local_file --dry-run` : simulate the download of an object from a bucket
- `exo storage metadata add bucket_name/object_key key=value` : manage the metadata of an object

## Categories
cloud, files, network

## Links
- More information: https://community.exoscale.com/product/storage/object-storage/
- tldr page: https://tldr.inbrowser.app/pages/common/exo-storage
