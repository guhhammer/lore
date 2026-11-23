# sqsc

An AWS Simple Queue Service client.

## Usage
- `sqsc lq queue_prefix` : list all queues
- `sqsc ls queue_name` : list all messages in a queue
- `sqsc cp source_queue destination_queue` : copy all messages from one queue to another
- `sqsc mv source_queue destination_queue` : move all messages from one queue to another
- `sqsc describe queue_name` : describe a queue
- `sqsc query "SELECT body FROM queue_name WHERE body LIKE '%user%'"` : query a queue with SQL syntax
- `sqsc pull queue_name` : pull all messages from a queue into a local SQLite database in your present working directory

## Categories
databases, cloud, files

## Links
- More information: https://github.com/yongfei25/sqsc
- tldr page: https://tldr.inbrowser.app/pages/common/sqsc
