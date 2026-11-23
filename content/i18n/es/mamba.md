# mamba

Gestor de paquetes rápido y multiplataforma, diseñado como sustituto directo de conda. Algunos subcomandos, como `repoquery`, tienen su propia documentación de uso.

## Usage
- `mamba create -n nombre_entorno python=3.10 matplotlib` : crea un nuevo entorno e instala los paquetes especificados en el mismo
- `mamba install -c conda-forge python=3.6 numpy` : instala paquetes en el entorno actual, especificando el canal del paquete
- `mamba update -a` : actualiza todos los paquetes del entorno actual
- `mamba repoquery search numpy` : busca un paquete específico en todos los repositorios
- `mamba info -e` : muestra todos los entornos
- `mamba clean -pt` : elimina los paquetes y archivos tarballs no utilizados de la caché
- `mamba activate nombre_del_entorno` : activa un entorno
- `mamba list` : enumera todos los paquetes instalados en el entorno actualmente activado

## Links
- More information: https://mamba.readthedocs.io/en/latest/user_guide/mamba.html
