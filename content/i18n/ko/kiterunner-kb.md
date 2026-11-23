# kiterunner kb

API 및 웹 엔드포인트 탐지에 사용되는 Kitebulder 스키마를 조작하는 컨텍스트 기반 웹 스캐너. `kb` 하위 명령은 스키마 컴파일, 변환, 파싱 및 요청 재실행을 처리.

## Usage
- `kiterunner kb compile 경로/대상/워드리스트.json 경로/대상/워드리스트.kite` : JSON 형식의 kitebuilder 스키마를 kite 파일로 컴파일
- `kiterunner kb convert 경로/대상/워드리스트.kite 경로/대상/워드리스트.txt` : kite 파일을 텍스트 워드리스트로 변환
- `kiterunner kb convert 경로/대상/워드리스트.txt 경로/대상/워드리스트.kite` : 텍스트 워드리스트를 kite 파일로 변환
- `kiterunner kb convert 경로/대상/워드리스트.kite 경로/대상/워드리스트.json` : kite 파일을 JSON 스키마로 변환
- `kiterunner kb parse 경로/대상/워드리스트.json -o json` : kitebuilder 스키마를 파싱하고 보기 좋게 정리된 JSON 데이터 출력
- `kiterunner kb parse 경로/대상/워드리스트.kite -o 텍스트` : kite 파일을 파싱하고 보기 좋게 정리된 텍스트 데이터 출력
- `kiterunner kb replay -w 경로/대상/워드리스트.kite "요청_출력결과"` : kitebuilder 스키마 출력에서 특정 요청을 재실행
- `kiterunner kb replay -w 경로/대상/워드리스트.kite -p http://localhost:8080 "요청_출력결과"` : 프록시를 통해 요청을 재실행하여 검사 및 분석 수행

## Links
- More information: https://github.com/assetnote/kiterunner#usage
