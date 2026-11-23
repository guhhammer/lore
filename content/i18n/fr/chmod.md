# chmod

Modifie les droits d'accès d'un fichier ou d'un répertoire.

## Usage
- `chmod u+x chemin/vers/fichier` : donne les droits d'exécution à l'utilisateur auquel le fichier appartient
- `chmod u+rw chemin/vers/fichier_ou_répertoire` : donne à l'utilisateur les droits de lecture r et d'écriture w sur un fichier/répertoire
- `chmod g-x chemin/vers/fichier` : enlève les droits d'exécution pour le groupe
- `chmod a+rx chemin/vers/fichier` : donne à tous a les utilisateurs les droits de lecture r et d'exécution
- `chmod o=g chemin/vers/fichier` : donne aux autres o utilisateurs (qui sont dans un autre groupe) les mêmes droits que ceux du groupe propriétaire
- `chmod o= chemin/vers/fichier` : retire tous les droits aux autres o utilisateurs
- `chmod -R g+w,o+w chemin/vers/répertoire` : modifie les permissions récursivement en donnant aux membres du groupe et aux autres o utilisateurs le droit d'écriture w
- `chmod -R a+rX chemin/vers/répertoire` : donne récursivement à tous a les utilisateurs les droits de lecture r de fichiers et d'eXécution de sous-répertoires à l'intérieur d'un répertoire
