# rabbitmq-upgrade

Управлять обновлением узла RabbitMQ.

## Usage
- `rabbitmq-upgrade await_online_quorum_plus_one` : дождаться, пока все очереди кворума получат онлайн-кворум выше минимального
- `rabbitmq-upgrade drain` : перевести узел в режим обслуживания
- `rabbitmq-upgrade revive` : вывести узел из режима обслуживания в обычный рабочий режим
- `rabbitmq-upgrade post_upgrade` : выполнить задачи после обновления
- `rabbitmq-upgrade help` : показать справку

## Links
- More information: https://www.rabbitmq.com/docs/man/rabbitmq-upgrade.8
