# nix-collect-garbage

Elimina rutas del almacén de nix desusados o inalcanzables. Las generaciones se pueden listar usando `nix-env --list-generations`.

## Usage
- `nix-collect-garbage -d` : elimina todas las rutas del almacén desusados por las generaciones actuales de cada perfil
- `nix-collect-garbage -d --dry-run` : simula la eliminación de rutas del almacén antiguas
- `nix-collect-garbage --delete-older-than 30d` : elimina todas las rutas del almacén más antiguas que 30 días

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/nix-collect-garbage.html
