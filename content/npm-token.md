# npm token

Manage and generate authentication tokens for the npm registry.

## Usage
- `npm token create` : create a new authentication token
- `npm token list` : list all tokens associated with an account
- `npm token revoke token_id` : delete a specific token using its token ID
- `npm token create --read-only` : create a token with read-only access
- `npm token create --publish` : create a token with publish access
- `npm login` : automatically configure an npm token in your global `.npmrc` file when you log in

## Links
- More information: https://docs.npmjs.com/cli/npm-token/
- tldr page: https://tldr.inbrowser.app/pages/common/npm-token
