# urpmf

파일을 패키지에서 찾고 Mageia에서 해당 정보를 조회.

## Usage
- `urpmf 파일명` : 파일을 포함하는 패키지 검색
- `urpmf --summary 키워드1 -a 키워드2` : 요약에 특정 키워드 a그리고 다른 키워드를 모두 포함하는 패키지 검색
- `urpmf --description 키워드1 -o 키워드2` : 설명에 특정 키워드 o또는 다른 키워드를 포함하는 패키지 검색
- `urpmf --description ! 키워드 -F'|'` : 이름에 특정 키워드를 대소문자 구분 없이 포함하지 않는 패키지를 ":" 대신 "|"를 Field 구분자로 사용하여 검색

## Links
- More information: https://man.linuxreviews.org/man8/urpmf.8.html
