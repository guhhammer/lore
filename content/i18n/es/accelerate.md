# accelerate

Una biblioteca que permite ejecutar el mismo código PyTorch en cualquier configuración distribuida.

## Usage
- `accelerate env` : imprime información del entorno
- `accelerate config` : crea interactivamente un archivo de configuración
- `accelerate estimate-memory nombre/modelo` : imprime el coste estimado en memoria de la GPU al ejecutar un modelo Hugging Face con distintos tipos de datos
- `accelerate test --config_file ruta/a/config.yaml` : prueba un archivo de configuración de Accelerate
- `accelerate launch ruta/a/script.py --cpu` : ejecuta un modelo en CPU con Accelerate
- `accelerate launch ruta/a/script.py --multi_gpu --num_machines 2` : ejecuta un modelo en multi-GPU con Accelerate, con 2 máquinas

## Links
- More information: https://huggingface.co/docs/accelerate/index
