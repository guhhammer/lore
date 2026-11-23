# boxes

ASCII 아트 상자 그리기, 제거 및 복구.

## Usage
- `echo "문자열" | boxes` : 문자열 주위에 상자 그리기
- `echo "문자열" | boxes -r` : 문자열에서 상자를 제거r
- `echo "문자열" | boxes -d parchment` : 문자열 주위에 특정 디자인d의 상자 그리기
- `echo "문자열" | boxes -s 10x5` : 상자 크기s 지정(열 단위)
- `echo "문자열" | boxes -a hl|c|r` : 상자 텍스트 수평h 정렬a(왼쪽l, 중앙c, 오른쪽r)
- `echo "문자열" | boxes -a vt|c|b` : 상자 텍스트 수직v 정렬a(위쪽t, 중앙c, 아래쪽b)
- `echo "문자열" | boxes -a jl|c|rvt` : 상자 텍스트 양쪽 조정j(왼쪽l, 중앙c, 오른쪽r)

## Links
- More information: https://boxes.thomasjensen.com/boxes-man-1.html
