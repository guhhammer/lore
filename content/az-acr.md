# az acr

Manage private registries with Azure Container Registries. Part of `azure-cli` (also known as `az`).

## Usage
- `az acr create -n registry_name -g resource_group --sku sku` : create a managed container registry
- `az acr login -n registry_name` : log in to a registry
- `docker tag image_name registry_name.azurecr.io/image_name:tag` : tag a local image for ACR
- `docker push registry_name.azurecr.io/image_name:tag` : push an image to a registry
- `docker pull registry_name.azurecr.io/image_name:tag` : pull an image from a registry
- `az acr repository delete -n registry_name --repository image_name:tag` : delete an image from a registry
- `az acr delete -n registry_name -g resource_group -y` : delete a managed container registry
- `az acr repository list -n registry_name --output table` : list images within a registry

## Categories
containers, cloud, media

## Links
- More information: https://learn.microsoft.com/cli/azure/acr
- tldr page: https://tldr.inbrowser.app/pages/common/az-acr
