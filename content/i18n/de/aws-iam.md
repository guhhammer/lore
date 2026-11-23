# aws iam

CLI für AWS IAM.

## Usage
- `aws iam list-users` : liste aller Benutzer auf
- `aws iam list-policies` : liste aller Richtlinien auf
- `aws iam list-groups` : liste aller Gruppen auf
- `aws iam get-group --group-name gruppe` : liste aller Benutzer zu einer Gruppe auf
- `aws iam get-policy --policy-arn arn:aws:iam::aws:policy/richtlinie` : liste einer IAM Richtlinie detailliert auf
- `aws iam list-access-keys` : liste alle Zugriffsschlüssel auf
- `aws iam list-access-keys --user-name benutzername` : liste alle Zugriffsschlüssel für einen Benutzer auf
- `aws iam help` : zeige Hilfe an

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/iam/
