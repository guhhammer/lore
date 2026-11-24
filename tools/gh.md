# gh

gh is GitHub's command line tool: pull requests, issues, repositories and workflow runs without opening the browser.

## Tasks
### Sign in
- `gh auth login` : log in through the browser or a token
- `gh auth status` : check who you are signed in as
- `gh auth refresh -s workflow` : add a permission to your login
- `gh auth logout` : sign out
### Repositories
- `gh repo clone OWNER/REPO` : clone a repository
- `gh repo create NAME --private --source=. --push` : make a repository from the current folder
- `gh repo fork --clone` : fork the current project and clone your fork
- `gh repo view --web` : open the repository in the browser
- `gh repo list OWNER` : list an owner's repositories
### Pull requests
- `gh pr create --fill` : open a pull request from the branch's commits
- `gh pr list` : open pull requests
- `gh pr view NUMBER` : read one
- `gh pr checkout NUMBER` : switch to a pull request's branch
- `gh pr diff NUMBER` : see its changes
- `gh pr review NUMBER --approve` : approve it
- `gh pr merge NUMBER --squash --delete-branch` : squash-merge and delete the branch
- `gh pr checks` : show the status of the checks
### Issues
- `gh issue create` : open a new issue
- `gh issue list` : open issues
- `gh issue view NUMBER` : read one
- `gh issue close NUMBER` : close it
- `gh issue comment NUMBER -b "text"` : add a comment
### Runs and releases
- `gh run list` : recent workflow runs
- `gh run watch` : follow a run live
- `gh run view RUN_ID --log-failed` : logs of the failed steps
- `gh run rerun RUN_ID --failed` : rerun the failed jobs
- `gh workflow run NAME.yml` : start a workflow by hand
- `gh release create v1.0.0 --generate-notes` : publish a release
### Anything else
- `gh api repos/OWNER/REPO` : call the GitHub API directly
- `gh alias set co 'pr checkout'` : make a shortcut

## Notes
- Run it inside a repository and it knows which one you mean; elsewhere add `-R OWNER/REPO`.
- `gh auth login` stores a token in your keyring: treat the machine like a password.
- Add `--json FIELDS --jq EXPR` to most list commands for script-friendly output.
- Install it from the official page in Links or your package manager.

## See also
- git
- hub
- glab

## Links
- Manual: https://cli.github.com/manual/
- Install: https://github.com/cli/cli#installation

## Categories
development cloud
