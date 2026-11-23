# git lfs

Werk met grote bestanden in Git repositories.

## Usage
- `git lfs install` : initialiseer Git LFS
- `git lfs track '*.bin'` : houd bestanden bij die overeenkomen met een `glob`
- `git config -f .lfsconfig lfs.url lfs_eindpunt_url` : verander de Git LFS-eindpunt URL (handig als de LFS-server gescheiden is van de Git-server)
- `git lfs track` : toon patronen die worden bijgehouden
- `git lfs ls-files` : toon bestanden die worden bijgehouden en gecommit zijn
- `git lfs push --all externe_naam branch_naam` : push alle Git LFS-objecten naar de externe server (handig als er fouten optreden)
- `git lfs fetch` : haal alle Git LFS-objecten op
- `git lfs checkout` : vervang pointerbestanden met werkelijke Git LFS-objecten

## Links
- More information: https://github.com/git-lfs/git-lfs/tree/main/docs
