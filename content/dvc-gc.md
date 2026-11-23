# dvc gc

Remove unused files and directories from the cache or remote storage.

## Usage
- `dvc gc -w` : garbage collect from the cache, keeping only versions referenced by the current workspace
- `dvc gc -a -T -A` : garbage collect from the cache, keeping only versions referenced by branch, tags, and commits
- `dvc gc -A -c` : garbage collect from the cache, including the default cloud remote storage (if set)
- `dvc gc -A -c -r remote_name` : garbage collect from the cache, including a specific cloud remote storage

## Categories
files

## Links
- More information: https://doc.dvc.org/command-reference/gc
- tldr page: https://tldr.inbrowser.app/pages/common/dvc-gc
