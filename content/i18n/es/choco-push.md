# choco push

Sube un paquete NuGet compilado (`.nupkg`) a un feed de paquetes.

## Usage
- `choco push -s https://push.chocolatey.org/` : sube un `.nupkg` compilado al feed especificado
- `choco push -s https://push.chocolatey.org/ --timeout 500` : sube un `.nupkg` compilado al feed especificado con un tiempo de espera en segundos (el valor predeterminado es 2700)

## Links
- More information: https://docs.chocolatey.org/en-us/create/commands/push/
