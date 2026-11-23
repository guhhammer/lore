# rabbitmqctl-vhosts

Manage virtual hosts (vhosts) in RabbitMQ. Vhosts are used to separate multiple logical brokers on the same RabbitMQ server.

## Usage
- `rabbitmqctl list_vhosts` : list all virtual hosts
- `rabbitmqctl add_vhost vhost_name` : add a new virtual host
- `rabbitmqctl delete_vhost vhost_name` : delete a virtual host
- `rabbitmqctl set_permissions -p vhost_name username read write configure` : set permissions for a user on a specific virtual host
- `rabbitmqctl clear_permissions -p vhost_name username` : clear permissions for a user on a specific virtual host

## Categories
network

## Links
- More information: https://www.rabbitmq.com/docs/vhosts
- tldr page: https://tldr.inbrowser.app/pages/common/rabbitmqctl-vhosts
