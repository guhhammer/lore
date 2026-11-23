# deployment

Automatically pull Git repositories and run their `deploy` scripts. Reads deployments as `path,repository_url` rows from a CSV file.

## Usage
- `deployment` : continuously deploy repositories listed in `/etc/deployments.csv`, using `/var/deployment` as the base directory for relative paths
- `deployment -d path/to/deployments.csv -H path/to/directory` : use a specific deployment list and base directory
- `deployment -c` : clone repositories that are missing locally, then exit
- `deployment -n` : simulate a deployment pass by printing commands without executing them
- `deployment --delay 60` : set the delay in seconds after each deployment
- `deployment -D` : move build output from each repository's `dist` directory under the base directory's `dist` directory

## Categories
files, text, version-control

## Links
- More information: https://github.com/chrissxMedia/deployment
- tldr page: https://tldr.inbrowser.app/pages/common/deployment
