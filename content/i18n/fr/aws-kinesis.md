# aws kinesis

CLI officiel d'AWS pour les services de streaming d'Amazon Kinesis.

## Usage
- `aws kinesis list-streams` : affiche tous les streams du compte
- `aws kinesis put-record --stream-name nom --partition-key clé --data message_encodé_en_base64` : écris une entrée dans le stream Kinesis
- `aws kinesis put-record --stream-name nom --partition-key clé --data "$( echo "mon message" | base64 )"` : écris une entrée dans le stream Kinesis avec un encodage Base64 inline
- `aws kinesis list-shards --stream-name nom` : liste tous les fragments disponible dans un stream
- `aws kinesis get-shard-iterator --shard-iterator-type TRIM_HORIZON --stream-name nom --shard-id id` : récupère un fragment pour lire depuis le plus vieux message dans la stream de ce dernier
- `aws kinesis get-records --shard-iterator itérateur` : lis les entrées d'un fragment en utilisant un itérateur de fragment
