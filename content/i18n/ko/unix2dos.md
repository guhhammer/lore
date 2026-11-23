# unix2dos

Unix 스타일 줄 끝을 DOS 스타일로 변경. LF를 CRLF로 대체.

## Usage
- `unix2dos 경로/대상/파일` : 파일의 줄 끝 변경
- `unix2dos -n 경로/대상/파일 경로/대상/새_파일` : DOS 스타일 줄 끝으로 복사본 생성
- `unix2dos -i 경로/대상/파일` : 파일 정보 표시
- `unix2dos --keep-bom|add-bom|remove-bom 경로/대상/파일` : 바이트 순서 표시(Byte Order Mark) 유지/추가/제거

## Links
- More information: https://manned.org/unix2dos
