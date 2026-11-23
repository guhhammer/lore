# xml elements

XML 문서의 요소를 추출하고 구조를 표시합니다.

## Usage
- `xml el 경로/대상/입력.xml|uri >경로/대상/요소.xpath` : XML 문서에서 요소 추출 (XPATH 표현 생성)
- `xml el -a 경로/대상/입력.xml|uri >경로/대상/요소.xpath` : XML 문서에서 요소와 그 속성 추출
- `xml el -v 경로/대상/입력.xml|uri >경로/대상/요소.xpath` : XML 문서에서 요소, 속성 및 값 추출
- `xml el -u 경로/대상/입력.xml|uri` : XML 문서의 정렬된 고유 요소를 출력하여 구조 확인
- `xml el -d3 경로/대상/입력.xml|uri` : 깊이 3까지의 XML 문서의 정렬된 고유 요소 출력
- `xml el --help` : 도움말 표시

## Links
- More information: https://xmlstar.sourceforge.net/doc/UG/xmlstarlet-ug.html#idm47077139665568
