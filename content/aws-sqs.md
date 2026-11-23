# aws sqs

Create, delete, and send messages to queues for the AWS SQS service.

## Usage
- `aws sqs list-queues` : list all available queues
- `aws sqs get-queue-url --queue-name queue_name` : display the URL of a specific queue
- `aws sqs create-queue --queue-name queue_name --attributes file://path/to/attributes_file.json` : create a queue with specific attributes from a file in JSON format
- `aws sqs send-message --queue-url https://sqs.region.amazonaws.com/queue_name --message-body "message_body" --delay-seconds delay --message-attributes file://path/to/attributes_file.json` : send a specific message to a queue
- `aws sqs delete-message --queue-url https://queue_url --receipt-handle receipt_handle` : delete the specified message from a queue
- `aws sqs delete-queue --queue-url https://sqs.region.amazonaws.com/queue_name` : delete a specific queue
- `aws sqs purge-queue --queue-url https://sqs.region.amazonaws.com/queue_name` : delete all messages from the specified queue
- `aws sqs add-permission --queue-url https://sqs.region.amazonaws.com/queue_name --label permission_name --aws-account-ids account_id --actions SendMessage` : enable a specific AWS account to send messages to queue

## Categories
cloud, network, files

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/sqs/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-sqs
