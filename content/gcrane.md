# gcrane

Container images managing tool. This tool implements a superset of the `crane` commands, with additional commands that are specific to Google Container Registry (`gcr.io`). Some subcommands such as `copy`, `gc`, `help`, `ls`, etc. have their own usage documentation.

## Usage
- `gcrane auth login registry -u user -p password` : log in to a registry
- `gcrane ls registry/project_id` : list tags, manifests, and sub-repositories
- `gcrane cp -r source_registry/project_id/repository target_registry/project_id/repository` : copy images from one registry to another
- `gcrane gc registry/project_id/repository` : print images that can be garbage collected
- `gcrane gc registry/project_id/repository | xargs -n 1 gcrane delete` : delete images that can be garbage collected
- `gcrane ls gcr.io/my-project-id` : list a specific registry with specific ID
- `gcrane cp -r gcr.io/my-project-id/repository eu.gcr.io/my-project-id/repository` : migrate all images from US registry to EU registry

## See also
- crane

## Categories
containers, media, packages

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/gcrane
