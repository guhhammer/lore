# git lfs

Lavora con file di grandi dimensioni in repository Git.

## Usage
- `git lfs install` : inizializza Git LFS
- `git lfs track '*.bin'` : tieni traccia dei file che soddisfano un criterio `glob`
- `git config -f .lfsconfig lfs.url lfs_url_endpoint` : cambia l'URL endpoint di Git LFS (utile quando server LFS e server Git sono separati)
- `git lfs track` : elenca i criteri tracciati
- `git lfs ls-files` : elenca i file tracciati che sono già stati salvati in un commit
- `git lfs push --all nome_repository_remoto nome_ramo` : invia tutti gli oggetti Git LFS al server remoto (utile in caso di errori)
- `git lfs fetch` : scarica tutti gli oggetti Git LFS
- `git lfs checkout` : ripristina gli oggetti Git LFS

## Links
- More information: https://github.com/git-lfs/git-lfs/tree/main/docs
