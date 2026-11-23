# john

Outil de cassage de mot de passe.

## Usage
- `john chemin/vers/hashs.txt` : craque les hashs de mots de passe
- `john --show chemin/vers/hashs.txt` : affiche les mots de passe cassés
- `john --show --users=ids_utilisateurs chemin/vers/hashs1.txt chemin/vers/autres/hashs2.txt ...` : affiche les mots de passe cassés des utilisateurs par identifiant d'utilisateur à partir de plusieurs fichiers
- `john --wordlist=chemin/vers/liste_de_mots.txt chemin/vers/hashs.txt` : craque des hashs de mots de passe, à l'aide d'une liste de mots personnalisée
- `john --list=formats` : liste des formats de hachage disponibles
- `john --format=md5crypt chemin/vers/hashs.txt` : craque les hashs de mots de passe, en utilisant un format de hash spécifique
- `john --rules chemin/vers/hashs.txt` : craque les hashs de mots de passe, en activant les règles d'altération de mots
- `john --restore=chemin/vers/mon_cassage.rec` : restaure une session de craquage interrompue à partir d'un fichier d'état, par exemple `mon_cassage.rec`

## Links
- More information: https://www.openwall.com/john/
