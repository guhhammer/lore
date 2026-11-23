# mssqlclient.py

Maak verbinding met Microsoft SQL Server instanties en voer queries uit. Onderdeel van de Impacket-suite.

## Usage
- `mssqlclient.py -windows-auth domein/gebruikersnaam:wachtwoord@doel` : maak verbinding met een MSSQL-server met Windows-authenticatie
- `mssqlclient.py gebruikersnaam:wachtwoord@doel` : maak verbinding met SQL-server-authenticatie
- `mssqlclient.py domein/gebruikersnaam@doel -hashes LM_Hash:NT_Hash` : maak verbinding met pass-the-hash-authenticatie
- `mssqlclient.py -k domein/gebruikersnaam@doel` : maak verbinding met Kerberos-authenticatie (geldige tickets vereist)
- `mssqlclient.py gebruikersnaam:wachtwoord@doel -query "SELECT user_name();"` : voer een specifieke SQL-opdracht uit na verbinding
- `mssqlclient.py gebruikersnaam:wachtwoord@doel -file pad/naar/sql_bestand.sql` : voer meerdere SQL-opdrachten vanuit een bestand uit
- `mssqlclient.py gebruikersnaam:wachtwoord@doel -db database_naam` : maak verbinding met een specifieke database-instantie (standaard is `None`)
- `mssqlclient.py gebruikersnaam:wachtwoord@doel -show` : geef SQL-query's weer voor uitvoering

## Links
- More information: https://github.com/fortra/impacket
