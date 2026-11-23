# firefox

Un navegador web gratuito y de código abierto.

## Usage
- `firefox https://www.duckduckgo.com` : inicia Firefox y abre una página web
- `firefox --new-window https://www.duckduckgo.com` : abre una nueva ventana
- `firefox --private-window` : abre una ventana privada (incógnito)
- `firefox --search "wikipedia"` : busca "wikipedia" utilizando el motor de búsqueda predeterminado
- `firefox --safe-mode` : inicia Firefox en modo seguro, con todas las extensiones desactivadas
- `firefox --headless --screenshot ruta/al/archivo_de_salida.png https://example.com/` : hace una captura de pantalla de una página web en modo sin interfaz gráfica
- `firefox --profile ruta/al/directorio https://example.com/` : utiliza un perfil específico para permitir que se ejecuten varias instancias independientes de Firefox a la vez
- `firefox --setDefaultBrowser` : establece Firefox como navegador predeterminado

## Links
- More information: https://wiki.mozilla.org/Firefox/CommandLineOptions
