# ifs

`$IFS` (Internal Field Separator)는 Unix쉘에서 단어 분할에 사용되는 구분 기호를 정의하는 특수 환경 변수. `$IFS`의 기본값은 공백, 탭 및 줄바꿈. 세 문자는 구분 기호 역할을 함.

## Usage
- `echo "$IFS"` : 현재 `$IFS` 값 보기
- `IFS=":"` : `$IFS` 값 변경
- `IFS=$' \t\n'` : `$IFS`를 기본값으로 재설정
- `(IFS=":"; echo "one:two:three")` : 서브셸에서 `$IFS` 값을 일시적으로 변경

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#Word-Splitting
