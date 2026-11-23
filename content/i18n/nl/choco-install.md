# choco install

Installeer een of meerdere pakketten met Chocolatey.

## Usage
- `choco install pakket1 pakket2 ...` : installeer een of meerdere pakketten
- `choco install pad\naar\pakketten_bestand.config` : installeer pakketten van een aangepast configuratiebestand
- `choco install pad\naar\bestand` : installeer een specifiek `.nuspec` of `.nupkg` bestand
- `choco install pakket --version versie` : installeer een specifieke versie van een pakket
- `choco install pakket -f` : installeer geforceerd een pakket opnieuw
- `choco install pakket -y` : bevestig alle prompts automatisch
- `choco install pakket -s source_url|alias` : specificeer een aangepaste bron om pakketten van te ontvangen
- `choco install pakket -u gebruikersnaam -p wachtwoord` : geef een gebruikersnaam en wachtwoord voor authenticatie op

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/install/
