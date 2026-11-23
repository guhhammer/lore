# aws rds

CLI AWS pour Relational Database Service (RDS), service web permettant de configurer, d’exploiter et de faire évoluer des bases de données relationnelles.

## Usage
- `aws rds sous_commande help` : affiche l'aide pour une sous-commande RDS donnée
- `aws rds stop-db-instance --db-instance-identifier identifiant_de_l_instance` : stoppe une instance
- `aws rds start-db-instance --db-instance-identifier identifiant_de_l_instance` : démarre une nouvelle instance
- `aws rds modify-db-instance --db-instance-identifier identifiant_de_l_instance paramètres --apply-immediately` : modifie une instance RDS
- `aws rds apply-pending-maintenance-action --resource-identifier arn_de_la_base_de_données --apply-action mise_à_jour_du_système --opt-in-type immediate` : applique les mises à jour à une instance RDS
- `aws rds modify-db-instance --db-instance-identifier ancien_identifiant_de_l_instance --new-db-instance-identifier nouvel_identifiant_de_l_instance` : change l'identifiant d'une instance
- `aws rds reboot-db-instance --db-instance-identifier identifiant_de_l_instance` : redémarre une instance
- `aws rds delete-db-instance --db-instance-identifier identifiant_de_l_instance --final-db-snapshot-identifier identifiant_de_l_image --delete-automated-backups` : supprime une instance

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/rds/
