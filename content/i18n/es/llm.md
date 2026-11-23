# llm

Interactúa con modelos grandes de lenguaje (LLMs) a través de APIs y modelos remotos que pueden instalarse y ejecutarse en su máquina.

## Usage
- `llm keys set openai` : configura una clave API de OpenAI
- `llm "Diez nombres divertidos para un pelícano"` : ejecuta un prompt
- `cat ruta/al/archivo.py | llm -s "Explica este código"` : ejecuta un prompt de sistema contra un archivo
- `llm install paquete1 paquete2 ...` : instala paquetes de PyPI en el mismo entorno que LLM
- `llm -m orca-mini-3b-gguf2-q4_0 "¿Cuál es la capital de Francia?"` : descarga y ejecuta un prompt frente a un modelo
- `llm -s 'Eres una torta de queso sensible' --save torta_de_queso_sensible` : crea un prompt de sistema y lo salva como una plantilla
- `llm chat -m chatgpt -t torta_de_queso_sensible` : establece un chat interactivo con un modelo específico utilizando una plantilla específica

## Links
- More information: https://llm.datasette.io/en/stable/help.html
