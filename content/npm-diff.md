# npm diff

Compare package versions from the `npm` registry and show differences. Similar to `git diff`.

## Usage
- `npm diff --diff package_name@version1 --diff package_name@version2` : compare two specific package versions
- `npm diff` : compare current local packages with latest published version
- `npm diff --diff package_name@version` : compare current local package with a specific version
- `npm diff --diff package_name` : compare a package in the current directory with the registry version
- `npm diff --diff-name-only --diff package_name@version1 --diff package_name@version2` : show only filenames that differ
- `npm diff path/to/file_or_directory --diff package_name@version1 --diff package_name@version2` : compare specific files or directories only
- `npm diff --diff-ignore-all-space --diff package_name@version1 --diff package_name@version2` : ignore whitespace when comparing

## Categories
version-control, packages, files

## Links
- More information: https://docs.npmjs.com/cli/npm-diff/
- tldr page: https://tldr.inbrowser.app/pages/common/npm-diff
