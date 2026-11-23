# dmenu

동적 메뉴. 각 항목이 새 줄에 있는 텍스트 입력에서 메뉴 생성.

## Usage
- `ls | dmenu` : `ls` 명령어의 출력을 메뉴로 표시
- `echo -e "red\ngreen\nblue" | dmenu` : 새 줄(`\n`)로 구분된 사용자 정의 항목으로 메뉴 표시
- `echo -e "red\ngreen\nblue" | dmenu >color.txt` : 여러 항목 중 사용자가 선택한 항목을 파일에 저장
- `ls | dmenu -m 1` : 특정 모니터에서 dmenu 실행
- `ls | dmenu -b` : 화면 아래쪽에 dmenu 표시

## Links
- More information: https://manned.org/dmenu
