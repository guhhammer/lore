# mlflow

Plateforme de suivi d’expériences, de gestion de modèles et de déploiement pour les projets de machine learning.

## Usage
- `mlflow ui --port 5000` : démarre uniquement l’interface web locale
- `mlflow server --host 0.0.0.0 --port 5000` : démarre un serveur accessible depuis le réseau local
- `mlflow server --backend-store-uri sqlite:///mlflow.db --artifacts-destination ./mlartifacts` : démarre un serveur en spécifiant des URI pour les données d'expériences et artefacts
- `mlflow db upgrade sqlite:///mlflow.db` : met à niveau le schéma d’une base de données MLflow après une mise à jour de MLflow
- `mlflow run . --entry-point nom --parameter learning_rate=0.001 --parameter epochs=20` : exécute un point d’entrée avec des paramètres personnalisés
- `mlflow artifacts download --run-id id_exécution --dst-path ./artifacts` : télécharge tous les artefacts d’une exécution
- `mlflow models serve --model-uri runs:/id_exécution/modèle --port 5001` : démarre un serveur d’inférence pour un modèle enregistré dans une exécution

## Links
- More information: https://mlflow.org/docs/latest/api_reference/cli.html
