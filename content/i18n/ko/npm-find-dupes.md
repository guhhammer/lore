# npm find-dupes

`node_modules`에서 중복된 의존성을 식별.

## Usage
- `npm find-dupes` : `node_modules` 내 모든 중복 패키지 나열
- `npm find-dupes --include dev` : 중복 감지에 `devDependencies` 포함
- `npm find-dupes 패키지_이름` : `node_modules`에서 특정 패키지의 모든 중복 인스턴스 나열
- `npm find-dupes --omit optional` : 중복 감지에서 선택적 의존성 제외
- `npm find-dupes --loglevel silent|error|warn|info|verbose` : 출력의 로그 레벨 설정
- `npm find-dupes --json` : 중복 정보를 JSON 형식으로 출력
- `npm find-dupes --scope @스코프1,@스코프2` : 중복 검색을 특정 스코프로 제한
- `npm find-dupes --omit-scope @스코프1,@스코프2` : 특정 스코프를 중복 감지에서 제외

## Links
- More information: https://docs.npmjs.com/cli/npm-find-dupes/
