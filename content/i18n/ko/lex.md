# lex

어휘 분석기 생성기. 어휘 분석기의 명세가 주어지면 이를 구현하는 C 코드를 생성합니다. 참고: 대부분의 주요 운영 체제에서 이 명령은 `flex`의 별칭입니다.

## Usage
- `lex 분석기.l` : lex 파일로부터 분석기를 생성하고, `lex.yy.c` 파일에 저장
- `lex -t 분석기.l >분석기.c` : 출력 파일 지정
- `c99 경로/대상/lex.yy.c -o 실행_파일` : lex가 생성한 C 파일 컴파일

## Links
- More information: https://manned.org/lex
