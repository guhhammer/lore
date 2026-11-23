# exrex

`regex`에 대해 모두/무작위로 일치하는 문자열을 생성. `regex`을 단순화할 수 있음.

## Usage
- `exrex '정규_표현식'` : `regex`와 일치하는 가능한 모든 문자열을 생성
- `exrex -r '정규_표현식'` : `regex`와 일치하는 임의의 문자열을 생성
- `exrex -m 100 '정규_표현식'` : `regex`와 일치하는 최대 100개의 문자열을 생성
- `exrex -d ", " '정규_표현식'` : 사용자 정의 구분 기호 문자열로 결합된, `regex`와 일치하는 가능한 모든 문자열을 생성
- `exrex -c '정규_표현식'` : `regex`와 일치하는 가능한 모든 문자열의 개수를 출력
- `exrex -s 'ab|ac'` : `regex` 표현식을 단순화
- `exrex '[oO0](_)[oO0]'` : 눈 출력
- `exrex '( {20}(\| *\\|-{22}|\|)|\.={50}| ( ){0,5}\\\.| {12}~{39})'` : 보트 출력

## Links
- More information: https://github.com/asciimoo/exrex#usage
