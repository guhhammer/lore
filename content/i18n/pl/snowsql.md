# snowsql

Narzędzie wiersza SnowSQL serwisu bazodanowego Snowflake.

## Usage
- `snowsql --accountname konto --username użytkownik --dbname baza_danych --schemaname nazwa_schematu` : połącz z konkretną instancją pod adresem <https://konto.snowflakecomputing.com> (hasło może być wprowadzone w wierszu polecenia lub pliku konfiguracyjnym)
- `snowsql --config ścieżka/do/pliku_konfiguracyjnego` : połącz się z instancją zdefiniowaną w pliku konfiguracyjnym (domyślnie w `~/.snowsql/config`)
- `snowsql --mfa-passcode kod_podwójnej_weryfikacji` : połącz się z domyślnie zdefiniowaną instancją, podając kod autentykacji drugiego poziomu
- `snowsql --query 'zapytanie'` : wykonaj pojedyncze zapytanie lub komendę SnowSQL na domyślnym połączeniu (użyteczne w skryptach powłoki)
- `snowsql --filename ścieżka/do/pliku.sql` : wykonaj zapytania lub komendy z konkretnego pliku

## Links
- More information: https://docs.snowflake.com/en/user-guide/snowsql
