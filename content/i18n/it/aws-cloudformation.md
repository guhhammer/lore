# aws cloudformation

Modella, provisiona e gestisce risorse AWS e di terze parti trattando l'infrastruttura come codice.

## Usage
- `aws cloudformation create-stack --stack-name nome_stack --region regione --template-body file://percorso/del/file.yml --profile profilo` : crea uno stack da un file template
- `aws cloudformation delete-stack --stack-name nome_stack --profile profilo` : elimina uno stack
- `aws cloudformation list-stacks --profile profilo` : elenca tutti gli stack
- `aws cloudformation list-stacks --stack-status-filter CREATE_COMPLETE --profile profilo` : elenca tutti gli stack in esecuzione
- `aws cloudformation describe-stacks --stack-name id_stack --profile profilo` : controlla lo stato di uno stack
- `aws cloudformation detect-stack-drift --stack-name id_stack --profile profilo` : avvia il rilevamento deriva per uno stack
- `aws cloudformation describe-stack-resource-drifts --stack-name stack_drift_detection_id --profile profilo` : controlla l'output dello stato deriva di uno stack usando `StackDriftDetectionId` dall'output del comando precedente

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/cloudformation/
