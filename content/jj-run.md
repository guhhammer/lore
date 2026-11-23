# jj run

Run a command across a set of revisions, amending each with the resulting changes. Each revision is checked out into an isolated working copy before the command runs.

## Usage
- `jj run -- command` : run a command on the current revision
- `jj run -r revset -- command` : run a command across a revset
- `jj run -r 'trunk()..@' -- cargo fmt` : run a formatter across all commits in a stack
- `jj run -r revset -- sh -c 'echo $JJ_CHANGE_ID'` : print the change ID of each revision in a revset (`$JJ_CHANGE_ID` is set by `jj run` for each revision)
- `jj run -r revset -j num_jobs -- command` : run a command in parallel across multiple revisions
- `jj run --ignore-changes -r revset -- command` : simulate running a command without rewriting any commits
- `jj run --restore-descendants -r revset -- command` : run a command while keeping descendants' content unchanged
- `jj run -r revset --root -- command` : run a command from the workspace root instead of each revision's root

## Categories
files

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-run
- tldr page: https://tldr.inbrowser.app/pages/common/jj-run
