# web-ext

웹 확장 프로그램 개발을 관리하는 명령줄 도구.

## Usage
- `web-ext run` : 현재 디렉터리에 있는 웹 확장 프로그램을 Firefox에서 실행
- `web-ext run --source-dir 경로/대상/폴더` : 특정 디렉터리에서 웹 확장 프로그램을 Firefox에서 실행
- `web-ext run --verbose` : 자세한 실행 출력 표시
- `web-ext run --target firefox-android` : firefox Android에서 웹 확장 프로그램 실행
- `web-ext lint` : 매니페스트 및 소스 파일의 오류 검사
- `web-ext build` : 확장 프로그램 빌드 및 패키징
- `web-ext build --verbose` : 자세한 빌드 출력 표시
- `web-ext sign --channel unlisted --api-key api_키 --api-secret api_비밀` : 자체 호스팅을 위한 패키지 서명

## Links
- More information: https://github.com/mozilla/web-ext
