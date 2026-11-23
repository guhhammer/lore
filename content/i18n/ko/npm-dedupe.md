# npm dedupe

`node_modules` 디렉토리에서 중복을 줄입니다.

## Usage
- `npm ddp` : `node_modules`의 패키지 중복 제거
- `npm ddp --lock` : 중복 제거 시 `package-lock.json` 또는 `npm-shrinkwrap.json`을 따르기
- `npm ddp --strict` : 엄격 모드로 중복 제거 실행
- `npm ddp --omit optional|peer` : 중복 제거 시 선택적/피어 의존성 건너뛰기
- `npm ddp --loglevel verbose` : 문제 해결을 위한 자세한 로깅 활성화
- `npm ddp 패키지_이름` : 특정 패키지에 대해 중복 제거 제한

## Links
- More information: https://docs.npmjs.com/cli/npm-dedupe/
