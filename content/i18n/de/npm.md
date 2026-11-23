# npm

Ein Kommandozeilenwerkzeug für die Verwaltung von JavaScript und Node.js Paketen (Packages).

## Usage
- `npm init -y` : erstelle eine `package.json` Datei interaktiv
- `npm i` : installiere alle in der `package.json` Datei gelisteten Abhängigkeiten
- `npm i package_name@version` : installiere eine spezifische Version eines Packages und füge es automatisch der `package.json` Datei hinzu
- `npm i package_name -D` : installiere ein Package und füge es als Entwicklungs-Abhängigkeit der `package.json` Datei hinzu
- `npm i package_name -g` : installiere ein Package global
- `npm r package_name` : deinstalliere ein Package und entferne es automatisch aus der `package.json` Datei
- `npm ls` : gib eine Liste aller lokal installierten Packages aus
- `npm ls -g --depth 0` : gib eine Liste aller global installierten Packages aus

## Links
- More information: https://docs.npmjs.com/cli/npm/
