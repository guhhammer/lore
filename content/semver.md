# semver

Semantic version string parser.

## Usage
- `semver 1.2` : check if a version string respects the semantic versioning format (prints an empty string if it does not match)
- `semver --coerce 1.2` : convert a version string to the semantic versioning format
- `semver 1.2.3 --range "^1.0"` : test if `1.2.3` matches the `^1.0` range (prints an empty string if it does not match)
- `semver 1.2.3 --range ">=1.0" "<2.0"` : test with multiple ranges
- `semver 1.2.3 2.0.0 --range "^1.0"` : test multiple version strings and return only the ones that match

## Categories
text, development

## Links
- More information: https://github.com/npm/node-semver
- tldr page: https://tldr.inbrowser.app/pages/common/semver
