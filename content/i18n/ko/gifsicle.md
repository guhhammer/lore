# gifsicle

GIF 파일에 대한 정보를 생성, 편집, 조작 및 가져옴.

## Usage
- `gifsicle 경로/대상/입력파일.gif -O3 -o 경로/대상/출력파일.gif` : GIF를 새로운 파일로 최적화
- `gifsicle -b 경로/대상/입력파일.gif -U` : batch 모드를 사용하고 (주어진 각 파일을 제자리에서 수정) GIF 최적화를 취소
- `gifsicle 경로/대상/입력파일.gif '#0' >경로/대상/첫번째_프레임.gif` : GIF에서 프레임 추출
- `gifsicle *.gif -d 10 -l >경로/대상/출력파일.gif` : 선택한 GIF로 GIF 애니메이션 만들기
- `gifsicle -b 경로/대상/입력파일.gif -O3 --lossy=100 -k 16 -f` : 손실 압축을 사용하여 파일 크기 줄이기
- `gifsicle -b 경로/대상/입력파일.gif --delete '#0-9' '#20-'` : GIF에서 처음 10개 프레임과 20개 이후 프레임 모두를 삭제
- `gifsicle -b --crop x시작점,y시작점+직사각형_너비x직사각형_높이 --scale 0.25 --flip-horizontal --rotate-90|180|270 경로/대상/입력파일.gif` : 직사각형으로 자르고, 비율을 변경하고, 뒤집고, 회전하여 모든 프레임을 수정

## Links
- More information: https://www.lcdf.org/gifsicle/
