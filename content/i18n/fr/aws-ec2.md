# aws ec2

CLI pour AWS EC2. Provisionne, sécurise et des capacités de calcul redimensionnable dans le cloud AWS pour accélérer le développement et le déploiement d'applications.

## Usage
- `aws ec2 describe-instances --instance-ids id_d_instance` : affiche les informations sur une instance spécifique
- `aws ec2 describe-instances` : affiche les informations de toutes les instances
- `aws ec2 describe-volumes` : affiche les informations sur tous les volumes EC2
- `aws ec2 delete-volume --volume-id id_de_volume` : supprime un volume EC2
- `aws ec2 create-snapshot --volume-id id_de_volume` : crée une sauvegarde de votre volume EC2
- `aws ec2 describe-images` : liste toutes les AMIs (Images de Machine Amazon) disponible
- `aws ec2 help` : affiche la liste de toutes les commandes EC2 disponible
- `aws ec2 sous-commande help` : affiche l'aide pour une sous-commande EC2 spécifique

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ec2/
