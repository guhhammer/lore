# glab mr

Manage GitLab merge requests. Some subcommands such as `create` have their own usage documentation.

## Usage
- `glab mr create` : create a merge request
- `glab co mr_number` : check out a specific merge request locally
- `glab co mr_number -R repo_owner/repository` : check out a merge request of a specific repo
- `glab mr diff` : view the changes made in the merge request
- `glab mr approve` : approve the merge request for the current branch
- `glab mr merge` : merge the merge request associated with the current branch interactively
- `glab mr update` : edit a merge request interactively
- `glab mr update --target-branch branch_name` : edit the target branch of a merge request

## Categories
version-control, documents, monitoring

## Links
- More information: https://gitlab.com/gitlab-org/cli/-/blob/main/docs/source/mr/_index.md
- tldr page: https://tldr.inbrowser.app/pages/common/glab-mr
