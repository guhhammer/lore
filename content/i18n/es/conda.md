# conda

Gestión de paquetes, dependencias y entornos para cualquier lenguaje de programación. Algunos subcomandos, como `create`, tienen su propia documentación de uso.

## Usage
- `conda create -n nombre_del_entorno python=3.9 matplotlib` : crea un nuevo entorno e instala en él los paquetes indicados
- `conda info -e` : lista todos los entornos
- `conda activate nombre_del_entorno` : activa un entorno
- `conda deactivate` : desactiva un entorno
- `conda remove -n nombre_del_entorno --all` : elimina un entorno (remueve todos los paquetes)
- `conda install python=3.4 numpy` : instala paquetes en el entorno actual
- `conda list` : lista los paquetes instalados en el entorno actual
- `conda clean -a` : elimina paquetes no usados y cachés

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/index.html
