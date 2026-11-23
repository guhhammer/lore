# echo

Imprime los argumentos dados.

## Usage
- `echo "Hola Mundo"` : imprime un mensaje de texto. Nota: las comillas son opcionales
- `echo "Mi ruta es $PATH"` : imprime un mensaje con variables de ambiente
- `echo -n "Hola Mundo"` : imprime un mensaje sin la nueva línea
- `echo "Hola Mundo" >>archivo.txt` : añade un mensaje al final del archivo
- `echo -e "Column 1\tColumn 2"` : habilita la interpretación de escapes de backslash (caracteres especiales)
- `echo $?` : imprime el estado de salida del último comando ejecutado (Nota: En Windows Command Prompt y PowerShell los equivalentes son `echo %errorlevel%` y `$lastexitcode` respectivamente)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/echo-invocation.html
