# rabbitmqctl-cluster

Manage RabbitMQ nodes in a cluster.

## Usage
- `rabbitmqctl cluster_status` : display the status of the cluster
- `rabbitmqctl status` : display the status of the current node
- `rabbitmqctl -n nodename start_app` : start the RabbitMQ application on a specific node
- `rabbitmqctl -n nodename stop_app` : stop the RabbitMQ application on a specific node
- `rabbitmqctl -n nodename stop` : stop a specific RabbitMQ node
- `rabbitmqctl -n nodename reset` : reset a specific RabbitMQ node to a clean state
- `rabbitmqctl join_cluster nodename` : make the current node join an existing cluster

## Categories
containers

## Links
- More information: https://www.rabbitmq.com/docs/man/rabbitmqctl.8
- tldr page: https://tldr.inbrowser.app/pages/common/rabbitmqctl-cluster
