# semodule

Beheer SELinux-beleidsmodules.

## Usage
- `sudo semodule -l` : toon alle geïnstalleerde beleidsmodules
- `sudo semodule -i pad/naar/module.pp` : installeer een nieuwe beleidsmodule
- `sudo semodule -r module_naam` : verwijder een beleidsmodule
- `sudo semodule -e module_naam` : schakel een beleidsmodule in
- `sudo semodule -d module_naam` : schakel een beleidsmodule uit
- `sudo semodule -R` : herlaad alle beleidsmodules
- `sudo semodule -l -v` : toon de versie van geïnstalleerde beleidsmodules

## Links
- More information: https://manned.org/semodule
