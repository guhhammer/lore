# ecpg

C 프로그램을 위한 Embedded SQL 전처리기.

## Usage
- `ecpg 입력파일.pgc` : 특정 파일 전처리
- `ecpg -o - -` : `stdin`에서 입력받아 `stdout`으로 출력
- `cat input.pgc | ecpg -o output.c -` : `stdin`에서 입력받아 파일로 출력
- `ecpg -o 출력파일.c 입력파일.pgc` : 출력 파일을 지정하여 전처리
- `ecpg 입력파일.pgh` : 헤더 파일 전처리 (`.pgh` 확장자)
- `ecpg -C INFORMIX|INFORMIX_SE|ORACLE 입력파일.pgc` : 특정 호환 모드로 전처리
- `ecpg -t 입력파일.pgc` : 자동 커밋 활성화하여 전처리

## Links
- More information: https://www.postgresql.org/docs/current/app-ecpg.html
