# asterisk

Ejecuta y administra instancias de servidores telefónicos e intercambiadores (teléfonos).

## Usage
- `asterisk -r -vvv` : reconecta a un servidor en ejecución y activa el registro con 3 niveles de verbosidad
- `asterisk -r -x "comando"` : reconecta a un servidor en ejecución, ejecuta un solo comando y regresa
- `asterisk -r -x "sip show peers"` : muestra los clientes chan_SIP (teléfonos)
- `asterisk -r -x "core show channels"` : muestra las llamadas y canales activos
- `asterisk -r -x "voicemail show users"` : muestra los buzones de voz
- `asterisk -r -x "hangup request id_del_canal"` : termina un canal
- `asterisk -r -x "sip reload"` : recarga la configuración de chan_SIP

## Links
- More information: https://docs.asterisk.org/Operation/
