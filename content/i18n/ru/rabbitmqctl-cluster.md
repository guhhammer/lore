# rabbitmqctl-cluster

Управлять узлами RabbitMQ в кластере.

## Usage
- `rabbitmqctl cluster_status` : показать статус кластера
- `rabbitmqctl status` : показать статус текущего узла
- `rabbitmqctl -n имя_узла start_app` : запустить приложение RabbitMQ на конкретном узле
- `rabbitmqctl -n имя_узла stop_app` : остановить приложение RabbitMQ на конкретном узле
- `rabbitmqctl -n имя_узла stop` : остановить конкретный узел RabbitMQ
- `rabbitmqctl -n имя_узла reset` : сбросить конкретный узел RabbitMQ в чистое состояние
- `rabbitmqctl join_cluster имя_узла` : присоединить текущий узел к существующему кластеру

## Links
- More information: https://www.rabbitmq.com/docs/man/rabbitmqctl.8
