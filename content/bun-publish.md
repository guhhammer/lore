# bun publish

Publish a package to the npm registry.

## Usage
- `bun publish` : publish the current package to the npm registry
- `bun publish path/to/package_directory` : publish a package from a specific directory
- `bun publish --access public|restricted` : publish a scoped package with specific access level
- `bun publish --registry registry` : publish a package to a custom registry
- `bun publish --dry-run` : simulate what would be published without uploading
- `bun publish --tag tag_name` : publish a package with a specific distribution tag
- `bun publish --otp one_time_password` : publish with a one-time password for 2FA-enabled accounts
- `bun publish --auth-type web|legacy` : publish using a specific authentication type

## Categories
packages

## Links
- More information: https://bun.com/docs/pm/cli/publish
- tldr page: https://tldr.inbrowser.app/pages/common/bun-publish
