# msedge

Navegador web moderno desarrollado por Microsoft basado en el navegador web Chromium desarrollado por Google. Este comando está disponible como `microsoft-edge` en otras plataformas. Nota: Argumentos adicionales del comando de `chromium` también pueden ser utilizables para controlar Microsoft Edge.

## Usage
- `msedge https://example.com|ruta\al\archivo.html` : abre una URL o archivo específico
- `msedge --inprivate example.com` : abre en modo InPrivate
- `msedge --new-window example.com` : abre en una nueva ventana
- `msedge --app https://example.com` : abre en modo aplicación (sin barras de herramientas, barra de URL, botones, etc.)
- `msedge --proxy-server "socks5://hostname:66" example.com` : usa un servidor proxy
- `msedge --user-data-dir ruta\al\directorio` : abre con un directorio de perfil personalizado
- `msedge --user-data-dir ruta\al\directorio --disable-web-security` : abre sin validación CORS (útil para probar una API)
- `msedge --auto-open-devtools-for-tabs` : abre con una ventana de DevTools para cada pestaña abierta

## Links
- More information: https://microsoft.com/edge
