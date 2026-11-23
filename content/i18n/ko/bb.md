# bb

스크립팅을 위한 기본 Clojure 인터프리터.

## Usage
- `bb -e "(+ 1 2 3)"` : 표현식 평가
- `bb -f 경로/대상/스크립트.clj` : 스크립트 파일 평가
- `printf "first\nsecond" | bb -i "(map clojure.string/capitalize *input*)"` : `stdin`의 일련의 라인에 입력을 바인딩
- `echo "{:key 'val}" | bb -I "(:key (first *input*))"` : `stdin`의 EDN(확장 가능한 데이터 표기법) 값 시퀀스에 입력을 바인딩

## Links
- More information: https://book.babashka.org/#usage
