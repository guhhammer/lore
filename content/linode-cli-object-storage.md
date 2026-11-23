# linode-cli object-storage

Manage Linode Object Storage.

## Usage
- `linode-cli object-storage buckets list` : list all Object Storage buckets
- `linode-cli object-storage buckets create --cluster cluster_id --label bucket_label` : create a new Object Storage bucket
- `linode-cli object-storage buckets delete cluster_id bucket_label` : delete an Object Storage bucket
- `linode-cli object-storage clusters list` : list Object Storage cluster regions
- `linode-cli object-storage keys list` : list access keys for Object Storage
- `linode-cli object-storage keys create --label label` : create a new access key for Object Storage
- `linode-cli object-storage keys revoke access_key_id` : revoke an access key for Object Storage

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/linode-cli-object-storage
