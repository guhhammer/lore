# rabbitmq-queues

Управлять очередями RabbitMQ.

## Usage
- `rabbitmq-queues grow узел all|even` : расширить кластеры очередей кворума, добавив участника на указанном узле
- `rabbitmq-queues rebalance all|quorum|stream` : перераспределить лидеров реплицированных очередей между узлами
- `rabbitmq-queues shrink узел` : сократить кластеры очередей кворума, удалив участников на указанном узле
- `rabbitmq-queues add_member очередь узел` : добавить участника очереди кворума на указанном узле
- `rabbitmq-queues delete_member очередь узел` : удалить участника очереди кворума на указанном узле
- `rabbitmq-queues quorum_status очередь` : показать статус кворума для очереди кворума

## Links
- More information: https://www.rabbitmq.com/docs/man/rabbitmq-queues.8
