# bun install

`package.json`에 정의된 JavaScript 의존성을 설치.

## Usage
- `bun i` : `package.json`에 나열된 모든 의존성 설치
- `bun i 패키지_이름@버전` : 단일 패키지 설치 (`bun add`의 별칭)
- `bun i -g 패키지_이름` : 패키지를 전역으로 설치
- `bun i -p` : 프로덕션 의존성만 설치 (`devDependencies` 제외)
- `bun i --frozen-lockfile` : `bun.lockb` 파일 기준으로 정확히 의존성 설치 (lockfile 고정)
- `bun i -f` : 캐시를 무시하고 모든 패키지를 강제로 다시 다운로드

## Links
- More information: https://bun.com/docs/pm/cli/install
