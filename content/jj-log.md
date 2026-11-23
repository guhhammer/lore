# jj log

Show revision history as a graph.

## Usage
- `jj log` : show revision history as a graph
- `jj log -r revsets` : show only given revsets (e.g. `B::D`, `A..D`, `B|C|D`, etc.)
- `jj log -T 'commit_id.shortest(5) ++ " " ++ author'` : show log with a custom template (e.g. the unique commit hash prefix of at least 5 characters and the author)

## Categories
monitoring, shell

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-log
- tldr page: https://tldr.inbrowser.app/pages/common/jj-log
