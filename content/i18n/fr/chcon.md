# chcon

Change le contexte de sécurité de SELinux d'un ou plusieurs fichiers/dossiers.

## Usage
- `ls -lZ chemin/vers/fichier` : affiche le contexte de sécurité d'un fichier
- `chcon --reference fichier_référence fichier_cible` : change le contexte de sécurité d'un fichier cible, en utilisant un fichier de référence
- `chcon utilisateur:role:type:range/level fichier` : change le contexte de sécurité SELinux complet d'un fichier
- `chcon -u utilisateur fichier` : change seulement la partie utilisateur du contexte de sécurité SELinux
- `chcon -r role fichier` : change seulement la partie role du contexte de sécurité SELinux
- `chcon -t type fichier` : change seulement la partie type du contexte de sécurité SELinux
- `chcon -l plage/niveau fichier` : change seulement la partie plage/niveau du contexte de sécurité SELinux
