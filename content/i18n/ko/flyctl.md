# flyctl

flyctl.io용 명령줄 도구.

## Usage
- `flyctl auth login` : fly 계정에 로그인
- `flyctl launch --dockerfile 경로/대상/도커파일` : 특정 Dockerfile에서 애플리케이션을 시작 (기본 경로는 현재 작업 디렉터리)
- `flyctl open` : 기본 웹 브라우저에서 현재 배포된 애플리케이션을 열기
- `flyctl deploy --dockerfile 경로/대상/도커파일` : 특정 Dockerfile에서 Fly 애플리케이션을 배포
- `flyctl dashboard` : 웹 브라우저에서 현재 애플리케이션에 대한 Fly Web UI를 열기
- `flyctl apps list` : 로그인한 Fly 계정의 모든 애플리케이션을 나열
- `flyctl status --app 앱_이름` : 실행 중인 특정 애플리케이션의 상태 보기
- `flyctl version` : 버전 표시

## Links
- More information: https://github.com/superfly/flyctl
