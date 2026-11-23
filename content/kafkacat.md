# kafkacat

This command is an alias of `kcat`.

## Usage
- `kcat -C -t topic -b brokers` : consume messages starting with the newest offset
- `kcat -C -t topic -b brokers -o beginning -e` : consume messages starting with the oldest offset and exit after the last message is received
- `kcat -G group_id topic -b brokers` : consume messages as a Kafka consumer group
- `echo message | kcat -P -t topic -b brokers` : publish message by reading from `stdin`
- `kcat -P -t topic -b brokers path/to/file` : publish messages by reading from a file
- `kcat -L -b brokers` : list metadata for all topics and brokers
- `kcat -L -t topic -b brokers` : list metadata for a specific topic
- `kcat -Q -t topic:partition:unix_timestamp -b brokers` : get offset for a topic/partition for a specific point in time

## Categories
files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/kafkacat
