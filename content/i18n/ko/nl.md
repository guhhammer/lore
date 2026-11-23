# nl

파일 또는 `stdin`에서 각 행에 번호를 매깁니다.

## Usage
- `nl 경로/대상/파일` : 파일에서 빈 줄이 아닌 행에 번호 매기기
- `명령어 | nl -` : `stdin`에서 읽기
- `nl -b a|n 경로/대상/파일` : 빈 줄을 포함한 all body 행에 번호를 매기거나 not body 행에 번호를 매기지 않음
- `nl -b p'FooBar[0-9]' 경로/대상/파일` : 기본 정규 표현식(BRE) pattern과 일치하는 body 행에만 번호 매기기
- `nl -i 증가량 경로/대상/파일` : 특정 increment로 행 번호 매기기
- `nl -n rz|ln|rn` : 행 번호 형식을 right 또는 left로 정렬하고 선행 zeros를 유지하거나 not
- `nl -w 열_너비 경로/대상/파일` : 행 번호의 width 지정 (기본값은 6)
- `nl -s 구분자 경로/대상/파일` : 행 번호와 행 사이를 구분하기 위한 특정 문자열 사용 (기본값은 TAB)

## Links
- More information: https://manned.org/nl.1p
