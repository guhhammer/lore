# chpasswd

Verander de wachtwoorden van meerdere gebruikers via `stdin`.

## Usage
- `printf "gebruikersnaam:nieuw_wachtwoord" | sudo chpasswd` : verander het wachtwoord van een specifieke gebruiker
- `printf "gebruikersnaam_1:nieuw_wachtwoord_1\ngebruikersnaam_2:nieuw_wachtwoord_2" | sudo chpasswd` : verander de wachtwoorden van meerdere gebruikers (de invoertekst mag geen spaties bevatten)
- `printf "gebruikersnaam:nieuw_versleuteld_wachtwoord" | sudo chpasswd -e` : verander het wachtwoord van een specifieke gebruiker en geef het op in versleutelde vorm
- `printf "gebruikersnaam:nieuw_wachtwoord" | sudo chpasswd -c NONE|DES|MD5|SHA256|SHA512` : verander het wachtwoord van een specifieke gebruiker en gebruik een specifieke versleuteling voor het opgeslagen wachtwoord

## Links
- More information: https://manned.org/chpasswd
