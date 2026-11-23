# rabbitmqctl-users

Gestiona los usuarios de RabbitMQ, sus permisos y sus etiquetas.

## Usage
- `rabbitmqctl list_users` : muestra todos los usuarios
- `rabbitmqctl add_user usuario contraseña` : añade un nuevo usuario con contraseña
- `rabbitmqctl delete_user usuario` : elimina un usuario existente
- `rabbitmqctl change_password usuario nueva_contraseña` : cambia la contraseña de un usuario
- `rabbitmqctl set_permissions -p vhost usuario read write configure` : establece permisos para un usuario en un host virtual específico
- `rabbitmqctl clear_permissions -p vhost usuario` : borra todos los permisos de un usuario en un host virtual específico
- `rabbitmqctl set_user_tags usuario etiqueta1 etiqueta2 ...` : asigna una o más etiquetas (por ejemplo, administrador) a un usuario

## Links
- More information: https://www.rabbitmq.com/docs/management
