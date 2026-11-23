# helm

Helm es un gestor de paquetes para Kubernetes. Algunos subcomandos como `install` tiene su propia documentación de uso.

## Usage
- `helm create nombre_del_chart` : crea un chart de helm
- `helm repo add nombre_del_repositorio` : añade un nuevo repositorio de helm
- `helm repo ls` : lista de repositorios de helm
- `helm repo up` : actualiza los repositorios de helm
- `helm repo rm nombre_del_repositorio` : elimina un repositorio de helm
- `helm install nombre nombre_del_repositorio/nombre_del_chart` : instala un chart de helm
- `helm get nombre_del_lanzamiento_del_chart` : descarga un chart de helm como un archivo `.tar`
- `helm dep up` : actualiza las dependencias de helm

## Links
- More information: https://helm.sh/docs/helm/
