# repair-bde

Intenta reparar o descifra un volumen cifrado con BitLocker dañado.

## Usage
- `repair-bde C:` : intenta reparar un volumen especificado
- `repair-bde C: D:` : intenta reparar un volumen especificado y envía la salida a otro volumen
- `repair-bde C: -RecoveryKey ruta\al\archivo.bek` : intenta reparar un volumen especificado usando el archivo de clave de recuperación proporcionado
- `repair-bde C: -RecoveryPassword contraseña` : intenta reparar un volumen especificado usando la contraseña numérica de recuperación proporcionada
- `repair-bde C: -Password contraseña` : intenta reparar un volumen especificado usando la contraseña proporcionada
- `repair-bde C: -KeyPackage ruta\al\directorio` : intenta reparar un volumen especificado usando el paquete de claves proporcionado
- `repair-bde C: -LogFile ruta\al\archivo` : registra toda la salida en un archivo específico
- `repair-bde /?` : muestra la ayuda
