# bw

Bitwarden 보관함에 접속과 관리를 위한 CLI.

## Usage
- `bw login` : bitwarden 사용자 계정 로그인
- `bw logout` : 사용자 계정 로그아웃
- `bw list items --search github` : bitwarden 보관함으로부터 아이템 검색과 출력
- `bw get item github` : bitwarden 보관함으로부터 특정 아이템 출력
- `echo -n '{"name":"My Folder1"}' | base64 | bw create folder` : bitwarden 보관함에 폴더 생성

## Links
- More information: https://bitwarden.com/help/cli/
