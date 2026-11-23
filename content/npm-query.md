# npm query

Print an array of dependency objects using CSS-like selectors.

## Usage
- `npm query ':root > *'` : print direct dependencies
- `npm query ':root > .prod|dev'` : print all direct production/development dependencies
- `npm query '#package'` : print dependencies with a specific name
- `npm query '#package@semantic_version'` : print dependencies with a specific name and within a semantic versioning range
- `npm query ':empty'` : print dependencies which have no dependencies
- `npm query ":attr(scripts, [postinstall])" | jq 'map(.name) | join("\n")' -r | xargs -I _ npm uninstall _` : find all dependencies with postinstall scripts and uninstall them
- `npm query ":type(git)" | jq 'map(.name)' | xargs -I _ npm why _` : find all Git dependencies and print which application requires them

## Categories
databases, packages, documents

## Links
- More information: https://docs.npmjs.com/cli/npm-query/
- tldr page: https://tldr.inbrowser.app/pages/common/npm-query
