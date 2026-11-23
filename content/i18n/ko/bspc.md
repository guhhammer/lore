# bspc

`bspwm`을 구성하고 제어하여 노드, 데스크톱, 모니터 등을 관리합니다.

## Usage
- `bspc monitor -d 데스크톱_이름1 데스크톱_이름2` : 두 개의 가상 데스크톱 정의
- `bspc desktop -f 번호` : 지정한 데스크톱으로 포커스 이동
- `bspc node -c` : 선택한 노드에 있는 창 닫기
- `bspc node -d 번호` : 선택한 노드를 지정한 데스크톱으로 보내기
- `bspc node -t ~fullscreen` : 선택한 노드의 전체 화면 모드 전환
- `bspc config 설정_이름 값` : 특정 설정의 값 설정

## Links
- More information: https://github.com/baskerville/bspwm/blob/master/doc/bspwm.1.asciidoc
