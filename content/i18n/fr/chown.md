# chown

Modifie l'utilisateur et le groupe propriétaire des fichiers et dossiers.

## Usage
- `chown utilisateur chemin/vers/fichier_ou_dossier` : modifie le propriétaire d'un fichier/dossier
- `chown utilisateur:groupe chemin/vers/fichier_ou_dossier` : modifie l'utilisateur et le groupe propriétaire d'un fichier/dossier
- `chown utilisateur: chemin/vers/fichier_ou_dossier` : modifie le propriétaire et le groupe pour qu'ils aient tous les deux le nom `utilisateur`
- `chown -R utilisateur chemin/vers/dossier` : modifie récursivement le propriétaire d'un dossier et de son contenu
- `chown -h utilisateur chemin/vers/lien_symbolique` : modifie le propriétaire d'un lien symbolique
- `chown --reference chemin/vers/fichier_de_référence chemin/vers/fichier_ou_dossier` : modifie le propriétaire d'un fichier / dossier pour correspondre à un fichier de référence
