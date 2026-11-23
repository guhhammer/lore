# gcrane completion

Genera el script de autocompletado para gcrane para el intérprete de comandos especificado. Los intérpretes de comandos disponibles son `bash`, `fish`, `powershell` y `zsh`.

## Usage
- `gcrane completion nombre_del_intérprete_de_comandos` : genera el script de autocompletado para tu intérprete de comandos
- `gcrane completion nombre_del_intérprete_de_comandos --no-descriptions` : deshabilita descripciones de autocompletado
- `gcrane completion powershell | Out-String | Invoke-Expression` : carga autocompletados en tu sesión actual de intérpretes de comandos (powershell)
- `gcrane completion powershell | Out-String | Invoke-Expression` : carga autocompletados para cada nueva sesión (powershell)
- `gcrane completion nombre_del_intérprete_de_comandos -h` : muestra la ayuda
