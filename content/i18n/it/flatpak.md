# flatpak

Compila, installa ed esegue applicazioni flatpak e runtime.

## Usage
- `flatpak run com.example.app` : esegue un'applicazione installata
- `flatpak install remote_name com.example.app` : installa un'applicazione da una fonte remota
- `flatpak list --app` : elenca le applicazioni installate, ignorando i runtime
- `flatpak update` : aggiorna tutte le applicazioni e i runtime installati
- `flatpak remote-add --if-not-exists remote_name remote_url` : aggiunge una fonte remota
- `flatpak remove com.example.app` : rimuove un'applicazione installata
- `flatpak remove --unused` : rimuove tutte le applicazioni inutilizzate
- `flatpak info com.example.app` : mostra informazioni su un'applicazione installata

## Links
- More information: https://docs.flatpak.org/en/latest/flatpak-command-reference.html#flatpak
