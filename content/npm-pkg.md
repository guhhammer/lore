# npm pkg

Show or modify `package.json` properties.

## Usage
- `npm pkg get name` : get the value of a specific property
- `npm pkg get name|version|...` : get multiple properties at once
- `npm pkg get name version --ws` : get multiple values across all workspaces
- `npm pkg get contributors[0].email` : get a nested or array property value
- `npm pkg set property=value` : set a property to a specific value
- `npm pkg set property1=value1 property2=value2` : set multiple properties at once
- `npm pkg delete scripts.build` : delete a property from `package.json`
- `npm pkg fix` : auto-fix common errors in `package.json`

## Categories
packages, text

## Links
- More information: https://docs.npmjs.com/cli/npm-pkg/
- tldr page: https://tldr.inbrowser.app/pages/common/npm-pkg
- Wikipedia: https://en.wikipedia.org/wiki/Npm
