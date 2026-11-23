# bd

A lightweight memory system and git-backed issue tracker for AI coding agents.

## Usage
- `bd init` : initialize a project database
- `bd create issue_title -d description -p 1 -t bug|feature|task|epic|chore` : create a new issue with description, priority, and type
- `bd list` : list all issues
- `bd ready` : show issues ready to work on (no blockers)
- `bd show issue_id` : display details of a specific issue
- `bd update issue_id -s open|in_progress|blocked|closed` : update an issue status
- `bd sync` : manually sync changes and import latest from Git
- `bd -h` : display help

## Categories
monitoring, version-control

## Links
- More information: https://github.com/steveyegge/beads#usage
- tldr page: https://tldr.inbrowser.app/pages/common/bd
