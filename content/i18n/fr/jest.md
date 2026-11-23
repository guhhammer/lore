# jest

Une plateforme de test JavaScript sans configuration.

## Usage
- `jest` : exécute tous les tests disponibles
- `jest chemin/vers/fichier1 chemin/vers/fichier2 ...` : exécute les suites de test de fichiers donnés
- `jest regex1 regex2 ...` : exécute les suites de test pour des fichiers, dans le répertoire actuel et ses sous-répertoires, dont le chemin correspond à la `regex` indiquée
- `jest --testNamePattern regex` : exécute les tests dont les noms correspondent aux `regex` indiquées
- `jest --findRelatedTests chemin/vers/fichier_source.js` : exécute les suites de test associées à un fichier source donné
- `jest --onlyChanged` : exécute les suites de test associées à tous les fichiers non commités
- `jest --watch` : surveille les changements sur les fichiers et ré-exécute les tests associés
- `jest --help` : affiche l'aide

## Links
- More information: https://jestjs.io/docs/cli
