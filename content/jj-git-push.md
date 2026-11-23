# jj git push

Push to a Git remote.

## Usage
- `jj git push -b bookmark --remote remote` : push a bookmark to a given remote
- `jj git push -b bookmark -N` : push a new bookmark
- `jj git push --tracked` : push all tracked bookmarks
- `jj git push --all` : push all bookmarks (including new bookmarks)
- `jj git push -r revset` : push all bookmarks pointing to given revisions
- `jj git push -c revset` : push changes/commits by creating new bookmarks (Name format is as per `templates.git_push_bookmark` setting, defaults to `"push-" ++ change_id.short()`)
- `jj git push --named name=revision` : create a bookmark with a given name for a revision and push it to the remote

## Categories
version-control

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-git-push
- tldr page: https://tldr.inbrowser.app/pages/common/jj-git-push
