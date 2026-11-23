# swagger-codegen

Generate code and documentation for your REST api from an OpenAPI/swagger definition.

## Usage
- `swagger-codegen generate -i swagger_file -l language` : generate documentation and code from an OpenAPI/swagger file
- `swagger-codegen generate -i http://petstore.swagger.io/v2/swagger.json -l java --library retrofit2 -DuseRxJava2=true` : generate Java code using the library retrofit2 and the option useRxJava2
- `swagger-codegen langs` : list available languages
- `swagger-codegen generate|config-help|meta|langs|version --help` : display help for a specific command

## Categories
documents, network

## Links
- More information: https://github.com/swagger-api/swagger-codegen
- tldr page: https://tldr.inbrowser.app/pages/common/swagger-codegen
