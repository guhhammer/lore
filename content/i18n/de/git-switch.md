# git switch

Wechsle zwischen Branches. Verfügbar ab Git Version 2.23+.

## Usage
- `git switch branche_name` : wechsle zu einem existierenden Branch
- `git switch -c branch_name` : erstelle einen neuen Branch und wechsele zu diesem
- `git switch -c branch_name commit` : erstelle einen neuen Branch basierend auf einem existierenden Commit und wechsele zu diesem
- `git switch -` : wechsele zum vorherigen Branch
- `git switch --recurse-submodules branch_name` : wechsele zu einem Branch und aktualisiere alle Submodule entsprechend
- `git switch -m branch_name` : wechsele zu einem Branch und merge automatisch den aktuellen Branch und alle Änderungen, die nicht committed wurden

## Links
- More information: https://git-scm.com/docs/git-switch
