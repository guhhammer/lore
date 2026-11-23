# komac

Crea manifiestos WinGet para el repositorio `winget-pkgs`.

## Usage
- `komac new Paquete.Identificador -v 1.2.3 -u https://example.com/app.exe` : crea un nuevo paquete desde cero
- `komac update Paquete.Identificador -v 1.2.3 -u https://example.com/app.exe` : actualiza un paquete existente con una nueva versión
- `komac update Paquete.Identificador -v 1.2.3 -u https://example.com/app.exe https://example.com/app.msi ... -s` : actualiza un paquete con varias URL y lo envía automáticamente
- `komac remove Paquete.Identificador -v 1.2.3` : elimina una versión de winget-pkgs
- `komac list Paquete.Identificador` : lista todas las versiones de un paquete
- `komac sync` : sincroniza tu fork de winget-pkgs con el repositorio fuente
- `komac token add -t tu_token_github` : actualiza el token de GitHub almacenado
- `komac complete bash|elvish|fish|powershell|zsh` : genera una secuencia de comandos de autocompletado del intérprete de comandos

## Links
- More information: https://github.com/russellbanks/Komac
