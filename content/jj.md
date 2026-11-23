# jj

Jujutsu, a version control system. Some subcommands such as `log`, `describe`, `new`, `git`, etc. have their own usage documentation.

## Usage
- `jj desc -m "message" -r revsets` : update description of the revisions specified by given revsets (e.g. `B::D`, `A..D`, `B|C|D`, etc.)
- `jj new revset` : create a new commit/revision on top of a given revision
- `jj new revset1 revset2 ...` : create a new merge commit on top of multiple revisions
- `jj edit revset` : update the working copy to point to a revision
- `jj undo` : undo the previous command (which may itself have been `undo`)
- `jj --ignore-working-copy subcommand` : execute a jj subcommand without snapshotting the working copy
- `jj --at-op operation subcommand` : execute a jj subcommand at an operation
- `jj help subcommand` : display help for a specific subcommand (like `new`, `commit`, `desc`, etc.)

## Categories
version-control, monitoring, documents

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/
- tldr page: https://tldr.inbrowser.app/pages/common/jj
