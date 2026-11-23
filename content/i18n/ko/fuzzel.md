# fuzzel

Wayland 환경에서 동작하는 애플리케이션 실행기 및 퍼지 검색 도구(`rofi`와 `dmenu`에서 영감을 받아 만들어짐).

## Usage
- `fuzzel` : 애플리케이션 실행
- `fuzzel -d` : dmenu 모드로 `fuzzel` 실행
- `ls | fuzzel -d` : `ls` 명령의 출력을 메뉴로 표시
- `echo -e "red\ngreen\nblue" | fuzzel -d` : 줄바꿈으로(`\n`) 구분된 사용자 지정 항목을 메뉴로 표시
- `echo -e "red\ngreen\nblue" | fuzzel -d >color.txt` : 여러 항목 중 사용자가 선택한 값을 파일에 저장
- `rm -v $HOME/.cache/fuzzel` : 애플리케이션 사용 횟수 캐시 초기화 (기본 캐시 디렉터리: `$XDG_CACHE_HOME/fuzzel`)
- `fuzzel -o "DP-1"` : 지정한 모니터에서 `fuzzel` 실행 (`wlr-randr` 또는 `swaymsg --type get_outputs` 참고)
- `fuzzel -d -l 0 --placeholder "Type your search" | sed 's/^/\"/g;s/$/\"/g' | xargs firefox --search` : `fuzzel`를 사용하여 온라인 검색 수행

## Links
- More information: https://codeberg.org/dnkl/fuzzel
