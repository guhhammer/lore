# chatgpt

Shell script para usar ChatGPT de OpenAI y DALL·E desde la terminal.

## Usage
- `chatgpt` : comienza en modo chat
- `chatgpt -p "¿Cuál es la expresión regular para emparejar una dirección de correo electrónico?"` : da un prompt para responder
- `chatgpt -m gpt-4` : inicia en modo chat utilizando un modelo específico (por defecto es `gpt-3.5-turbo`)
- `chatgpt -i "Tú eres Rick, de Rick y Morty. Responde a las preguntas usando su amaneramiento e incluye chistes insultantes."` : inicia en modo chat con un prompt inicial
- `echo "¿Cómo ver los procesos en ejecución en Ubuntu?" | chatgpt` : envía el resultado de un comando a ChatGPT como un prompt
- `chatgpt -p "image: Un gato blanco"` : genera una imagen utilizando DALL·E

## Links
- More information: https://github.com/0xacx/chatGPT-shell-cli
