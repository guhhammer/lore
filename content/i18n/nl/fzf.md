# fzf

Fuzzy finder.

## Usage
- `find pad/naar/map -type f | fzf` : start `fzf` op alle bestanden in de opgegeven map
- `ps aux | fzf` : start `fzf` voor actieve processen
- `find pad/naar/map -type f | fzf -m >pad/naar/bestand` : selecteer meerdere bestanden met `<Shift Tab>` en schrijf naar een bestand
- `fzf -q "query"` : start `fzf` met een bepaalde query
- `fzf -q "^core go$ | rb$ | py$"` : start `fzf` op items die beginnen met `core` en eindigen met `go`, `rb` of `py`
- `fzf -q '!pyc travis'` : start `fzf` op items die niet overeenkomen met `pyc`, maar wel `travis` bevatten

## Links
- More information: https://github.com/junegunn/fzf#usage
