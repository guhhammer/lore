# dbclient

경량 Dropbear 보안 셸 클라이언트.

## Usage
- `dbclient 사용자@호스트` : 원격 호스트에 연결
- `dbclient 사용자@호스트 -p 2222` : p포트 2222로 원격 호스트에 연결
- `dbclient -i 경로/대상/키_파일 사용자@호스트` : dropbear 형식의 특정 identity 키를 사용하여 원격 호스트에 연결
- `dbclient 사용자@호스트 -t 명령 인수1 인수2 ...` : tty 할당으로 원격 명령과 상호작용하며 원격 호스트에서 명령 실행
- `dbclient -A 사용자@호스트` : agent 연결을 원격 호스트로 전달하며 연결

## Links
- More information: https://manned.org/dbclient
