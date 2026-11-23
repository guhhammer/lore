# pio team

Beheer PlatformIO teams.

## Usage
- `pio team create --description beschrijving organisatie_naam:team_naam` : maak een nieuw team met de gespecificeerde beschrijving
- `pio team destroy organisatie_naam:team_naam` : verwijder een team
- `pio team add organisatie_naam:team_naam gebruikersnaam` : voeg een nieuwe gebruiker toe aan een team
- `pio team remove organisatie_naam:team_naam gebruikersnaam` : verwijder een gebruiker uit een team
- `pio team list` : toon alle teams waar de gebruiker lid van is en alle leden
- `pio team list organisatie_naam` : toon alle teams in een organisatie
- `pio team update --name nieuwe_team_naam organisatie_naam:team_naam` : hernoem een team
- `pio team update --description nieuwe_beschrijving organisatie_naam:team_naam` : verander de beschrijving van een team

## Links
- More information: https://docs.platformio.org/en/latest/core/userguide/team/
