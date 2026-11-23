# jwt

Work with JSON Web Tokens (JWTs). Encryption algorithms available are HS256, HS384, HS512, RS256, RS384, RS512, ES256, ES384.

## Usage
- `jwt decode jwt_string` : decode a JWT
- `jwt decode -j jwt_string` : decode a JWT as a JSON string
- `jwt encode -A HS256 -S 1234567890 'json_string'` : encode a JSON string to a JWT
- `jwt encode -A HS256 -S 1234567890 -P key=value` : encode key pair payload to JWT

## Categories
text

## Links
- More information: https://github.com/mike-engel/jwt-cli
- tldr page: https://tldr.inbrowser.app/pages/common/jwt
