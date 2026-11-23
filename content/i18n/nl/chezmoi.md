# chezmoi

Beheer dotfiles op meerdere, diverse machines.

## Usage
- `chezmoi init` : stel `chezmoi` in en maak een Git-repository aan in `~/.local/share/chezmoi`
- `chezmoi init repository_url` : stel `chezmoi` in vanuit bestaande dotfiles in een Git-repository
- `chezmoi add pad/naar/dotfile1 pad/naar/dotfile2 ...` : houd één of meer dotfiles bij
- `chezmoi re-add pad/naar/dotfile1 pad/naar/dotfile2 ...` : update repository met lokale wijzigingen
- `chezmoi edit pad/naar/dotfile_of_symlink` : bewerk de source status van een bijgehouden dotfile
- `chezmoi diff` : zie openstaande wijzigingen
- `chezmoi apply` : pas de wijzigingen toe
- `chezmoi update` : haal wijzigingen op van een externe repository en pas deze toe

## Links
- More information: https://www.chezmoi.io/reference/
