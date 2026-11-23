# chmod

Ändere die Zugriffsberechtigungen einer Datei oder eines Verzeichnisses.

## Usage
- `chmod u+x pfad/zu/datei` : gib dem Besitzer einer Datei (user) das Recht, sie auszuführen (execute)
- `chmod u+rw pfad/zu/datei_oder_verzeichnis` : gib dem Besitzer (user) Rechte zum Lesen (read) und Schreiben (write) einer Datei / einem Verzeichnis
- `chmod g-x pfad/zu/datei` : entferne die Ausführrechte (execute) der Besitzergruppe
- `chmod a+rx pfad/zu/datei` : gib allen Benutzern Rechte zum Lesen (read) und Ausführen (execute) einer Datei
- `chmod o=g pfad/zu/datei` : gib anderen (others) (nicht in der Besitzergruppe) die gleichen Rechte wie der Besitzergruppe
- `chmod o= pfad/zu/datei` : entferne alle Rechte der anderen (others)
- `chmod -R g+w,o+w pfad/zu/verzeichnis` : ändere Rechte rekursiv, indem der Besitzergruppe und anderen (others) die Rechte zum Schreiben (write) geben werden
- `chmod -R a+rX pfad/zu/verzeichnis` : gib allen Benutzern rekursiv Rechte zum Lesen (read) von Dateien und Ausführen (eXecute) von Unterverzeichnissen innerhalb eines Verzeichnisses
