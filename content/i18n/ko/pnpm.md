# pnpm

빠르고, 디스크 공간 효율적인 Node.js용 패키지 관리자. Node.js 프로젝트 및 해당 모듈 의존성 관리.

## Usage
- `pnpm init` : `package.json` 파일 생성
- `pnpm i` : `package.json`에 의존성으로 나열된 모든 패키지를 다운로드
- `pnpm add 모듈_이름@버전` : 특정 버전의 패키지를 다운로드하여 `package.json`의 의존성 목록에 추가
- `pnpm add 모듈_이름 -D` : 패키지를 다운로드하고 `package.json`의 개발(Dev) 의존성 목록에 추가
- `pnpm add 모듈_이름 -g` : 패키지를 다운로드하고 전역적으로(globally) 설치
- `pnpm rm 모듈_이름` : 패키지를 제거하고 `package.json`의 종속성 목록에서 제거
- `pnpm ls` : 로컬에 설치된 모듈의 트리 출력
- `pnpm ls -g --depth 0` : 최상위 전역적으로(globally) 설치된 모듈 나열

## Links
- More information: https://pnpm.io/pnpm-cli
