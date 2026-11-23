# rabbitmqctl-vhosts

Gestiona los hosts virtuales (vhosts) en RabbitMQ. Los vhosts se utilizan para separar varios brokers lógicos en el mismo servidor de RabbitMQ.

## Usage
- `rabbitmqctl list_vhosts` : muestra todos los hosts virtuales
- `rabbitmqctl add_vhost vhost_nombre` : añade un nuevo host virtual
- `rabbitmqctl delete_vhost vhost_nombre` : elimina un host virtual
- `rabbitmqctl set_permissions -p vhost_nombre usuario read write configure` : establece permisos para un usuario en un host virtual específico
- `rabbitmqctl clear_permissions -p vhost_nombre usuario` : borra los permisos de un usuario en un host virtual específico

## Links
- More information: https://www.rabbitmq.com/docs/vhosts
