# webstorm

JetBrains JavaScript IDE.

## Usage
- `webstorm` : 현재 디렉토리를 WebStorm에서 열기
- `webstorm 경로/대상/폴더` : 특정 디렉토리를 WebStorm에서 열기
- `webstorm -e 경로/대상/파일1 경로/대상/파일2 ...` : 특정 파일들을 LightEdit 모드에서 열기
- `webstorm --wait -e 경로/대상/파일` : 특정 파일을 LightEdit 모드에서 열고 편집이 완료될 때까지 대기
- `webstorm --line 줄_번호 경로/대상/파일` : 특정 줄에 커서를 두고 파일 열기
- `webstorm diff 경로/대상/파일1 경로/대상/파일2 경로/대상/선택_파일3` : 파일을 열고 비교 (최대 3개 파일 지원)
- `webstorm merge 경로/대상/왼쪽_파일 경로/대상/오른쪽_파일 경로/대상/대상_파일` : 3방향 병합 수행하기
