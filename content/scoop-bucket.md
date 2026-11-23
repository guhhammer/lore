# scoop bucket

Manage buckets: Git repositories containing files which describe how scoop installs applications. If Scoop doesn't know where the bucket is located its repository location must be specified.

## Usage
- `scoop bucket list` : list all buckets currently in use
- `scoop bucket known` : list all known buckets
- `scoop bucket add name` : add a known bucket by its name
- `scoop bucket add name https://example.com/repository.git` : add an unknown bucket by its name and Git repository URL
- `scoop bucket rm name` : remove a bucket by its name

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
cloud, packages, version-control, windows

## Links
- More information: https://github.com/ScoopInstaller/Scoop/wiki/Buckets
- tldr page: https://tldr.inbrowser.app/pages/windows/scoop-bucket
