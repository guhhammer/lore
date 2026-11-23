# flatpak run

Esegue applicazioni flatpak e runtime.

## Usage
- `flatpak run com.example.app` : esegue un'applicazione installata
- `flatpak run --branch=stable|beta|master|... com.example.app` : esegue un'applicazione installata da un ramo specifico, ad es. stable, beta, master
- `flatpak run --command=sh com.example.app` : esegue una shell interattiva all'interno di un flatpak
- `flatpak run --runtime-version=24.08|master|stable|... com.example.app` : esegue un'applicazione installata con una versione di runtime specifica
- `flatpak run --runtime=org.freedesktop.Sdk com.example.app` : esegue un'applicazione installata con un runtime diverso (ma stesso numero di versione)
