# rabbitmq-diagnostics

Diagnose, monitor, and run health checks on RabbitMQ nodes. Many subcommands are delegated to `rabbitmqctl`.

## Usage
- `rabbitmq-diagnostics alarms` : list resource alarms
- `rabbitmq-diagnostics certificates` : list node certificates
- `rabbitmq-diagnostics check_running --node node` : check if RabbitMQ is running on the specified node
- `rabbitmq-diagnostics discover_peers` : run peer discovery
- `rabbitmq-diagnostics listeners` : list listeners (bound sockets)
- `rabbitmq-diagnostics log_tail --number n --node node` : print the last `n` log lines on the specified node

## Categories
monitoring

## Links
- More information: https://www.rabbitmq.com/docs/man/rabbitmq-diagnostics.8
- tldr page: https://tldr.inbrowser.app/pages/common/rabbitmq-diagnostics
