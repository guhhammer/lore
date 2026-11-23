# prqlc

PRQL 컴파일러. PRQL은 데이터를 변환하기 위한 현대적인 언어로, 간단하고 강력한 파이프라인 SQL 대체 언어입니다.

## Usage
- `prqlc compile` : 대화형으로 컴파일러 실행
- `prqlc compile 경로/대상/파일.prql` : 특정 `.prql` 파일을 `stdout`으로 컴파일
- `prqlc compile 경로/대상/소스.prql 경로/대상/타겟.sql` : `.prql` 파일을 `.sql` 파일로 컴파일
- `echo "from employees | filter has_dog | select salary" | prqlc compile` : 쿼리 컴파일
- `prqlc watch 경로/대상/폴더` : 디렉터리를 감시하고 파일 수정 시 컴파일

## Links
- More information: https://prql-lang.org/book/project/integrations/prqlc-cli.html
