# git lfs

Work with large files in Git repositories.

## Usage
- `git lfs install` : initialize Git LFS
- `git lfs track '*.bin'` : track files that match a `glob`
- `git config -f .lfsconfig lfs.url lfs_endpoint_url` : change the Git LFS endpoint URL (useful if the LFS server is separate from the Git server)
- `git lfs track` : list tracked patterns
- `git lfs ls-files` : list tracked files that have been committed
- `git lfs push --all remote_name branch_name` : push all Git LFS objects to the remote server (useful if errors are encountered)
- `git lfs fetch` : fetch all Git LFS objects
- `git lfs checkout` : replace pointer files with actual Git LFS objects

## Categories
version-control, network

## Links
- More information: https://github.com/git-lfs/git-lfs/tree/main/docs
- tldr page: https://tldr.inbrowser.app/pages/common/git-lfs
