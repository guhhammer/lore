# swagger-codegen

OpenAPI/swagger 정의에서 REST API에 대한 코드와 문서를 생성합니다.

## Usage
- `swagger-codegen generate -i swagger_파일 -l 언어` : openAPI/swagger 파일에서 문서와 코드 생성
- `swagger-codegen generate -i http://petstore.swagger.io/v2/swagger.json -l java --library retrofit2 -DuseRxJava2=true` : 라이브러리 retrofit2와 옵션 useRxJava2를 사용하여 Java 코드 생성
- `swagger-codegen langs` : 사용 가능한 언어 나열
- `swagger-codegen generate|config-help|meta|langs|version --help` : 특정 명령에 대한 도움말 표시

## Links
- More information: https://github.com/swagger-api/swagger-codegen
