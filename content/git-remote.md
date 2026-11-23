# git remote

Manage set of tracked repositories ("remotes").

## Usage
- `git remote -v` : list existing remotes with their names and URLs
- `git remote show remote_name` : show information about a remote
- `git remote add remote_name remote_url` : add a remote
- `git remote set-url remote_name new_url` : change the URL of a remote (use `--add` to keep the existing URL)
- `git remote set-url remote_name new_url --push` : set push remote to be different from fetch remote
- `git remote get-url remote_name` : show the URL of a remote
- `git remote remove remote_name` : remove a remote
- `git remote rename old_name new_name` : rename a remote

## Categories
version-control, network

## Links
- More information: https://git-scm.com/docs/git-remote
- tldr page: https://tldr.inbrowser.app/pages/common/git-remote
