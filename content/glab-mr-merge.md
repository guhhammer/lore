# glab mr merge

Merge GitLab merge requests.

## Usage
- `glab mr merge` : merge the merge request associated with the current branch interactively
- `glab mr merge mr_number` : merge the specified merge request, interactively
- `glab mr merge -d` : merge the merge request, removing the branch on both the local and the remote
- `glab mr merge -s -m "commit_message_body"` : squash the current merge request into one commit with the message body and merge
- `glab mr merge --help` : display help

## Categories
version-control

## Links
- More information: https://gitlab.com/gitlab-org/cli/-/blob/main/docs/source/mr/merge.md
- tldr page: https://tldr.inbrowser.app/pages/common/glab-mr-merge
