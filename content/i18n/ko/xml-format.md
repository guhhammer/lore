# xml format

XML 문서를 포맷합니다.

## Usage
- `xml fo -t 경로/대상/입력.xml|uri >경로/대상/출력.xml` : XML 문서를 탭으로 들여쓰기하여 포맷
- `xml fo -H -s 4 경로/대상/입력.html|uri >경로/대상/출력.html` : HTML 문서를 4칸의 공백으로 들여쓰기하여 포맷
- `xml fo -R -n 경로/대상/잘못된.xml|uri >경로/대상/복구된.xml` : 잘못된 XML 문서에서 구문 분석이 가능한 부분을 복구하고 들여쓰지 않음
- `cat 경로/대상/입력.xml | xml fo -D >경로/대상/출력.xml` : `stdin`에서 XML 문서를 포맷하고 `DOCTYPE` 선언을 제거
- `xml fo -o 경로/대상/입력.xml|uri >경로/대상/출력.xml` : XML 선언을 생략하여 XML 문서를 포맷
- `xml fo --help` : 도움말 표시

## Links
- More information: https://xmlstar.sourceforge.net/doc/UG/xmlstarlet-ug.html#idm47077139569312
