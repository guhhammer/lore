# aws kinesis

CLI oficial de AWS para los servicios de streaming de datos de Amazon Kinesis.

## Usage
- `aws kinesis list-streams` : muestra todos los streams de la cuenta
- `aws kinesis put-record --stream-name nombre --partition-key clave --data base64_encoded_message` : escribe un registro en un flujo de Kinesis
- `aws kinesis put-record --stream-name nombre --partition-key clave --data "$( echo "my raw message" | base64 )"` : escribe un registro en un flujo Kinesis con codificación Base64 en línea
- `aws kinesis list-shards --stream-name nombre` : lista los fragmentos disponibles en un flujo
- `aws kinesis get-shard-iterator --shard-iterator-type TRIM_HORIZON --stream-name nombre --shard-id id` : obtén un iterador de fragmentos para leer el mensaje más antiguo de un fragmento de flujo
- `aws kinesis get-records --shard-iterator iterador` : lee registros de un fragmento utilizando un iterador de fragmento
