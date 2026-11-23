# install

Copie des fichiers et met à jour leurs attributs. Copie des fichiers (souvent des exécutables) dans un répertoire système comme `/usr/local/bin` et leur donne les permissions et propriétaires appropriés.

## Usage
- `install chemin/fichier/source chemin/repertoire/destination` : copie des fichiers vers une destination
- `install --owner utilisateur chemin/fichier/source chemin/repertoire/destination` : copie des fichiers vers une destination en mettant à jour leur propriétaire
- `install --group utilisateur chemin/fichier/source chemin/repertoire/destination` : copie des fichiers vers une destination en mettant à jour leur groupe d'appartenance
- `install --mode +x chemin/fichier/source chemin/repertoire/destination` : copie des fichiers vers une destination en mettant à jour leur mode
- `install --preserve-timestamps chemin/fichier/source chemin/repertoire/destination` : copie des fichiers en mettant à jour leur dates d'accès et de modification à partir de leurs sources respectives
