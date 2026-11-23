# pnpm

Schneller, speicherplatzsparender Paketmanager für Node.js. Ein Kommandozeilenwerkzeug für die Verwaltung von JavaScript und Node.js Paketen (Packages).

## Usage
- `pnpm init` : erstelle eine `package.json` Datei interaktiv
- `pnpm i` : installiere alle in der `package.json` Datei gelisteten Abhängigkeiten
- `pnpm add modul_name@version` : installiere eine spezifische Version eines Packages und füge es automatisch der `package.json` Datei hinzu
- `pnpm add modul_name -D` : installiere ein Package und füge es als Entwicklungs-Abhängigkeit der `package.json` Datei hinzu
- `pnpm add modul_name -g` : installiere ein Package global
- `pnpm rm modul_name` : deinstalliere ein Package und entferne es automatisch aus der `package.json` Datei
- `pnpm ls` : gib eine Liste aller lokal installierten Packages aus
- `pnpm ls -g --depth 0` : gib eine Liste aller global installierten Packages aus

## Links
- More information: https://pnpm.io/pnpm-cli
