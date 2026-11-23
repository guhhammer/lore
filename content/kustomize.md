# kustomize

Easily deploy resources for Kubernetes.

## Usage
- `kustomize create --resources deployment.yaml,service.yaml --namespace staging` : create a kustomization file with resources and namespace
- `kustomize build . | kubectl apply -f -` : build a kustomization file and deploy it with `kubectl`
- `kustomize edit set image busybox=alpine:3.6` : set an image in the kustomization file
- `kustomize create --autodetect` : search for Kubernetes resources in the current directory to be added to the kustomization file

## Categories
containers, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/kustomize
