# cs fetch

Fetch fetches the JARs of dependencies.

## Usage
- `cs fetch group_id:artifact_id:artifact_version` : fetch a specific version of a `.jar`
- `CP="$(cs fetch --classpath org.scalameta::scalafmt-cli:latest.release)"` : fetch a package and evaluate the classpath corresponding to the selected package in an env var
- `cs fetch --sources group_id:artifact_id:artifact_version` : fetch a source of a specific `.jar`
- `cs fetch --javadoc group_id:artifact_id:artifact_version` : fetch the javadoc jars
- `cs fetch --default=true --sources --javadoc group_id:artifact_id:artifact_version` : fetch dependency with javadoc jars and source jars
- `cs fetch --dependency-file path/to/file1 --dependency-file path/to/file2 ...` : fetch jars coming from dependency files

## Categories
packages, files

## Links
- More information: https://get-coursier.io/docs/cli-fetch
- tldr page: https://tldr.inbrowser.app/pages/common/cs-fetch
