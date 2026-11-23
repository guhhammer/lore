# rsh

Voer commando's uit op een externe host.

## Usage
- `rsh remote_host ls -l` : voer een commando uit op een externe host
- `rsh remote_host -l gebruikersnaam ls -l` : voer een commando uit op een externe host met een specifieke gebruikersnaam
- `rsh remote_host --no-err ls -l` : redirect `stdin` naar `/dev/null` bij het uitvoeren van een commando op een externe host

## Links
- More information: https://www.gnu.org/software/inetutils/manual/inetutils.html#rsh-invocation
