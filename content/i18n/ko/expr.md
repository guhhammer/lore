# expr

표현식을 평가하고 문자열을 조작.

## Usage
- `expr length "문자열"` : 특정 문자열의 길이를 가져옴
- `expr substr "문자열" 시작 길이` : 특정 길이의 문자열의 하위 문자열을 가져옴
- `expr match "문자열" '패턴'` : 고정된 패턴과 특정 하위 문자열을 일치시킴
- `expr index "문자열" "문자"` : 문자열의 특정 세트에서 첫 번째 문자 위치를 가져옴
- `expr 표현식1 +|-|*|/|% 표현식2` : 특정 수학적 표현을 계산
- `expr 표현식1 \| 표현식2` : 값이 0이 아니며 null이 아닌 경우, 첫 번째 표현식을 가져오고, 그렇지 않으면 두 번째 표현식을 가져옴
- `expr 표현식1 \& 표현식2` : 두 표현식이 모두 0이 아니고 null이 아닌 경우, 첫 번째 표현식을 가져오고 그렇지 않으면 0을 얻음

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/expr-invocation.html
