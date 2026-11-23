# frida

개발자, 리버스 엔지니어, 보안 연구자를 위한 동적 계측 툴킷.

## Usage
- `frida 프로세스_이름` : 실행 중인 프로세스에 연결된 대화형 쉘(REPL) 시작
- `frida -U 프로세스_이름` : USB를 통해 프로세스에 연결된 대화형 쉘 시작
- `frida -p pid` : PID를 사용하여 실행 중인 프로세스에 연결
- `frida -l 경로/대상/스크립트.js 프로세스_이름` : 프로세스에 JavaScript 스크립트 로드
- `frida -c 스크립트_이름 프로세스_이름` : frida Codeshare <https://codeshare.frida.re/>에서 스크립트를 로드하여 프로세스에 적용

## Links
- More information: https://frida.re/docs/frida-cli/
