# xml canonic

XML 문서를 정규화.

## Usage
- `xml c14n 경로/대상/입력.xml|uri >경로/대상/출력.xml` : XML 문서를 정규화하여 주석을 보존
- `xml c14n --without-comments 경로/대상/입력.xml|uri >경로/대상/출력.xml` : XML 문서를 정규화하여 주석 제거
- `xml c14n --exc-with-comments 경로/대상/입력.xml|uri 경로/대상/c14n.xpath` : 파일의 XPATH를 사용하여 XML을 독점적으로 정규화하고, 주석을 보존
- `xml c14n --help` : 도움말 표시

## Links
- More information: https://xmlstar.sourceforge.net/doc/UG/xmlstarlet-ug.html#idm47077139560880
