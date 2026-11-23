# emaint

Portage 유지 관리 작업을 수행.

## Usage
- `sudo emaint sync -a` : 자동 동기화가 설정된 저장소 동기화 (대부분의 저장소에서 기본값)
- `sudo emaint sync -r 저장소` : 지정한 저장소 동기화
- `sudo emaint sync -A` : 모든 저장소 동기화
- `sudo emaint cleanresume -f` : portage 재개 목록 정리
- `sudo emaint logs -C` : portage 로그 정리

## Links
- More information: https://wiki.gentoo.org/wiki/Portage#emaint
