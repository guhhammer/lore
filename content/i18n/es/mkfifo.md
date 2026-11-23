# mkfifo

Crea FIFOs (named pipes) (pipes nombrados).

## Usage
- `mkfifo ruta/al/pipe` : crea un pipe nombrado en una ruta específica
- `echo "Hola Mundo" >ruta/al/pipe &` : envía datos a través de un pipe nombrado ejecutando el comando en segundo plano
- `cat ruta/al/pipe` : recibe datos a través de un pipe nombrado
- `mkfifo ruta/al/pipe; script -f ruta/al/pipe` : comparte tu sesión de la terminal en tiempo real
