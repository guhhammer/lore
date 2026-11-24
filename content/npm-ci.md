# npm ci

Clean install of `npm` project dependencies for automated environments. Installs packages based on `package-lock.json` or `npm-shrinkwrap.json`.

## Usage
- `npm ci` : install project dependencies from `package-lock.json` or `npm-shrinkwrap.json`
- `npm ci --omit dev|optional|peer` : install project dependencies but skip the specified dependency type
- `npm ci --ignore-scripts` : install project dependencies without running any pre-/post-scripts defined in `package.json`

## Categories
packages, text, development

## Links
- More information: https://docs.npmjs.com/cli/npm-ci/
- tldr page: https://tldr.inbrowser.app/pages/common/npm-ci
- Wikipedia: https://en.wikipedia.org/wiki/Npm
