# git am

Applica file di patch. Utile quando si ricevono commit via email.

## Usage
- `git am percorso/del/file.patch` : applica un file di patch
- `git am --abort` : interrompi l'applicazione di un file di patch
- `git am --reject percorso/del/file.patch` : applica quanto possibile di un file di patch, salvando le parti non applicabili in file `.rej`

## Links
- More information: https://git-scm.com/docs/git-am
