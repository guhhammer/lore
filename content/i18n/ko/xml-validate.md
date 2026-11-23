# xml validate

XML 문서 유효성 검사.

## Usage
- `xml val 경로/대상/입력1.xml|uri1 경로/대상/입력2.xml|uri2 ...` : 하나 이상의 XML 문서가 잘 형성되었는지 검사
- `xml val -d 경로/대상/스키마.dtd 경로/대상/입력1.xml|uri1 경로/대상/입력2.xml|uri2 ...` : 하나 이상의 XML 문서를 문서 유형 정의(DTD)와 비교하여 유효성 검사
- `xml val -s 경로/대상/스키마.xsd 경로/대상/입력1.xml|uri1 경로/대상/입력2.xml|uri2 ...` : 하나 이상의 XML 문서를 XML 스키마 정의(XSD)와 비교하여 유효성 검사
- `xml val -r 경로/대상/스키마.rng 경로/대상/입력1.xml|uri1 경로/대상/입력2.xml|uri2 ...` : 하나 이상의 XML 문서를 Relax NG 스키마(RNG)와 비교하여 유효성 검사
- `xml val --help` : 도움말 표시

## Links
- More information: https://xmlstar.sourceforge.net/doc/UG/xmlstarlet-ug.html#idm47077139576400
