# rabbitmqctl-cluster

Gestiona los nodos de RabbitMQ en un clúster.

## Usage
- `rabbitmqctl cluster_status` : muestra el estado del clúster
- `rabbitmqctl status` : muestra el estado del nodo actual
- `rabbitmqctl -n nombre_del_nodo start_app` : inicia la aplicación RabbitMQ en un nodo específico
- `rabbitmqctl -n nombre_del_nodo stop_app` : detiene la aplicación RabbitMQ en un nodo específico
- `rabbitmqctl -n nombre_del_nodo stop` : detiene un nodo RabbitMQ específico
- `rabbitmqctl -n nombre_del_nodo reset` : restablece un nodo RabbitMQ específico a un estado limpio
- `rabbitmqctl join_cluster nombre_del_nodo` : hace que el nodo actual se una a un clúster existente

## Links
- More information: https://www.rabbitmq.com/docs/man/rabbitmqctl.8
