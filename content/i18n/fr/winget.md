# winget

Gestionnaire de paquets Windows.

## Usage
- `winget add paquet` : installe un paquet
- `winget rm paquet` : supprime un paquet (remarque : `remove` peut aussi être utilisé à la place de `uninstall`)
- `winget show paquet` : affiche des informations sur un paquet
- `winget search paquet` : recherche un paquet
- `winget upgrade -r` : met à jour tous les paquets vers les dernières versions
- `winget ls -s winget` : liste tous les paquets installés qui peuvent être gérés avec `winget`
- `winget import|export --import-file|--output chemin/vers/fichier` : importe des paquets depuis un fichier, ou exporte les paquets installés vers un fichier
- `winget validate chemin/vers/manifeste` : valide des manifestes avant de soumettre une PR au dépôt winget-pkgs

## Links
- More information: https://learn.microsoft.com/windows/package-manager/winget
