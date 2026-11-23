# dbx

Interactúa con la plataforma Databricks. Nota: esta herramienta ha sido retirada y se recomienda utilizar Databricks Asset Bundles en su lugar.

## Usage
- `dbx configure --profile DEFAULT` : crea un nuevo proyecto `dbx` en el directorio de trabajo actual
- `dbx sync dbfs --source ruta/al/directorio --dest ruta/al/directorio_remoto` : sincroniza archivos locales de la ruta especificada a DBFS y vigila los cambios
- `dbx deploy nombre_del_flujo_de_trabajo` : despliega el flujo de trabajo especificado en el almacenamiento de artefactos
- `dbx launch nombre_del_flujo_de_trabajo` : inicia el flujo de trabajo especificado después de desplegarlo

## Links
- More information: https://dbx.readthedocs.io/en/latest/reference/cli/#dbx
