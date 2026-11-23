# rabbitmq-queues

Manage RabbitMQ queues.

## Usage
- `rabbitmq-queues grow node all|even` : grow quorum queue clusters by adding a member on the specified node
- `rabbitmq-queues rebalance all|quorum|stream` : rebalance leaders of replicated queues across nodes
- `rabbitmq-queues shrink node` : shrink quorum queue clusters by removing any members on the specified node
- `rabbitmq-queues add_member queue node` : add a quorum queue member on the specified node
- `rabbitmq-queues delete_member queue node` : delete a quorum queue member on the specified node
- `rabbitmq-queues quorum_status queue` : display quorum status of a quorum queue

## Links
- More information: https://www.rabbitmq.com/docs/man/rabbitmq-queues.8
- tldr page: https://tldr.inbrowser.app/pages/common/rabbitmq-queues
