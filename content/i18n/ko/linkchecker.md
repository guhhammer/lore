# linkchecker

HTML 문서 및 웹사이트의 깨진 링크를 확인하는 명령줄 클라이언트.

## Usage
- `linkchecker https://example.com/` : <https://example.com/>에서 깨진 링크 찾기
- `linkchecker --check-extern https://example.com/` : 외부 도메인을 가리키는 URL도 확인
- `linkchecker --ignore-url 정규_표현식 https://example.com/` : 특정 정규 표현식과 일치하는 URL 무시
- `linkchecker --file-output csv/경로/대상/파일 https://example.com/` : 결과를 CSV 파일로 출력

## Links
- More information: https://linkchecker.github.io/linkchecker/man/linkchecker.html
