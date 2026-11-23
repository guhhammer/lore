# glab mr create

Manage GitLab merge requests.

## Usage
- `glab mr create` : interactively create a merge request
- `glab mr create -f` : create a merge request, determining the title and description from the commit messages of the current branch
- `glab mr create --draft` : create a draft merge request
- `glab mr create -b target_branch -t "title" -d "description"` : create a merge request specifying the target branch, title, and description
- `glab mr create -w` : start opening a merge request in the default web browser

## Categories
version-control

## Links
- More information: https://gitlab.com/gitlab-org/cli/-/blob/main/docs/source/mr/create.md
- tldr page: https://tldr.inbrowser.app/pages/common/glab-mr-create
