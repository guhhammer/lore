# msg

Envía un mensaje a un usuario o sesión.

## Usage
- `msg nombre_de_usuario|nombre_de_sesión|id_de_sesión mensaje` : envía un mensaje a un usuario o sesión especificada
- `echo "mensaje" | msg nombre_de_usuario|nombre_de_sesión|id_de_sesión` : envía un mensaje desde `stdin`
- `msg /server:nombre_del_servidor nombre_de_usuario|nombre_de_sesión|id_de_sesión` : envía un mensaje a un servidor específico
- `msg *` : envía un mensaje a todos los usuarios de la máquina actual
- `msg /time:10` : establece un retraso en segundos para un mensaje
