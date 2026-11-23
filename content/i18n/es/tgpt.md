# tgpt

Habla con un chatbot de IA sin necesidad de claves API.

## Usage
- `tgpt "prompt"` : chatea con el proveedor por defecto (GPT-3.5-turbo)
- `tgpt -m` : inicia el modo interactivo multilínea
- `tgpt -img "prompt"` : genera imágenes y las guarda en el directorio actual
- `tgpt -c "prompt"` : genera código con el proveedor por defecto (GPT-3.5-turbo)
- `tgpt --provider openai|opengpts|koboldai|phind|llama2|blackboxai -q -w "prompt"` : chatea con un proveedor específico silenciosamente (sin animaciones)
- `tgpt --provider llama2 -s "prompt"` : genera y ejecuta comandos de intérprete utilizando un proveedor específico (con confirmación)
- `tgpt --provider openai --key "api_key" --model "gpt-3.5-turbo" --max-length 10 --temperature 0.7 --top_p 0.9 "prompt"` : pregunta con una clave de API, modelo, longitud máxima de respuesta, temperatura y `top_p` (necesario cuando se utiliza el proveedor `openai`)
- `tgpt <ruta/al/archivo --provider blackboxai "prompt"` : alimenta un archivo como pre-entrada adicional

## Links
- More information: https://github.com/aandrew-me/tgpt
