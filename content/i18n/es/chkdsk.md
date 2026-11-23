# chkdsk

Verifica el sistema de archivos y los metadatos del volumen en busca de errores.

## Usage
- `chkdsk volumen` : especifica la letra de la unidad (seguida de dos puntos), el punto de montaje o el nombre del volumen a verificar
- `chkdsk volumen /f` : corrige errores en un volumen específico
- `chkdsk volumen /x` : desmonta un volumen específico antes de verificar
- `chkdsk /ltamaño` : cambia el tamaño del archivo de registro al tamaño especificado (solo para NTFS)
