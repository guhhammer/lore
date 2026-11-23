# aws rds

Use AWS Relational Database Service, a web service for setting up, operating, and scaling relational databases.

## Usage
- `aws rds subcommand help` : display help for a specific RDS subcommand
- `aws rds stop-db-instance --db-instance-identifier instance_identifier` : stop instance
- `aws rds start-db-instance --db-instance-identifier instance_identifier` : start instance
- `aws rds modify-db-instance --db-instance-identifier instance_identifier parameters --apply-immediately` : modify an RDS instance
- `aws rds apply-pending-maintenance-action --resource-identifier database_arn --apply-action system-update --opt-in-type immediate` : apply updates to an RDS instance
- `aws rds modify-db-instance --db-instance-identifier old_instance_identifier --new-db-instance-identifier new_instance_identifier` : change an instance identifier
- `aws rds reboot-db-instance --db-instance-identifier instance_identifier` : reboot an instance
- `aws rds delete-db-instance --db-instance-identifier instance_identifier --final-db-snapshot-identifier snapshot_identifier --delete-automated-backups` : delete an instance

## Categories
cloud, system, databases

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/rds/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-rds
