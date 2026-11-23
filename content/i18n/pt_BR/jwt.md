# jwt

Uma ferramenta de linha de comando (command-line tool) para trabalhar com JSON Web Tokens (JWTs). Algoritmos de encriptação disponíveis são HS256, HS384, HS512, RS256, RS384, RS512, ES256, ES384.

## Usage
- `jwt decode jwt_string` : decodifica um JWT
- `jwt decode -j jwt_string` : decodifica um JWT em uma JSON string
- `jwt encode -A HS256 -S 1234567890 'json_string'` : codifica uma JSON string em um JWT
- `jwt encode -A HS256 -S 1234567890 -P chave=valor` : codifica dados (payload) de um par de chaves (key pair) em um JWT

## Links
- More information: https://github.com/mike-engel/jwt-cli
