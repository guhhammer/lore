# ko

Build and deploy Go applications as container images on Kubernetes.

## Usage
- `ko build path/to/import_directory` : build and publish a container image from a Go package import path
- `ko build -L path/to/import_directory` : build and load a container image into a local Docker daemon
- `ko apply -f path/to/manifest.yaml` : apply Kubernetes manifests with Go image references resolved to digests
- `ko create -f path/to/manifest.yaml` : create Kubernetes resources with resolved image references
- `ko resolve -f path/to/manifest.yaml` : print resolved Kubernetes manifests without applying them
- `ko run path/to/import_directory` : build and run a Go package on Kubernetes
- `ko delete -f path/to/manifest.yaml` : delete Kubernetes resources defined in a manifest
- `ko login registry.example.com -u username -p password` : log in to a container registry

## Categories
containers, development, files

## Links
- More information: https://ko.build/reference/ko/
- tldr page: https://tldr.inbrowser.app/pages/common/ko
