# bun exec

Bun 런타임을 사용해 셸 명령어나 스크립트 파일을 실행. 참고: 셸에서 실행할 때는, 따옴표를 이스케이프(escape)해야 합니다.

## Usage
- `bun exec "echo hello"` : 단일 명령어 실행
- `bun exec "ls -la"` : 플래그를 가진 명령어를 실행
- `bun exec "echo \"hello friends\""` : 따옴표를 포함한 명령어를 실행
- `bun exec "mkdir test && cd test"` : 조합된 셸 명령어를 실행
- `bun exec 경로/대상/스크립트` : 스크립트 파일을 실행

## Links
- More information: https://bun.com/docs/runtime/shell
