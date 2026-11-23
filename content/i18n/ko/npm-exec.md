# npm exec

`npm` 패키지 바이너리 실행.

## Usage
- `npm x 명령어 argument1 argument2 ...` : 로컬 또는 원격 `npm` 패키지 명령어 실행
- `npm x --package 패키지 명령어` : 특정 패키지를 명시하여 실행 (동일한 이름의 명령이 여러개 있을 때 유용)
- `npm x --no-install 명령어 인자1 인자2 ...` : `node_modules/.bin`에 존재하는 경우에만 실행
- `npm x --quiet 명령어 인자1 인자2 ...` : `npm` 자체 출력 없이 명령 실행
- `npm x --help` : 도움말 표시

## Links
- More information: https://docs.npmjs.com/cli/npm-exec/
