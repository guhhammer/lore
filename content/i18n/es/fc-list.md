# fc-list

Lista las fuentes disponibles instaladas en el sistema.

## Usage
- `fc-list` : devuelve una lista de las fuentes instaladas en su sistema
- `fc-list | grep 'DejaVu Serif'` : devuelve una lista de las fuentes instaladas con el nombre dado
- `fc-list | wc -l` : devuelve el número de fuentes instaladas con el nombre dado
- `fc-list :lang=jp` : devuelve una lista de las fuentes instaladas que soportan el idioma basado en su código de idioma
- `fc-list :charset=f303` : devuelve una lista de las fuentes instaladas que contienen el glifo especificado por su código Unicode

## Links
- More information: https://manned.org/fc-list
