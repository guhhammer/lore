# rabbitmq-upgrade

Manage the upgrade of a RabbitMQ node.

## Usage
- `rabbitmq-upgrade await_online_quorum_plus_one` : wait for all quorum queues to have an above minimum online quorum
- `rabbitmq-upgrade drain` : put the node in maintenance mode
- `rabbitmq-upgrade revive` : put the node out of maintenance and into regular operating mode
- `rabbitmq-upgrade post_upgrade` : run post-upgrade tasks
- `rabbitmq-upgrade help` : display help

## Categories
packages

## Links
- More information: https://www.rabbitmq.com/docs/man/rabbitmq-upgrade.8
- tldr page: https://tldr.inbrowser.app/pages/common/rabbitmq-upgrade
