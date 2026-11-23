# atuin

검색 가능한 데이터베이스에 쉘 기록을 저장하세요. 선택적으로 컴퓨터 간에 암호화된 기록을 동기화하세요.

## Usage
- `eval "$(atuin init bash|zsh|fish)"` : 쉘에 atuin 설치
- `atuin import auto` : 쉘 기본 기록 파일에서 기록 가져오기
- `atuin search 명령어` : 특정 명령에 대한 쉘 기록 검색
- `atuin register -u 사용자명 -e 이메일 -p 비밀번호` : 기본 동기화 서버에 계정 등록
- `atuin login -u 사용자명 -p 비밀번호` : 기본 동기화 서버에 로그인
- `atuin sync` : 동기화 서버와의 동기화 기록

## Links
- More information: https://docs.atuin.sh/
