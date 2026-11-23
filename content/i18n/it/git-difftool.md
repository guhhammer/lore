# git difftool

Mostra le modifiche ai file tracciati usando uno strumento Diff esterno. Accetta le stesse opzioni e argomenti di Git diff.

## Usage
- `git difftool --tool-help` : elenca gli strumenti Diff disponibili
- `git config --global diff.tool "meld"` : imposta meld come strumento Diff predefinito
- `git difftool --staged` : usa lo strumento Diff predefinito per mostrare le modifiche nell'area di stage
- `git difftool -t opendiff commit` : uso uno specifico strumento Diff (opendiff) per mostrare le modifiche a partire da un dato commit

## Links
- More information: https://git-scm.com/docs/git-difftool
