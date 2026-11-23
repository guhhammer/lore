# flatpak update

Aggiorna applicazioni flatpak e runtime.

## Usage
- `flatpak update` : aggiorna tutte le applicazioni e i runtime installati (usa `-y` per confermare automaticamente tutti i prompt)
- `flatpak update com.example.app` : aggiorna solo un'app specifica
- `flatpak update --commit COMMIT com.example.app` : aggiorna/Ripristina a un commit specifico (vedi anche flatpak remote-info e flatpak mask)
