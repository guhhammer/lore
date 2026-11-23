# adduser

Tool um Benutzer hinzuzufügen.

## Usage
- `adduser benutzername` : erstelle einen neuen Benutzer mit einem Standard-Home-Verzeichnis und Aufforderung an den Benutzer, ein Passwort festzulegen
- `adduser --no-create-home benutzername` : erstelle einen neuen Benutzer ohne Home-Verzeichnis
- `adduser --home pfad/zu/home benutzername` : erstelle einen neuen Benutzer mit einem Home-Verzeichnis unter dem angegebenen Pfad
- `adduser --shell pfad/zu/shell benutzername` : erstelle einen neuen Benutzer, bei dem die angegebene Shell als Anmeldeshell eingestellt ist
- `adduser --ingroup gruppe benutzername` : erstelle einen neuen Benutzer und füge ihn zur angegebenen Gruppe hinzu

## Links
- More information: https://manned.org/adduser
