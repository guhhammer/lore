# trdsql

Ejecuta SQL en archivos CSV, LTSV, JSON, YAML y TBLN.

## Usage
- `trdsql -ocsv -oh "SELECT * FROM ruta/al/archivo/*.json" | sed 's/\([^,]*\)/"&"/g' >ruta/al/archivo.csv` : convierte datos de objetos de varios archivos JSON a un archivo CSV con encabezado (`-oh`) y comillas dobles
- `trdsql "SELECT * FROM ruta/al/archivo.json::.list"` : interpreta una lista JSON como tabla y pone objetos dentro como columnas ( ruta/al/archivo.json: `{"lista":[{"edad":"26", "nombre":"Tanaka"}]}`)
- `trdsql -icsv -ih "SELECT columna1,columna2 FROM ruta/al/archivo*.csv WHERE column2 != '' ORDER BY columna1 GROUP BY columna1"` : manipula una consulta SQL compleja con datos de varios archivos CSV cuya primera línea es la cabecera (`-ih`)
- `trdsql "SELECT columna1,columna2 FROM ruta/al/archivo1.csv UNION SELECT columna1,columna2 FROM ruta/al/archivo2.csv"` : combina el contenido de 2 archivos CSV en un archivo CSV
- `trdsql -driver postgres -dsn "host=nombre_del_host port=5433 dbname=nombre_de_la_base_de_datos" "SELECT 1"` : se conecta a la base de datos PostgreSQL
- `trdsql -driver mysql -dsn "usuario:contraseña@nombre_del_host/base_de_datos" -ih "CREATE TABLE tabla (columna1 int, columna2 varchar(20)) AS SELECT columna3 AS columna1,columna2 FROM ruta/al/archivo_cabecera.csv"` : crea una tabla de datos en una base de datos MySQL a partir de un archivo CSV
- `trdsql -iltsv "SELECT * FROM ruta/a/acceso.log.gz"` : muestra datos de archivos de registro comprimidos

## Links
- More information: https://noborus.github.io/trdsql/
