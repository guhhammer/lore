# bun test

Bun의 내장 테스트 러너를 사용하여 테스트를 실행. 빠르고 Jest와 호환되는 테스트 러너로, `*.test.ts` (및 유사한 파일)을 자동으로 탐색.

## Usage
- `bun test` : 프로젝트에서 발견된 모든 테스트 파일 실행
- `bun test 경로/대상/파일.test.ts` : 특정 테스트 파일 또는 디렉토리 실행
- `bun test --watch` : 파일 변경 시 자동 재시작되는 "watch" 모드로 실행
- `bun test --coverage` : 테스트 실행 및 코드 커버리지 리포트 생성

## Links
- More information: https://bun.com/docs/test
