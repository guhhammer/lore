# ausearch

Linux audit 로그에서 이벤트를 조회하는 도구. `audit` 패키지의 일붘.

## Usage
- `sudo ausearch -m avc` : 모든 SELinux AVC 거부 이벤트 검색
- `sudo ausearch -c httpd` : 특정 실행 파일과 관련된 이벤트 검색
- `sudo ausearch -ui 1000` : 특정 사용자 이벤트 검색
- `sudo ausearch -ts recent` : 최근 10분 내 이벤트 검색
- `sudo ausearch -m user_login -sv no` : 로그인 실패 이벤트 검색
- `sudo ausearch -f 경로/대상/파일` : 특정 파일 관련 이벤트 검색
- `sudo ausearch -m avc --raw` : 후처리를 위해 raw 형식으로 결과 출력

## Links
- More information: https://manned.org/ausearch
