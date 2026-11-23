# rabbitmq-streams

Управлять потоками RabbitMQ.

## Usage
- `rabbitmq-streams add_replica поток узел` : добавить реплику потока на указанном узле
- `rabbitmq-streams delete_replica поток узел` : удалить реплику потока на указанном узле
- `rabbitmq-streams stream_status поток` : показать статус потока
- `rabbitmq-streams restart_stream поток` : перезапустить поток
- `rabbitmq-streams list_stream_connections` : вывести список подключений потоков
- `rabbitmq-streams list_stream_consumers` : вывести список всех потребителей потоков
- `rabbitmq-streams list_stream_publishers` : вывести список всех издателей потоков
- `rabbitmq-streams list_stream_tracking поток` : вывести информацию отслеживания для потока

## Links
- More information: https://www.rabbitmq.com/docs/man/rabbitmq-streams.8
