# chromium

Browser web open-source sviluppato e mantenuto principalmente da Google. Nota: potrebbe essere necessario sostituire il comando `chromium` con il browser desiderato, come `brave`, `google-chrome`, `opera` o `vivaldi`.

## Usage
- `chromium https://example.com|percorso/del/file.html` : apri un URL o un file specifico
- `chromium --incognito example.com` : apri in modalità incognito
- `chromium --new-window example.com` : apri in una nuova finestra
- `chromium --app=https://example.com` : apri in modalità applicazione (senza barre degli strumenti, barra URL, pulsanti, ecc.)
- `chromium --proxy-server="socks5://hostname:66" example.com` : usa un server proxy
- `chromium --user-data-dir=percorso/della/directory` : apri con una directory di profilo personalizzata
- `chromium --user-data-dir=percorso/della/directory --disable-web-security` : apri senza validazione CORS (utile per testare un'API)
- `chromium --auto-open-devtools-for-tabs` : apri con una finestra DevTools per ogni scheda aperta

## Links
- More information: https://www.chromium.org/developers/how-tos/run-chromium-with-flags/
