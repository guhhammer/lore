# helm upgrade

Upgrade a helm release to a new version of a chart or with new configuration.

## Usage
- `helm upgrade release_name repository_name/chart_name` : upgrade a release to the latest version of a chart
- `helm upgrade release_name path/to/source_directory` : upgrade a release from an unpacked chart directory
- `helm upgrade -i release_name repository_name/chart_name` : upgrade a release, or install it if it does not exist yet
- `helm upgrade release_name repository_name/chart_name --version chart_version` : upgrade a release to a specific chart version
- `helm upgrade release_name repository_name/chart_name -n namespace` : upgrade a release in a specific namespace
- `helm upgrade release_name repository_name/chart_name --set parameter1=value1,parameter2=value2` : upgrade a release with custom values
- `helm upgrade release_name repository_name/chart_name -f path/to/values.yaml` : upgrade a release passing a custom values file
- `helm upgrade release_name repository_name/chart_name --dry-run` : simulate an upgrade without actually doing it

## Categories
packages, containers, files

## Links
- More information: https://helm.sh/docs/helm/helm_upgrade/
- tldr page: https://tldr.inbrowser.app/pages/common/helm-upgrade
- Wikipedia: https://en.wikipedia.org/wiki/Helm_(package_manager)
