# helm

Helm is the package manager of Kubernetes. A chart is a package of manifests; a release is a chart installed in a cluster.

## Tasks
### Repositories
- `helm repo add NAME https://charts.example.org` : add a chart repository
- `helm repo update` : refresh the repository lists
- `helm repo list` : show the repositories added
- `helm search repo nginx` : look for a chart
- `helm search hub nginx` : look in Artifact Hub
### Look at a chart
- `helm show chart REPO/CHART` : the chart's description
- `helm show values REPO/CHART > values.yaml` : every setting with its default
- `helm template RELEASE REPO/CHART -f values.yaml` : print the manifests without installing
### Install and upgrade
- `helm install RELEASE REPO/CHART -n NAMESPACE --create-namespace` : install a chart
- `helm install RELEASE REPO/CHART -f values.yaml` : install with your settings
- `helm install RELEASE REPO/CHART --set key=value` : set one value
- `helm upgrade RELEASE REPO/CHART -f values.yaml` : change a release
- `helm upgrade --install RELEASE REPO/CHART -f values.yaml` : install or upgrade, whichever fits
- `helm install RELEASE REPO/CHART --dry-run --debug` : check what would happen
### Inspect releases
- `helm list -A` : releases in every namespace
- `helm status RELEASE` : state and notes of a release
- `helm get values RELEASE` : the values it uses
- `helm get manifest RELEASE` : the manifests it deployed
- `helm history RELEASE` : its revisions
### Roll back and remove
- `helm rollback RELEASE 1` : go back to revision 1
- `helm uninstall RELEASE` : remove a release (deletes its resources)
### Make your own chart
- `helm create NAME` : a chart skeleton
- `helm lint NAME` : check a chart for mistakes
- `helm package NAME` : build the .tgz
- `helm dependency update NAME` : fetch the chart's dependencies

## Notes
- Helm uses the same cluster and namespace settings as kubectl.
- `--set` values are hard to keep: put changes in a values file under version control.
- `helm upgrade` without `--reuse-values` goes back to the chart defaults for anything not given again.
- Chart repositories can also be OCI addresses: `helm install RELEASE oci://REGISTRY/CHART`.

## See also
- kubectl
- docker
- k9s

## Links
- Docs: https://helm.sh/docs/helm/
- Charts: https://artifacthub.io

## Categories
containers cloud
