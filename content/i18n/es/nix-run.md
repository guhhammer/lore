# nix run

Ejecuta una aplicación desde un flake de Nix.

## Usage
- `nix run` : ejecuta la aplicación predeterminada del flake en el directorio actual
- `nix run nixpkgs#pkg` : ejecuta un comando cuyo nombre iguala el nombre de un paquete de nixpkgs (si quieres un comando diferente de ese paquete, ver también `tldr nix shell`)
- `nix run nixpkgs#vim -- ruta/al/archivo` : ejecuta un comando con los argumentos proporcionados
- `nix run nombre_de_remoto:dueño/repositorio` : ejecuta desde un repositorio remoto
- `nix run nombre_de_remoto:dueño/repositorio/referencia` : ejecuta desde un repositorio remoto usando una etiqueta específica, revisión o rama
- `nix run "nombre_de_remoto:dueño/repositorio?dir=nombre_del_directorio#aplicación"` : ejecuta desde un repositorio remoto especificando un subdirectorio y un programa
- `nix run github:dueño/repositorio/pull/número/head` : ejecuta el flake de una solicitud de incorporación de cambios de GitHub

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-run.html
