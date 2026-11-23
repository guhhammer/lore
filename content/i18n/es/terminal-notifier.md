# terminal-notifier

Envía notificaciones de usuario en macOS.

## Usage
- `terminal-notifier -group tldr-info -title TLDR -message 'TLDR mola'` : envía una notificación (sólo se necesita el mensaje)
- `echo 'Datos de mensajes transmitidos!' | terminal-notifier -sound default` : muestra datos transmitidos con un sonido
- `terminal-notifier -message 'Comprueba tus acciones de Apple!' -open 'http://finance.yahoo.com/q?s=AAPL'` : abre una URL al hacer clic en la notificación
- `terminal-notifier -message 'Importados 42 contactos.' -activate com.apple.AddressBook` : abre una aplicación al hacer clic en la notificación

## Links
- More information: https://github.com/julienXX/terminal-notifier
