# npm

JavaScript and Node.js package manager. Manage Node.js projects and their module dependencies. Some subcommands such as `install`, `run`, etc. have their own usage documentation.

## Usage
- `npm init -y` : create a `package.json` file with default values (omit `--yes` to do it interactively)
- `npm i` : download all the packages listed as dependencies in `package.json`
- `npm i package_name@version` : download a specific version of a package and add it to the list of dependencies in `package.json`
- `npm i package_name -D` : download the latest version of a package and add it to the list of dev dependencies in `package.json`
- `npm i package_name -g` : download the latest version of a package and install it globally (set the install location with `npm config set prefix`)
- `npm r package_name` : uninstall a package and remove it from the list of dependencies in `package.json`
- `npm ls` : list all locally installed dependencies
- `npm ls -g --depth 0` : list all top-level globally installed packages

## Categories
packages, development

## Links
- More information: https://docs.npmjs.com/cli/npm/
- tldr page: https://tldr.inbrowser.app/pages/common/npm
- Wikipedia: https://en.wikipedia.org/wiki/Npm
