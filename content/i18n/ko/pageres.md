# pageres

다양한 해상도로 웹사이트의 스크린샷을 캡처.

## Usage
- `pageres https://example.com/ https://example2.com/ 1366x768 1600x900` : 여러 URL의 다양한 해상도로 스크린샷 촬영
- `pageres [ https://example.com/ 1366x768 --no-crop ] [ https://example2.com/ 1024x768 ] -c` : 특정 URL에 대해 글로벌 옵션을 무시하고 특정 옵션 제공
- `pageres https://example.com/ 1024x768 --filename='<%= date %> - <%= url %>'` : 사용자 정의 파일명 템플릿 제공
- `pageres https://example.com/ 1366x768 --selector='.page-header'` : 페이지의 특정 요소 캡처
- `pageres https://example.com/ 1366x768 --hide='.page-header'` : 특정 요소 숨기기
- `pageres path/to/local_file.html 1366x768` : 로컬 파일의 스크린샷 캡처

## Links
- More information: https://github.com/sindresorhus/pageres-cli
