# msfconsole

Consola para el Metasploit Framework.

## Usage
- `msfconsole` : inicia la consola
- `msfconsole --quiet` : lanza la consola quietamente sin ningún mensaje
- `msfconsole --no-database` : no habilita el soporte de bases de datos
- `msfconsole --execute-command "use auxiliary/server/capture/ftp; set SRVHOST 0.0.0.0; set SRVPORT 21; run"` : ejecuta los comandos de la consola (Nota: utiliza `;` para pasar varios comandos)
- `msfconsole --version` : muestra versión

## Links
- More information: https://docs.rapid7.com/metasploit/msf-overview/
