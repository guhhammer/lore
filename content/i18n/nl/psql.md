# psql

PostgreSQL-client.

## Usage
- `psql database` : maak verbinding met de database. Standaard wordt er verbinding gemaakt met de lokale socket op poort 5432 met de huidig ingelogde gebruiker
- `psql -h host -p poort -U gebruikersnaam database` : maak verbinding met de database op de opgegeven serverhost die draait op de opgegeven poort met de opgegeven gebruikersnaam, zonder een wachtwoordprompt
- `psql -h host -p poort -U gebruikersnaam -W database` : maak verbinding met de database, waarbij aan de gebruiker om een wachtwoord wordt gevraagd
- `psql -c 'query' database` : voer één SQL-query of PostgreSQL-commando uit op de opgegeven database (handig in shell-scripts)
- `psql database -f pad/naar/bestand.sql` : voer commando's vanuit een bestand uit op de opgegeven database

## Links
- More information: https://www.postgresql.org/docs/current/app-psql.html
