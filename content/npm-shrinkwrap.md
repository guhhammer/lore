# npm shrinkwrap

Lock down the dependencies of a package, creating an `npm-shrinkwrap.json` file. Similar to `package-lock.json` but intended for published packages.

## Usage
- `npm shrinkwrap` : generate an `npm-shrinkwrap.json` file from the current `package-lock.json`
- `npm shrinkwrap --production` : run in production mode (excludes devDependencies)
- `npm shrinkwrap --force` : force recreate the shrinkwrap file even if it already exists

## Categories
packages, text, files

## Links
- More information: https://docs.npmjs.com/cli/v11/npm-shrinkwrap
- tldr page: https://tldr.inbrowser.app/pages/common/npm-shrinkwrap
- Wikipedia: https://en.wikipedia.org/wiki/Npm
