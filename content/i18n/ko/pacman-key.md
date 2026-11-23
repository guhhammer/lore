# pacman-key

GnuPG를 사용하여 pacman의 키링을 관리하는 래퍼 스크립트.

## Usage
- `sudo pacman-key --init` : `pacman` 키링 초기화
- `sudo pacman-key --populate` : 기본 Arch Linux 키 추가
- `pacman-key -l` : 공개 키링에서 키 나열
- `sudo pacman-key -a 경로/대상/키파일.gpg` : 지정된 키 추가
- `sudo pacman-key -r "uid|name|email"` : 키 서버에서 키 수신
- `pacman-key -f "uid|name|email"` : 특정 키의 지문 출력
- `sudo pacman-key --lsign-key "uid|name|email"` : 가져온 키를 로컬에서 서명
- `sudo pacman-key -d "uid|name|email"` : 특정 키 제거

## Links
- More information: https://manned.org/pacman-key
