# hub

A wrapper for Git that adds commands for working with GitHub-based projects. If set up as instructed by `hub alias`, one can use `git` to run `hub` commands.

## Usage
- `hub clone username/repo_name` : clone a repository using its slug (owners can omit the username)
- `hub fork` : create a fork of the current repository (cloned from another user) under your GitHub profile
- `hub push remote_name && hub pull-request` : push the current local branch to GitHub and create a PR for it in the original repository
- `hub pull-request --no-edit` : create a PR of the current (already pushed) branch, reusing the message from the first commit
- `hub pr checkout pr_number` : create a new branch with the contents of a pull request and switch to it
- `hub create` : upload the current (local-only) repository to your GitHub account
- `hub sync` : fetch Git objects from upstream and update local branches

## Categories
version-control, packages, network

## Links
- More information: https://hub.github.com/hub.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/hub
