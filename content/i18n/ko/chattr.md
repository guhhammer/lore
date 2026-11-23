# chattr

파일 또는 디렉토리의 속성 변경.

## Usage
- `chattr +i 경로/대상/파일_또는_폴더` : 파일 또는 디렉토리를 변경 및 삭제 불가능하도록 설정 (슈퍼유저도 포함)
- `chattr -i 경로/대상/파일_또는_폴더` : 파일 또는 디렉토리를 변경 가능하도록 설정
- `chattr -R +i 경로/대상/폴더` : 전체 디렉토리와 그 내용을 재귀적으로 변경 및 삭제 불가능하도록 설정

## Links
- More information: https://manned.org/chattr
