# frida-trace

함수 호출을 동적으로 추적하는 도구.

## Usage
- `frida-trace -i "와일드카드" 프로세스_이름` : 프로세스에서 패턴과 일치하는 모든 함수 추적
- `frida-trace -i "함수_이름" 프로세스_이름` : 프로세스에서 특정 함수 추적
- `frida-trace -I "모듈_이름" 프로세스_이름` : 특정 모듈의 모든 함수 추적
- `frida-trace -U -i "함수_이름" 프로세스_이름` : USB로 연결된 장치에서 함수 추적
- `frida-trace -f 경로/대상/실행파일 -i "함수_이름"` : 프로세스를 실행하면서 특정 함수 추적

## Links
- More information: https://frida.re/docs/frida-trace/
