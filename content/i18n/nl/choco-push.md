# choco push

Push een gecompileerd NuGet pakket (`.nupkg`) naar een pakketfeed.

## Usage
- `choco push -s https://push.chocolatey.org/` : push een gecompileerd `.nupkg` naar de gespecificeerde feed
- `choco push -s https://push.chocolatey.org/ --timeout 500` : push een gecompileerd `.nupkg` naar de gespecificeerde feed met een timeout in seconden (standaard is 2700)

## Links
- More information: https://docs.chocolatey.org/en-us/create/commands/push/
