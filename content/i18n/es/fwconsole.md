# fwconsole

Gestiona y configura un sistema FreePBX (servidor PBX).

## Usage
- `fwconsole reload` : recarga las configuraciones de FreePBX
- `fwconsole start` : inicia Asterisk y otros comandos necesarios para FreePBX
- `fwconsole stop` : detiene Asterisk y otros comandos necesarios para FreePBX
- `fwconsole setting palabra_clave nuevo_valor` : visualiza y actualiza la configuración
- `fwconsole backup --list` : lista las copias de seguridad disponibles
- `fwconsole list` : lista de comandos FreePBX disponibles
- `fwconsole chown` : cambia la propiedad de todos los archivos y directorios que FreePBX necesita que sean propiedad del usuario apache
