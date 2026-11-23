# dropdb

Verwijder een PostgreSQL-database. Een eenvoudige wrapper voor het SQL-commando `DROP DATABASE`.

## Usage
- `dropdb database_naam` : verwijder een database
- `dropdb -i database_naam` : vraag om bevestiging voordat destructieve acties worden uitgevoerd
- `dropdb -U gebruikersnaam database_naam` : maak verbinding als een specifieke gebruiker en verwijder een database
- `dropdb -W database_naam` : forceer een wachtwoordprompt voordat er wordt verbonden met de database
- `dropdb -w database_naam` : onderdruk een wachtwoordprompt voordat er wordt verbonden met de database
- `dropdb -h host database_naam` : specificeer de hostnaam van de server
- `dropdb -p poort database_naam` : specificeer de serverpoort
- `dropdb -f database_naam` : probeer actieve verbindingen te verbreken voordat de database wordt vernietigd

## Links
- More information: https://www.postgresql.org/docs/current/app-dropdb.html
