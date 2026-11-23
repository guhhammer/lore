# kopia

Fast, secure open-source backup tool. Supports encryption, compression, deduplication, and incremental snapshots.

## Usage
- `kopia repository create filesystem --path path/to/local_repository` : create a repository in the local filesystem
- `kopia repository create s3 --bucket bucket_name --access-key AWS_access_key_id --secret-access-key AWS_secret_access_key` : create a repository on Amazon S3
- `kopia repository connect repository_type --path path/to/repository` : connect to a repository
- `kopia snapshot create path/to/directory` : create a snapshot of a directory
- `kopia snapshot list` : list snapshots
- `kopia snapshot restore snapshot_id path/to/target_directory` : restore a snapshot to a specific directory
- `kopia policy set --global --keep-latest number_of_snapshots_to_keep --compression compression_algorithm` : create a new policy
- `kopia policy set --global --add-ignore path/to/file_or_folder` : ignore a specific file or folder from backups

## Categories
files, packages, cloud

## Links
- More information: https://kopia.io/docs/reference/command-line/
- tldr page: https://tldr.inbrowser.app/pages/common/kopia
