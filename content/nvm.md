# nvm

Install, uninstall, or switch between Node.js versions. Supports version numbers like "12.8" or "v16.13.1", and labels like "node", "system", etc.

## Usage
- `nvm install|uninstall node_version` : install/uninstall a specific version of Node.js
- `nvm use node_version` : use a specific version of Node.js in the current shell
- `nvm alias default node_version` : set the default Node.js version
- `nvm ls-remote --lts` : list remote versions available for install, only show LTS (long-term support) versions
- `nvm ls` : list installed versions
- `nvm run node_version` : launch the REPL of a specific version of Node.js
- `nvm exec node_version node path/to/script.js` : execute a script in a specific version of Node.js
- `nvm install-latest-npm` : upgrade to the latest working npm version on the current Node.js version

## See also
- asdf

## Categories
packages

## Links
- More information: https://github.com/nvm-sh/nvm#usage
- tldr page: https://tldr.inbrowser.app/pages/common/nvm
