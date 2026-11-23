# oauth2c

Interact with OAuth 2.0 authorization servers.

## Usage
- `oauth2c issuer_url --client-id client_id --client-secret client_secret` : fetch an access token using client credentials
- `oauth2c issuer_url --client-id client_id --response-types code` : fetch a token using authorization code flow
- `oauth2c issuer_url --client-id client_id --pkce` : fetch a token using authorization code with PKCE
- `oauth2c issuer_url --client-id client_id --username username --password password` : fetch a token using password credentials
- `oauth2c issuer_url --client-id client_id --refresh-token refresh_token` : refresh an existing access token
- `oauth2c issuer_url --client-id client_id --scopes scope1,scope2` : fetch a token with specific scopes
- `oauth2c issuer_url --client-id client_id --grant-type device_code` : use device authorization flow
- `oauth2c issuer_url --client-id client_id -s --no-browser` : run in silent mode without browser

## Categories
users

## Links
- More information: https://github.com/cloudentity/oauth2c#usage
- tldr page: https://tldr.inbrowser.app/pages/common/oauth2c
