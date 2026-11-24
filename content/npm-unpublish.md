# npm unpublish

Remove a package from the npm registry.

## Usage
- `npm unpublish package_name@version` : unpublish a specific package version
- `npm unpublish package_name -f` : unpublish the entire package
- `npm unpublish @scope/package_name` : unpublish a package that is scoped
- `npm unpublish package_name --timeout time_in_milliseconds` : specify a timeout period before unpublishing
- `npm unpublish package_name --dry-run` : simulate what would be unpublished without actually doing it

## Categories
packages

## Links
- More information: https://docs.npmjs.com/cli/npm-unpublish/
- tldr page: https://tldr.inbrowser.app/pages/common/npm-unpublish
- Wikipedia: https://en.wikipedia.org/wiki/Npm
