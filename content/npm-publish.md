# npm publish

Publish a package to the npm registry.

## Usage
- `npm publish` : publish the current package to the default npm registry
- `npm publish path/to/package_directory` : publish a package from a specific directory
- `npm publish --access public` : publish a scoped package with public access
- `npm publish --access restricted` : publish a scoped package with restricted (private) access
- `npm publish --registry https://registry.npmjs.org/` : publish a package to a custom registry
- `npm publish --dry-run` : simulate what would be published without uploading
- `npm publish --tag beta` : publish a package with a specific distribution tag (e.g., beta)
- `npm publish --otp one_time_password` : publish with a one-time password for 2FA-enabled accounts

## Categories
packages, network

## Links
- More information: https://docs.npmjs.com/cli/publish/
- tldr page: https://tldr.inbrowser.app/pages/common/npm-publish
