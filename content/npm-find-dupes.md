# npm find-dupes

Identify duplicate dependencies in `node_modules`.

## Usage
- `npm find-dupes` : list all duplicate packages within `node_modules`
- `npm find-dupes --include dev` : include `devDependencies` in duplicate detection
- `npm find-dupes package_name` : list all duplicate instances of a specific package in `node_modules`
- `npm find-dupes --omit optional` : exclude optional dependencies from duplicate detection
- `npm find-dupes --loglevel silent|error|warn|info|verbose` : set the logging level for output
- `npm find-dupes --json` : output duplicate information in JSON format
- `npm find-dupes --scope @scope1,@scope2` : limit duplicate search to specific scopes
- `npm find-dupes --omit-scope @scope1,@scope2` : exclude specific scopes from duplicate detection

## Categories
packages

## Links
- More information: https://docs.npmjs.com/cli/npm-find-dupes/
- tldr page: https://tldr.inbrowser.app/pages/common/npm-find-dupes
- Wikipedia: https://en.wikipedia.org/wiki/Npm
