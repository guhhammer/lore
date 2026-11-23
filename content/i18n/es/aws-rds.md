# aws rds

Utiliza AWS Relational Database Service, un servicio web para configurar, operar y escalar bases de datos relacionales.

## Usage
- `aws rds subcommand help` : muestra ayuda para subcomando RDS específicos
- `aws rds stop-db-instance --db-instance-identifier identificador_de_instancia` : detiene instancia
- `aws rds start-db-instance --db-instance-identifier identificador_de_instancia` : inicia instancia
- `aws rds modify-db-instance --db-instance-identifier identificador_de_instancia parametros --apply-immediately` : modifica una instancia RDS
- `aws rds apply-pending-maintenance-action --resource-identifier database_arn --apply-action system-update --opt-in-type immediate` : aplica actualizaciones a una instancia RDS
- `aws rds modify-db-instance --db-instance-identifier antiguo_identificador_instancia --new-db-instance-identifier nuevo_identificador_instancia` : modifica un identificador de instancia
- `aws rds reboot-db-instance --db-instance-identifier identificador_de_instancia` : reinicia una instancia
- `aws rds delete-db-instance --db-instance-identifier identificador_de_instancia --final-db-snapshot-identifier identificador_snapshot --delete-automated-backups` : elimina una instancia

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/rds/
