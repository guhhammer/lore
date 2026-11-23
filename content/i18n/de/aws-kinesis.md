# aws kinesis

Offizielles AWS CLI für die Amazon Kinesis-Streaming-Datenplattform.

## Usage
- `aws kinesis list-streams` : liste alle Streams auf
- `aws kinesis put-record --stream-name name --partition-key schlüssel --data base64_codierte_nachricht` : schreibe einen Datensatz in einen Kinesis Stream
- `aws kinesis put-record --stream-name name --partition-key schlüssel --data "$( echo "meine nachricht" | base64 )"` : schreibe einen Datensatz in einen Kinesis Stream mit Base64 inline Encodierung
- `aws kinesis list-shards --stream-name name` : liste alle verfügbaren Shards in einem Stream auf
- `aws kinesis get-shard-iterator --shard-iterator-type TRIM_HORIZON --stream-name name --shard-id id` : rufe einen Shard Iterators auf, um diesen beginnend mit der ältesten Nachricht auszulesen
- `aws kinesis get-records --shard-iterator iterator` : lies einen Datensatz aus einem Shard über einen Shard Iterator
