# aws kinesis

Interact with the Amazon Kinesis Data Streams, a service that scales elastically for real-time processing of streaming big data.

## Usage
- `aws kinesis list-streams` : show all streams in the account
- `aws kinesis put-record --stream-name name --partition-key key --data base64_encoded_message` : write one record to a Kinesis stream
- `aws kinesis put-record --stream-name name --partition-key key --data "$( echo "my raw message" | base64 )"` : write a record to a Kinesis stream with inline Base64 encoding
- `aws kinesis list-shards --stream-name name` : list the shards available on a stream
- `aws kinesis get-shard-iterator --shard-iterator-type TRIM_HORIZON --stream-name name --shard-id shard_id` : get a shard iterator for reading from the oldest message in a stream's shard
- `aws kinesis get-records --shard-iterator iterator` : read records from a shard, using a shard iterator

## Categories
media, cloud, system

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/aws-kinesis
