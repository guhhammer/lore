# linode-cli object-storage

Beheer Linode Object Storage.

## Usage
- `linode-cli object-storage buckets list` : toon alle Object Storage buckets
- `linode-cli object-storage buckets create --cluster cluster_id --label bucket_label` : maak een nieuwe Object Storage bucket
- `linode-cli object-storage buckets delete cluster_id bucket_label` : verwijder een Object Storage bucket
- `linode-cli object-storage clusters list` : toon alle Object Storage clusterregio's
- `linode-cli object-storage keys list` : toon alle access keys voor Object Storage
- `linode-cli object-storage keys create --label label` : maak een nieuwe access key voor Object Storage
- `linode-cli object-storage keys revoke access_key_id` : trek een access key voor Object Storage in
