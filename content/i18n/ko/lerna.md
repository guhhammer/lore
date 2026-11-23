# lerna

여러 패키지가 있는 JavaScript 프로젝트 관리.

## Usage
- `lerna init` : 프로젝트 파일 초기화(`lerna.json`, `package.json`, `.git` 등)
- `lerna bootstrap` : 각 패키지의 모든 외부 의존성을 설치하고 로컬 의존성을 심볼릭 링크로 연결
- `lerna run 스크립트` : `package.json`에 포함된 특정 스크립트를 모든 패키지에서 실행
- `lerna exec -- ls` : 모든 패키지에서 임의의 셸 명령 실행
- `lerna publish` : 마지막 릴리스 이후 변경된 모든 패키지 배포

## Links
- More information: https://lerna.js.org/docs/api-reference/commands
