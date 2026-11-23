# xml transform

XSLT를 사용하여 XML 문서를 변환.

## Usage
- `xml tr 경로/대상/스타일시트.xsl -p "Count='count(/xml/table/rec)'" -s Text="Count=" 경로/대상/입력.xml|uri` : XSL 스타일시트를 사용하여 XML 문서를 변환하고, 하나의 XPATH 매개변수와 하나의 리터럴 문자열 매개변수를 전달
- `xml tr --help` : 도움말 표시

## Links
- More information: https://xmlstar.sourceforge.net/doc/UG/xmlstarlet-ug.html#idm47077139602800
