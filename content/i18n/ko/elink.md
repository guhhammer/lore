# elink

데이터베이스 내에서 미리 계산된 이웃을 조회하거나 다른 데이터베이스에서 관련 레코드를 찾습니다. `edirect` 패키지의 일부입니다.

## Usage
- `esearch -db pubmed -query "선택적 세로토닌 재흡수 억제제" | elink -target nuccore` : pubmed를 검색한 후 관련 시퀀스 찾기
- `esearch -db nuccore -query "인슐린 [PROT] AND 설치류 [ORGN]" | elink -target biosample` : 뉴클레오타이드를 검색한 후 관련 생물 샘플 찾기

## Links
- More information: https://www.ncbi.nlm.nih.gov/books/NBK179288/
