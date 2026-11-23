# forfiles

지정한 명령어를 실행할 파일을 선택.

## Usage
- `forfiles` : 현재 디렉토리에서 파일 검색
- `forfiles /p 경로\대상\폴더` : 특정 디렉토리에서 파일 검색
- `forfiles /c "명령어"` : 각 파일에 대해 지정된 명령어 실행
- `forfiles /m 글로브_패턴` : 특정 글로브 패턴을 사용하여 파일 검색
- `forfiles /s` : 파일을 재귀적으로 검색
- `forfiles /d +5` : 5일 이상된 파일 검색
