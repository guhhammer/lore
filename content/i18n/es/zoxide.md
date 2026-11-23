# zoxide

Realiza un seguimiento de los directorios más utilizados. Utiliza un algoritmo de clasificación para navegar a la mejor coincidencia.

## Usage
- `zoxide query string` : va al directorio mejor clasificado que contiene una `cadena` (`string`) en su nombre
- `zoxide query string1 string2` : va al directorio mejor clasificado que contiene `cadena1` (`string1`) y luego `cadena2` (`string2`)
- `zoxide query -i` : inicia una búsqueda interactiva de directorios (requiere `fzf`)
- `zoxide add ruta/al/directorio` : añade un directorio o incrementa su rango
- `zoxide remove ruta/al/directorio` : elimina un directorio de la base de datos de `zoxide`
- `zoxide init bash|elvish|fish|nushell|posix|powershell|tcsh|xonsh|zsh` : genera la configuración de la terminal para los alias de comandos (`z`, `zi`)

## Links
- More information: https://manned.org/zoxide
