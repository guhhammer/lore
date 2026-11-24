# npm dist-tag

Manage distribution tags on packages.

## Usage
- `npm dist-tag ls package_name` : list all distribution tags for a package
- `npm dist-tag ls` : list all distribution tags for the current package
- `npm dist-tag add package_name@version tag` : add a distribution tag to a specific package version
- `npm dist-tag rm package_name tag` : remove a distribution tag from a package
- `npm dist-tag add package_name@version` : add a tag using the configured tag from npm config
- `npm dist-tag add package_name@version tag --otp one_time_password` : add a tag with two-factor authentication

## Categories
packages

## Links
- More information: https://docs.npmjs.com/cli/npm-dist-tag/
- tldr page: https://tldr.inbrowser.app/pages/common/npm-dist-tag
- Wikipedia: https://en.wikipedia.org/wiki/Npm
