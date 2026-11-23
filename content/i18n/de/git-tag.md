# git tag

Erstelle, lösche, überprüfe und liste Tags auf.

## Usage
- `git tag` : liste alle Tags auf
- `git tag tag_name` : erstelle einen Tag mit Namen, welcher auf den aktuellen Commit zeigt
- `git tag tag_name commit` : erstelle einen Tag mit Namen, welcher auf einen bestimmten Commit zeigt
- `git tag tag_name -m anmkerung` : erstelle einen Tag mit Anmerkung
- `git tag -d tag_name` : lösche einen Tag mit bestimmten Namen
- `git fetch -t` : lade die aktualisierten Tags aus dem Upstream
- `git tag --contains commit` : liste alle Tags auf, bei denen sich in den vorangegangenen Commits ein bestimmter Commit findet

## Links
- More information: https://git-scm.com/docs/git-tag
