# gcloud

La herramienta CLI oficial de Google Cloud Platform. Algunos subcomandos, como `app` e `init`, tienen su propia documentación de uso.

## Usage
- `gcloud config list` : lista todas las propiedades de la configuración activa
- `gcloud auth login` : inicia sesión en la cuenta de Google
- `gcloud config set project nombre_del_proyecto` : establece como proyecto activo
- `gcloud compute ssh usuario@instancia` : SSH en una instancia de máquina virtual
- `gcloud compute instances list` : muestra todas las instancias de Google Compute Engine de un proyecto. Por defecto, se muestran las instancias de todas las zonas
- `gcloud container clusters get-credentials nombre_cluster` : actualiza un archivo kubeconfig con las credenciales adecuadas para apuntar kubectl a un clúster específico en Google Kubernetes Engine
- `gcloud components update` : actualiza todos los componentes de la CLI de gcloud
- `gcloud help comando` : muestra la ayuda para un comando determinado

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud
