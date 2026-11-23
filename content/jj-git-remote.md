# jj git remote

Manage Git remotes.

## Usage
- `jj git remote list` : list all Git remotes
- `jj git remote add remote_name remote_url` : add a Git remote
- `jj git remote add --push-url push_url remote_name remote_url` : add a Git remote with a specific push URL
- `jj git remote set-url remote_name remote_url` : change the URL of a Git remote
- `jj git remote set-url --push push_url remote_name` : set the push URL for a Git remote
- `jj git remote set-url --fetch fetch_url --push push_url remote_name` : set both fetch and push URLs for a Git remote
- `jj git remote remove remote_name` : remove a Git remote
- `jj git remote rename old_name new_name` : rename a Git remote

## Categories
version-control, network

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-git-remote
- tldr page: https://tldr.inbrowser.app/pages/common/jj-git-remote
