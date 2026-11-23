# immich

Immich에는 커멘드라인에서 특정 작업을 수행할 수 있는 커멘드라인 인터페이스 (CLI)가 있음.

## Usage
- `immich login 서버_주소/api 서버_키` : immich 서버에 인증
- `immich upload 파일1.jpg 파일2.jpg` : 일부 이미지 파일 업로드
- `immich upload -r 경로/대상/디렉터리` : 하위 디렉터리를 포함한 디렉터리 업로드
- `immich upload -r 경로/대상/디렉터리 -A "My summer holiday"` : 디렉터리를 기반으로 앨범 만들기
- `immich upload -r 경로/대상/디렉터리 -i **/Raw/** **/*.tif` : `glob` 패턴과 일치하는 리소스 건너뛰기
- `immich upload -r 경로/대상/디렉터리 -H` : 숨겨진 파일 포함

## Links
- More information: https://docs.immich.app/features/command-line-interface/
