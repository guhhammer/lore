# chromium

Navegador web de código abierto desarrollado y mantenido principalmente por Google. Nota: Es posible que necesite reemplazar el comando `chromium` con su navegador web deseado, como `brave`, `google-chrome`, `opera`, o `vivaldi`.

## Usage
- `chromium https://example.com|ruta/al/archivo.html` : abre una URL o archivo específico
- `chromium --incognito example.com` : abre en modo incógnito
- `chromium --new-window example.com` : abre en una nueva ventana
- `chromium --app=https://example.com` : abre en modo de aplicación (sin barras de herramientas, barra de URL, botones, etc.)
- `chromium --proxy-server="socks5://hostname:66" example.com` : usa un servidor proxy
- `chromium --user-data-dir=ruta/al/directorio` : abre con un directorio de perfil personalizado
- `chromium --user-data-dir=ruta/al/directorio --disable-web-security` : abre sin validación CORS (útil para probar una API)
- `chromium --auto-open-devtools-for-tabs` : abre con una ventana DevTools para cada pestaña abierta

## Links
- More information: https://www.chromium.org/developers/how-tos/run-chromium-with-flags/
