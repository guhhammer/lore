# ddev

PHP 환경을 위한 컨테이너 기반 로컬 개발도구.

## Usage
- `ddev start` : 프로젝트 시작
- `ddev config` : 프로젝트 유형 및 문서 루트를 구성
- `ddev logs -f` : 로그를 계속해서 출력(follow)
- `ddev composer` : 컨테이너 내에서 composer를 실행
- `ddev nvm install 버전` : 특정 Node.js 버전 설치
- `ddev export-db -f /tmp/db.sql.gz` : 데이터베이스 내보내기
- `ddev exec echo 1` : 컨테이너 내에서 특정 명령을 실행

## Links
- More information: https://docs.ddev.com/en/stable/users/usage/cli/
