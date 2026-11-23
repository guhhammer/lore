# rabbitmq-diagnostics

Выполнять диагностику, мониторинг и проверки работоспособности узлов RabbitMQ. Многие подкоманды делегируются команде `rabbitmqctl`.

## Usage
- `rabbitmq-diagnostics alarms` : вывести список тревог ресурсов
- `rabbitmq-diagnostics certificates` : вывести список сертификатов узла
- `rabbitmq-diagnostics check_running --node узел` : проверить, запущен ли RabbitMQ на указанном узле
- `rabbitmq-diagnostics discover_peers` : запустить обнаружение одноранговых узлов
- `rabbitmq-diagnostics listeners` : вывести список слушателей (привязанных сокетов)
- `rabbitmq-diagnostics log_tail --number n --node узел` : вывести последние `n` строк журнала на указанном узле

## Links
- More information: https://www.rabbitmq.com/docs/man/rabbitmq-diagnostics.8
