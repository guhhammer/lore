# xwininfo

창에 대한 정보를 표시.

## Usage
- `xwininfo` : 커서를 표시하여 창을 선택하고 해당 속성(아이디, 이름, 크기, 위치 등) 표시
- `xwininfo -tree -root` : 모든 창의 트리 구조 표시
- `xwininfo -id 아이디` : 특정 ID를 가진 창의 속성 표시
- `xwininfo -name 이름` : 특정 이름을 가진 창의 속성 표시
- `xwininfo -tree -root | grep 키워드 | head -1 | perl -ne 'print $1 if /(0x[\da-f]+)/ig;'` : 이름으로 검색하여 창의 ID 표시

## Links
- More information: https://www.x.org/releases/current/doc/man/man1/xwininfo.1.xhtml
