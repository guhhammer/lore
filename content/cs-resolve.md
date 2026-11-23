# cs resolve

Resolve lists the transitive dependencies of other dependencies.

## Usage
- `cs resolve group_id1:artifact_id1:artifact_version1 group_id2:artifact_id2:artifact_version2` : resolve lists of transitive dependencies of two dependencies
- `cs resolve --tree group_id:artifact_id:artifact_version` : resolve lists of transitive dependencies of a package by the dependency tree
- `cs resolve --reverse-tree group_id:artifact_id:artifact_version` : resolve dependency tree in a reverse order (from a dependency to its dependencies)
- `cs resolve group_id:artifact_id:artifact_version --what-depends-on searched_group_id:searched_artifact_id` : print all the libraries that depends on a specific library
- `cs resolve group_id:artifact_id:artifact_version --what-depends-on searched_group_id:searched_artifact_idsearched_artifact_version` : print all the libraries that depends on a specific library version
- `cs resolve --conflicts group_id1:artifact_id1:artifact_version1 group_id2:artifact_id2:artifact_version2 ...` : print eventual conflicts between a set of packages

## Categories
packages, documents

## Links
- More information: https://get-coursier.io/docs/cli-resolve
- tldr page: https://tldr.inbrowser.app/pages/common/cs-resolve
