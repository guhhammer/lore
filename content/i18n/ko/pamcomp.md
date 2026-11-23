# pamcomp

두 개의 PAM 이미지를 오버레이.

## Usage
- `pamcomp 경로/대상/오버레이.pam 경로/대상/언더레이.pam >경로/대상/출력.pam` : 오버레이가 아래 부분을 가리도록 두 이미지를 오버레이
- `pamcomp -ali left|center|right|beyondleft|beyondright -x 수평_오프셋 경로/대상/오버레이.pam 경로/대상/언더레이.pam >경로/대상/출력.pam` : 오버레이의 수평 정렬 설정
- `pamcomp -va top|middle|bottom|above|below -y 수직_오프셋 경로/대상/오버레이.pam 경로/대상/언더레이.pam >경로/대상/출력.pam` : 오버레이의 수직 정렬 설정
- `pamcomp -o 0.7 경로/대상/오버레이.pam 경로/대상/언더레이.pam >경로/대상/출력.pam` : 오버레이의 불투명도 설정

## Links
- More information: https://netpbm.sourceforge.net/doc/pamcomp.html
