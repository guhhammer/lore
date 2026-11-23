# screenkey

키 입력을 화면에 표시하는 스크린캐스트 도구.

## Usage
- `screenkey` : 현재 눌린 키를 화면에 표시
- `screenkey -M` : 현재 눌린 키와 마우스 버튼을 화면에 표시
- `screenkey --show-settings` : screenkey의 설정 메뉴 실행
- `screenkey -p top|center|bottom|fixed` : 특정 위치에 screenkey 실행
- `screenkey --mods-mode normal|emacs|mac|win|tux` : 화면에 표시되는 키 수정자의 형식 변경
- `screenkey --bg-color "#a1b2c3" -f Hack --font-color yellow --opacity 0.8` : screenkey의 외관 변경
- `screenkey -p fixed -g $(slop --nodecorations --format '%g')` : 화면에서 창을 드래그하여 screenkey 표시 위치 선택

## Links
- More information: https://www.thregr.org/wavexx/software/screenkey/
