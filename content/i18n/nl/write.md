# write

Schrijf een bericht naar de terminal van een opgegeven ingelogde gebruiker (`<Ctrl c>` om te stoppen met het schrijven van berichten). Gebruik het `who`-commando om alle terminal-ID's van alle actieve gebruikers op het systeem te vinden.

## Usage
- `write gebruikersnaam terminal_id` : verstuur een bericht naar een gegeven gebruiker op een gegeven terminal-ID
- `write testuser tty5` : verstuur een bericht naar "testuser" op terminal `/dev/tty5`
- `write johndoe pts/5` : verstuur een bericht naar "johndoe" op pseudo-terminal `/dev/pts/5`

## Links
- More information: https://manned.org/write.1p
