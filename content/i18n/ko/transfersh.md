# transfersh

비공식적인 transfer.sh 명령줄 클라이언트.

## Usage
- `transfersh 경로/대상/파일` : 파일을 transfer.sh에 업로드
- `transfersh -p 경로/대상/파일` : 진행 표시줄을 보여주며 파일 업로드 (Python 패키지 `requests_toolbelt` 필요)
- `transfersh -n 파일명 경로/대상/파일` : 다른 파일 이름으로 파일 업로드
- `transfersh -sn 업로드.서버.이름 경로/대상/파일` : 사용자 지정 transfer.sh 서버에 파일 업로드
- `transfersh -r 경로/대상/폴더/` : 디렉터리의 모든 파일을 재귀적으로 업로드
- `transfersh -rt 경로/대상/폴더` : 특정 디렉터리를 압축되지 않은 tar로 업로드

## Links
- More information: https://github.com/AlpixTM/transfersh
