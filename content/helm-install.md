# helm install

Install a helm chart.

## Usage
- `helm install name repository_name/chart_name` : install a helm chart
- `helm install name path/to/source_directory` : install a helm chart from an unpacked chart directory
- `helm install package_name https://example.com/charts/packagename-1.2.3.tgz` : install a helm chart from a URL
- `helm install repository_name/chart_name -g` : install a helm chart and generate a name
- `helm install name repository_name/chart_name --dry-run` : simulate an install without actually doing it
- `helm install name repository_name/chart_name --set parameter1=value1,parameter2=value2` : install a helm chart with custom values
- `helm install name repository_name/chart_name -f path/to/values.yaml` : install a helm chart passing a custom values file

## Categories
packages, containers, files

## Links
- More information: https://helm.sh/docs/intro/using_helm/#helm-install-installing-a-package
- tldr page: https://tldr.inbrowser.app/pages/common/helm-install
