# dockdiver

Una herramienta para interactuar con registros Docker, incluyendo listar y volcar repositorios.

## Usage
- `dockdiver -url https://example.com -list` : lista todos los repositorios en un registro Docker
- `dockdiver -url https://example.com -dump nombre_repositorio` : vuelca un repositorio específico al directorio de salida por defecto (docker_dump)
- `dockdiver -url http://example.com -dump-all -username usuario -password contraseña` : vuelca todos los repositorios con autenticación básica
- `dockdiver -url http://example.com -dump nombre_repositorio -port puerto -rate solicitudes_por_segundo -dir ruta/al/directorio_salida` : vuelca un repositorio con un puerto personalizado y un límite de velocidad
- `dockdiver -url http://example.com -dump-all -bearer bearer_token` : vuelca todos los repositorios con token de portador para autorización
- `dockdiver -url http://example.com -list -headers '{"X-Custom": "Value"}'` : añade cabeceras personalizadas como JSON (por ejemplo, '{"X-Custom": "Value"}')

## Links
- More information: https://github.com/MachiavelliII/dockdiver#instructions
