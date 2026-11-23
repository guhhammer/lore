# case

표현식의 값에 근거하여 분기.

## Usage
- `case $tocount in words) wc -w README; ;; lines) wc -l README; ;; esac` : 변수를 문자열 리터럴과 일치시켜 실행할 명령어 결정
- `case $tocount in wW]) wc -w README; ;; lL]) wc -l README; ;; *) echo "what?"; ;; esac` : 패턴을 |와 결합하고, *를 대비책 패턴으로 사용

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-case
