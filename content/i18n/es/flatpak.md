# flatpak

Construye, instala y ejecuta aplicaciones y tiempos de ejecución flatpak.

## Usage
- `flatpak run com.example.app` : ejecuta una aplicación instalada
- `flatpak install nombre_remoto com.example.app` : instala una aplicación desde una fuente remota
- `flatpak list --app` : lista las aplicaciones instaladas, ignorando los tiempos de ejecución
- `flatpak update` : actualiza todas las aplicaciones y tiempos de ejecución instalados
- `flatpak remote-add --if-not-exists nombre_remoto url_remota` : añade una fuente remota
- `flatpak remove com.example.app` : elimina una aplicación instalada
- `flatpak remove --unused` : elimina todas las aplicaciones no utilizadas
- `flatpak info com.example.app` : muestra información sobre una aplicación instalada

## Links
- More information: https://docs.flatpak.org/en/latest/flatpak-command-reference.html#flatpak
