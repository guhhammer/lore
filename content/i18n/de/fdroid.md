# fdroid

F-Droid Build Tool. F-Droid ist ein installierbarer Katalog mit FOSS (Freie Open Source Software) Apps für Android.

## Usage
- `fdroid build app_id` : kompiliere eine bestimmte App
- `fdroid build app_id --server` : kompiliere eine bestimmte App in einer Build-Server-VM
- `fdroid publish app_id` : veröffentliche die App im lokalen Repository
- `fdroid install app_id` : installiere die App auf jedem verbundenen Gerät
- `fdroid lint --format app_id` : überprüfe, ob die Metadaten korrekt formatiert sind
- `fdroid rewritemeta app_id` : korrigiere die Formatierung automatisch (wenn möglich)

## Links
- More information: https://f-droid.org/en/docs/Building_Applications/
