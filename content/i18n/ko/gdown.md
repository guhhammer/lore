# gdown

Google Drive 및 기타 URL에서 파일을 다운로드.

## Usage
- `gdown 주소` : URL에서 파일 다운로드
- `gdown 파일_아이디` : 파일 ID를 사용하여 다운로드
- `gdown --fuzzy 주소` : 퍼지 파일 ID 추출을 사용하여 다운로드 (<https://docs.google.com> 링크에서도 작동)
- `gdown 폴더_아이디|주소 -O 경로/대상/출력_디렉토리 --folder` : 해당 ID 또는 전체 URL을 사용하여 폴더를 다운로드
- `gdown tar압축파일_주소 -O - -q | tar xvf -` : `.tar` 아카이브를 다운로드하고, `stdout`에 쓴 후 추출

## Links
- More information: https://github.com/wkentaro/gdown#usage
