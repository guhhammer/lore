# npm prune

Remove extraneous packages from `node_modules`. Note: Extraneous packages are those present in the node_modules folder that are not listed as any package's dependency list.

## Usage
- `npm prune` : remove all extraneous packages not listed in dependencies
- `npm prune --production` : remove extraneous packages and devDependencies (useful for production builds)
- `npm prune --dry-run` : simulate pruning, showing what would be removed without making any changes
- `npm prune --json` : output the changes as JSON
- `npm prune package_name` : remove specific packages by name

## Categories
packages, files

## Links
- More information: https://docs.npmjs.com/cli/npm-prune/
- tldr page: https://tldr.inbrowser.app/pages/common/npm-prune
- Wikipedia: https://en.wikipedia.org/wiki/Npm
