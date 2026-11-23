# local

지역 변수를 선언하고 속성을 부여.

## Usage
- `local 변수="값"` : 지정한 값으로 문자열 변수 선언
- `local -i 변수="값"` : 지정한 값으로 정수형 변수 선언
- `local 변수=(요소_a 요소_b 요소_c)` : 지정한 값으로 배열 변수 선언
- `local -A variable=([key_a]=요소_a [key_b]=요소_b [key_c]=요소_c)` : 지정한 값으로 연관 배열 변수 선언
- `local -r 변수="값"` : 지정한 값으로 읽기 전용 변수 선언
- `local --help` : 도움말 표시

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-local
