# figlet

사용자 입력에서 ASCII 배너를 생성.

## Usage
- `figlet 입력_문자열` : 텍스트를 직접 입력하여 생성
- `figlet -f 경로/대상/폰트_파일.flf 입력_문자열` : 사용자 정의 폰트(font) 파일을 사용
- `figlet -f 폰트_파일이름 입력_문자열` : 기본 글꼴 디렉토리의 폰트(font)를 사용 (확장자는 생략 가능)
- `명령어 | figlet` : FIGlet을 통한 파이프 명령 출력
- `showfigfonts 표시할 선택적 문자열` : 사용 가능한 FIGlet 글꼴 표시
- `figlet -t -c 입력_문자열` : 터미널(terminal)의 전체 너비를 사용하고 입력 텍스트를 중앙(center)에 배치
- `figlet -W 입력_문자열` : 겹치지 않도록 모든 문자를 전체 너비(Width)로 표시

## Links
- More information: https://www.figlet.org/figlet-man.html
