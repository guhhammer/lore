# npm dedupe

Reduce duplication in the `node_modules` directory.

## Usage
- `npm ddp` : deduplicate packages in `node_modules`
- `npm ddp --lock` : follow `package-lock.json` or `npm-shrinkwrap.json` during deduplication
- `npm ddp --strict` : run deduplication in strict mode
- `npm ddp --omit optional|peer` : skip optional/peer dependencies during deduplication
- `npm ddp --loglevel verbose` : enable detailed logging for troubleshooting
- `npm ddp package_name` : limit deduplication to a specific package

## Categories
packages, files

## Links
- More information: https://docs.npmjs.com/cli/npm-dedupe/
- tldr page: https://tldr.inbrowser.app/pages/common/npm-dedupe
- Wikipedia: https://en.wikipedia.org/wiki/Npm
