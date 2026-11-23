# gobuster

Findet versteckte Pfade auf Webservern und mehr.

## Usage
- `gobuster dir -u https://example.com/ -w pfad/zu/datei` : finde Verzeichnisse und Dateien, die den Wörtern der Wortliste entsprechen
- `gobuster dns -do example.com -w pfad/zu/datei` : finde Subdomains
- `gobuster s3 -w pfad/zu/datei` : finde Amazon S3-Buckets
- `gobuster vhost -u https://example.com/ -w pfad/zu/datei` : finde andere virtuelle Hosts eines Servers
- `gobuster fuzz -u https://example.com/?parameter=FUZZ -w pfad/zu/datei` : fuzze den Wert eines URL-Parameters
- `gobuster fuzz -u https://example.com/?FUZZ=wert -w pfad/zu/datei` : fuzze den Namen eines URL-Parameters

## Links
- More information: https://github.com/OJ/gobuster#modes
