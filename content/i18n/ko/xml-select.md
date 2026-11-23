# xml select

XPATH를 사용하여 XML 문서에서 선택. 팁: XML 문서의 XPATH를 표시하려면 `xml elements`를 사용하세요.

## Usage
- `xml sel -t -m "XPATH1" -v "XPATH2" 경로/대상/입력.xml|uri` : "XPATH1"과 일치하는 모든 요소를 선택하고 그 하위 요소 "XPATH2"의 값을 출력
- `xml sel -T -t -m "XPATH1" -v "XPATH2" -n 경로/대상/입력.xml|uri` : "XPATH1"과 일치하는 요소를 선택하고 "XPATH2"의 값을 새 줄과 함께 텍스트로 출력
- `xml sel -t -v "count(XPATH1)" 경로/대상/입력.xml|uri` : "XPATH1"의 요소 수를 계산
- `xml sel -T -t -f -o " " -v "count(node())" -n 경로/대상/입력1.xml|uri 경로/대상/입력2.xml|uri` : 하나 이상의 XML 문서에서 모든 노드 수를 계산
- `xml sel --help` : 도움말 표시

## Links
- More information: https://xmlstar.sourceforge.net/doc/UG/xmlstarlet-ug.html#idm47077139652416
