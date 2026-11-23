# eselect kernel

`/usr/src/linux` 심볼릭 링크를 관리하기 위한 `eselect` 모듈.

## Usage
- `eselect kernel list` : 사용 가능한 커널 심볼릭 링크 대상과 해당 번호 나열
- `eselect kernel set 이름|번호` : `list` 명령어의 이름이나 번호로 `/usr/src/linux` 심볼릭 링크 설정
- `eselect kernel show` : 현재 커널 심볼릭 링크가 가리키는 대상을 표시
- `eselect kernel update` : 현재 실행 중인 커널로 커널 심볼릭 링크 설정

## Links
- More information: https://wiki.gentoo.org/wiki/Eselect#Kernel
