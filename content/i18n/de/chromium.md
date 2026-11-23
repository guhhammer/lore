# chromium

Open-Source-Webbrowser von Google.

## Usage
- `chromium https://example.com|pfad/zu/datei.html` : öffne eine bestimmte Datei oder URL
- `chromium --incognito example.com` : öffne eine URL im Inkognito-Modus
- `chromium --new-window example.com` : öffne eine URL in einem neuen Fenster
- `chromium --app=https://example.com` : öffne eine URL im Anwendungsmodus (ohne Symbolleisten, Suchleiste, Schaltflächen usw.)
- `chromium --proxy-server="socks5://hostname:66" example.com` : öffne eine URL und verwende einen Proxy-Server
- `chromium --user-data-dir=pfad/zu/verzeichnis` : öffne Chromium mit einem eigenen Profil-Verzeichnis
- `chromium --user-data-dir=pfad/zu/verzeichnis --disable-web-security` : öffne Chromium ohne CORS-Verifizierung (nützlich, um eine API zu testen)
- `chromium --auto-open-devtools-for-tabs` : öffne Chromium mit einem `DevTools`-Fenster für jeden geöffneten Tab

## Links
- More information: https://www.chromium.org/developers/how-tos/run-chromium-with-flags/
