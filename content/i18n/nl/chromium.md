# chromium

Open-source webbrowser voornamelijk ontwikkeld en onderhouden door Google. Opmerking: Wellicht dient het `chromium` commando vervangen te worden door de gewenste webbrowser, zoals `brave`, `google-chrome`, `opera`, of `vivaldi`.

## Usage
- `chromium https://example.com|pad/naar/bestand.html` : open een specifieke URL of bestand
- `chromium --incognito example.com` : open in de incognito-modus
- `chromium --new-window example.com` : open in een nieuw venster
- `chromium --app=https://example.com` : open in de applicatiemodus (zonder toolbars, URL-balk, knoppen, etc.)
- `chromium --proxy-server="socks5://hostname:66" example.com` : gebruik een proxyserver
- `chromium --user-data-dir=pad/naar/map` : open met een aangepaste profielmap
- `chromium --user-data-dir=pad/naar/map --disable-web-security` : open zonder CORS validatie (nuttig om een API te testen)
- `chromium --auto-open-devtools-for-tabs` : open met een DevTools venster voor elk geopend tabblad

## Links
- More information: https://www.chromium.org/developers/how-tos/run-chromium-with-flags/
