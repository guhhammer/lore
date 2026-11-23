# sshpass

Stelle SSH Passwörtern bereit.

## Usage
- `sshpass -d 0 ssh benutzer@server` : stelle eine Verbindung zu einem externen Server über ein Passwort aus einem Datei-Objekt her (in diesem Fall `stdin`)
- `sshpass -p passwort ssh -o StrictHostKeyChecking=no benutzer@server` : stelle eine Verbindung zu einem externen Server mit Hilfe eines Passworts bei automatischer Akzeptierung von unbekannten SSH Schlüsseln her
- `sshpass -f pfad/zu/datei ssh -o StrictHostKeyChecking=no benutzer@server "befehl"` : stelle eine Verbindung zu einem externen Server mit Hilfe eines Passworts aus der ersten Zeile einer Datei bei automatischer Akzeptierung von unbekannten SSH Schlüsseln mit anschließender Ausführung eines Befehls her

## Links
- More information: https://manned.org/sshpass
