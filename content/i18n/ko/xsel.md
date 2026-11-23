# xsel

X11 선택 및 클립보드 조작 도구.

## Usage
- `echo 123 | xsel -ib` : 명령의 출력을 클립보드 입력으로 사용 (`<Ctrl c>`와 동일)
- `cat 경로/대상/파일 | xsel -ib` : 파일의 내용을 클립보드 입력으로 사용
- `xsel -ob` : 클립보드의 내용을 터미널에 출력 (`<Ctrl v>`와 동일)
- `xsel -ob >경로/대상/파일` : 클립보드의 내용을 파일에 출력
- `xsel -cb` : 클립보드 내용 지우기
- `xsel -op` : X11 기본 선택의 내용을 터미널에 출력 (`<MiddleClick>` 동일)

## Links
- More information: https://manned.org/xsel
