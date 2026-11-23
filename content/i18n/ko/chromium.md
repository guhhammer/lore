# chromium

구글에서 주도하는 오픈소스 웹 브라우저. 참고: 원하는 웹 브라우저로 `chromium` 명령어를 대체할 수 있습니다. 예를 들어 `brave`, `google-chrome`, `opera`, `vivaldi` 등을 사용할 수 있습니다.

## Usage
- `chromium https://example.com|경로/대상/파일.html` : 특정 URL 또는 파일 열기
- `chromium --incognito example.com` : 익명으로 열기
- `chromium --new-window example.com` : 새 창으로 열기
- `chromium --app=https://example.com` : 앱 모드로 열기 (툴바, URL 바, 버튼 등 제외)
- `chromium --proxy-server="socks5://호스트명:포트" example.com` : 프록시 서버 사용
- `chromium --user-data-dir=경로/대상/디렉토리` : 사용자 데이터 디렉토리 지정
- `chromium --user-data-dir=경로/대상/디렉토리 --disable-web-security` : CORS 검증 없이 열기 (API 테스트 유용)
- `chromium --auto-open-devtools-for-tabs` : 각 탭에 대해 DevTools 창 열기

## Links
- More information: https://www.chromium.org/developers/how-tos/run-chromium-with-flags/
