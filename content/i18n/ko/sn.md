# sn

Mono StrongName 유틸리티로 IL 어셈블리에 서명하고 검증합니다.

## Usage
- `sn -k 경로/대상/키.snk` : 새로운 StrongNaming 키 생성
- `sn -R 경로/대상/어셈블리.dll 경로/대상/키_페어.snk` : 지정된 개인 키로 어셈블리에 다시 서명
- `sn -T 경로/대상/어셈블리.exe` : 어셈블리에 서명하는 데 사용된 개인 키의 공개 키 표시
- `sn -e 경로/대상/어셈블리.dll 경로/대상/출력.pub` : 공개 키를 파일로 추출

## Links
- More information: https://manned.org/sn
