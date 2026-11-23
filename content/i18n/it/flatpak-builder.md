# flatpak-builder

Aiuta a compilare le dipendenze delle applicazioni.

## Usage
- `flatpak-builder path/to/build_directory path/to/manifest` : compila un Flatpak e lo esporta in un nuovo repository
- `flatpak-builder --repo repository_name path/to/build_directory path/to/manifest` : compila un Flatpak e lo esporta nel repository specificato
- `flatpak-builder --install path/to/build_directory path/to/manifest` : compila un Flatpak e lo installa localmente
- `flatpak-builder --gpg-sign key_id --repo repository_name path/to/manifest` : compila e firma un Flatpak e lo esporta nel repository specificato
- `flatpak-builder --run path/to/build_directory path/to/manifest sh` : esegue una shell all'interno di un sandbox dell'applicazione senza installarla

## Links
- More information: https://docs.flatpak.org/en/latest/flatpak-builder-command-reference.html
