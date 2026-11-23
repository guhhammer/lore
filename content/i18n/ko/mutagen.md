# mutagen

실시간 파일 동기화 및 네트워크 전달 도구.

## Usage
- `mutagen sync create --name=세션_이름 /경로/대상/로컬/폴더/ 사용자@호스트:/경로/대상/원격/폴더/` : 로컬 디렉토리와 원격 호스트 간의 동기화 세션 시작
- `mutagen sync create --name=세션_이름 /경로/대상/로컬/폴더/ docker://사용자@컨테이너_이름/경로/대상/원격/폴더/` : 로컬 디렉토리와 Docker 컨테이너 간의 동기화 세션 시작
- `mutagen sync terminate 세션_이름` : 실행 중인 세션 중지
- `mutagen project start` : 프로젝트 시작
- `mutagen project terminate` : 프로젝트 중지
- `mutagen project list` : 현재 프로젝트의 실행 중인 세션 나열

## Links
- More information: https://mutagen.io/documentation/introduction/
