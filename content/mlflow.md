# mlflow

A platform to track experiments, manage models, and deploy machine learning projects.

## Usage
- `mlflow ui --port 5000` : start only the local web interface
- `mlflow server --host 0.0.0.0 --port 5000` : start a server accessible from the local network
- `mlflow server --backend-store-uri sqlite:///mlflow.db --artifacts-destination ./mlartifacts` : start a server by specifying URIs for experiment data and artifacts
- `mlflow db upgrade sqlite:///mlflow.db` : upgrade the schema of an MLflow database after an MLflow update
- `mlflow run . --entry-point name --parameter learning_rate=0.001 --parameter epochs=20` : run an entry point with custom parameters
- `mlflow artifacts download --run-id run_id --dst-path ./artifacts` : download all artifacts from a run
- `mlflow models serve --model-uri runs:/run_id/model --port 5001` : start an inference server for a model saved in a run

## Categories
network, databases

## Links
- More information: https://mlflow.org/docs/latest/api_reference/cli.html
- tldr page: https://tldr.inbrowser.app/pages/common/mlflow
