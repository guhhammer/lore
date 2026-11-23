# ajson

JSON 객체에서 JSONPath를 실행합니다.

## Usage
- `ajson '$..json[?(@.path)]' 경로/대상/파일.json` : 파일에서 JSON을 읽고 지정된 JSONPath 표현식을 실행
- `cat 경로/대상/파일.json | ajson '$..json[?(@.path)]'` : `stdin`에서 JSON을 읽고 지정된 JSONPath 표현식을 실행
- `ajson 'avg($..price)' 'https://example.com/api/'` : URL에서 JSON을 읽고 지정된 JSONPath 표현식을 평가
- `echo '3' | ajson '2 * pi * $'` : 간단한 JSON을 읽고 값을 계산

## Links
- More information: https://github.com/spyzhov/ajson#console-application
