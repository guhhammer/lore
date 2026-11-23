# octo

Octopus Deploy 명령줄 도구.

## Usage
- `octo pack --id 패키지` : 패키지 생성
- `octo push --package 패키지` : 패키지를 Octopus 서버의 저장소에 푸시
- `octo create-release --project 프로젝트_이름 --packageversion 버전` : 릴리스 생성
- `octo deploy-release --project 프로젝트_이름 --packageversion 버전 --deployto 환경_이름 --tenant 배포_대상` : 릴리스 배포

## Links
- More information: https://octopus.com/docs/cli/octopus-cli
