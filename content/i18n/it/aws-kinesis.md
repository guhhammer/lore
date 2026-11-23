# aws kinesis

Interagisce con Amazon Kinesis Data Streams, un servizio che scala elasticamente per l'elaborazione in tempo reale di big data in streaming.

## Usage
- `aws kinesis list-streams` : mostra tutti gli stream nell'account
- `aws kinesis put-record --stream-name nome --partition-key chiave --data messaggio_codificato_base64` : scrivi un record in uno stream Kinesis
- `aws kinesis put-record --stream-name nome --partition-key chiave --data "$( echo "mio messaggio raw" | base64 )"` : scrivi un record in uno stream Kinesis con codifica Base64 inline
- `aws kinesis list-shards --stream-name nome` : elenca i shard disponibili in uno stream
- `aws kinesis get-shard-iterator --shard-iterator-type TRIM_HORIZON --stream-name nome --shard-id id` : ottieni un iteratore shard per leggere dal messaggio più vecchio in uno shard dello stream
- `aws kinesis get-records --shard-iterator iteratore` : leggi record da uno shard, utilizzando un iteratore shard
