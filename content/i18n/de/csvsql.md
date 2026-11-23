# csvsql

Generiere SQL-Anweisungen für eine CSV-Datei oder führe diese Anweisungen direkt in einer Datenbank aus. Teil von csvkit.

## Usage
- `csvsql pfad/zu/datei.csv` : generiere eine `CREATE TABLE`-SQL-Anweisung für eine CSV-Datei
- `csvsql --insert --db "mysql://benutzer:passwort@host/datenbank" pfad/zu/datei.csv` : importiere eine CSV-Datei in eine SQL-Datenbank
- `csvsql --query "select * from 'datei'" datei.csv` : führe eine SQL-Abfrage auf einer CSV-Datei aus

## Links
- More information: https://csvkit.readthedocs.io/en/latest/scripts/csvsql.html
