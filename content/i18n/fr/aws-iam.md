# aws iam

CLI pour AWS IAM.

## Usage
- `aws iam list-users` : liste les utilisateurs
- `aws iam list-policies` : liste les politiques
- `aws iam list-groups` : liste les groupes
- `aws iam get-group --group-name nom_groupe` : récupère les utilisateurs dans un groupe
- `aws iam get-policy --policy-arn arn:aws:iam::aws:policy/nom_politique` : décris une politique IAM
- `aws iam list-access-keys` : liste les clés d’accès
- `aws iam list-access-keys --user-name nom_utilisateur` : liste les clés d'accès pour un utilisateur spécifique
- `aws iam help` : affiche l'aide

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/iam/
