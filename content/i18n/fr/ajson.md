# ajson

Exécute un JSONPath sur un objet JSON.

## Usage
- `ajson '$..json[?(@.path)]' chemin/vers/fichier.json` : lit un JSON depuis un fichier et exécute une expression JSONPath spécifique
- `cat chemin/vers/fichier.json | ajson '$..json[?(@.path)]'` : lit un JSON depuis l'entrée standard et exécute une expression JSONPath spécifique
- `ajson 'avg($..price)' 'https://example.com/api/'` : lit un JSON depuis une URL et évalue une expression JSONPath spécifique
- `echo '3' | ajson '2 * pi * $'` : lit un JSON simple et calcule une valeur

## Links
- More information: https://github.com/spyzhov/ajson#console-application
