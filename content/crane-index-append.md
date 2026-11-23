# crane index append

Append manifest to a remote index. This sub-command pushes an index based on an (optional) base index, with appended manifests. The platform for appended manifests is inferred from the configuration file or omitted if that is infeasible.

## Usage
- `crane index append` : append manifest to a remote index
- `crane index append -m manifest_name1 manifest_name2 ...` : append specific manifests to the base index
- `crane index append -t tag_name` : apply a tag to the resulting image
- `crane index append --docker-empty-base` : use Docker media types instead of OCI for an empty base index
- `crane index append --flatten` : append each of its children rather than the index itself (defaults true)
- `crane index append -h` : display help

## Categories
containers, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/crane-index-append
