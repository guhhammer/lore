# valkey-cli

Valkey 서버에 연결.

## Usage
- `valkey-cli` : 로컬 서버에 연결
- `valkey-cli -h 호스트` : 기본 포트 (6379)를 사용하여 원격 서버에 연결
- `valkey-cli -h 호스트 -p 포트` : 포트 번호를 지정하여 원격 서버에 연결
- `valkey-cli -u uri` : URI를 지정하여 원격 서버에 연결
- `valkey-cli -a 비밀번호` : 비밀번호 지정
- `valkey-cli valkey_명령어` : valkey 명령 실행
- `valkey-cli -c` : 로컬 클러스터에 연결

## Links
- More information: https://valkey.io/topics/cli/
