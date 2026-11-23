# dos2unix

DOS 스타일의 줄 바꿈을 Unix 스타일로 변경. CRLF를 LF로 대체합니다.

## Usage
- `dos2unix 경로/대상/파일` : 파일의 줄 바꿈 변경
- `dos2unix -n 경로/대상/파일 경로/대상/새_파일` : unix 스타일의 줄 바꿈으로 복사본 생성
- `dos2unix -i 경로/대상/파일` : 파일 정보 표시
- `dos2unix --keep-bom|add-bom|remove-bom 경로/대상/파일` : 바이트 순서 표시(BOM) 유지/추가/제거

## Links
- More information: https://manned.org/dos2unix
