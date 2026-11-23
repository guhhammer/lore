# paste

Combina líneas de archivos.

## Usage
- `paste -s ruta/al/archivo` : une todas las líneas en una sola línea, utilizando `TAB` como delimitador
- `paste -s -d delimitador ruta/al/archivo` : une todas las líneas en una sola línea, utilizando el delimitador especificado
- `paste ruta/al/archivo1 ruta/al/archivo2` : combina dos archivos uno al lado del otro, cada uno en su columna, utilizando `TAB` como delimitador
- `paste -d delimitador ruta/al/archivo1 ruta/al/archivo2` : combina dos archivos uno al lado del otro, cada uno en su columna, utilizando el delimitador especificado
- `paste -d '\n' ruta/al/archivo1 ruta/al/archivo2` : combina dos archivos, con líneas añadidas alternativamente
