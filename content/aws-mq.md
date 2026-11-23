# aws mq

Define and operate message brokers in AWS.

## Usage
- `aws mq create-broker --host-instance-type instance_type --broker-name broker_name --engine-type ACTIVEMQ|RABBITMQ --publicly-accessible|--no-publicly-accessible` : create a broker
- `aws mq list-brokers` : list all brokers
- `aws mq describe-broker --broker-id broker_id` : describe a specific broker

## Categories
cloud, network

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/mq/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-mq
