# dnf reposync

Synchronize packages and metadata of a remote DNF repository to a local directory. Not default to `dnf` but supported via `dnf-plugins-core`.

## Usage
- `dnf reposync --repoid repo_name` : synchronize all packages from the repository with id `repo_name` to a subdirectory `repo_name` of the current working directory
- `dnf reposync --repoid repo_name -p path/to/directory` : synchronize all packages and define a custom save location
- `dnf reposync --repoid repo_name --download-metadata` : synchronize all packages and metadata
- `dnf reposync --repoid repo_name -n` : download only newest packages per-repo
- `dnf reposync --repoid repo_name -u` : just print URLs of what would be downloaded, don't download
- `dnf reposync --repoid repo_name --remote-time` : try to set the timestamps of the downloaded files to those on the remote side

## Categories
packages, files, network

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/reposync.html
- tldr page: https://tldr.inbrowser.app/pages/linux/dnf-reposync
