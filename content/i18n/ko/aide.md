# aide

파일 무결성을 검증하기 위한 고급 침임 탐지 환경.

## Usage
- `sudo aide -i` : 데이터베이스 초기화
- `sudo aide -C` : 데이터베이스의 불일치 여부 검사
- `sudo aide -E` : 설정 파일 정의에 따라 두 데이터베이스를 비교
- `sudo aide -u` : 비대화식으로 검사 후 데이터베이스 업데이트
- `sudo aide -c 경로/대상/설정_파일` : 기본 `aide.conf` 대신 사용할 설정 파일 지정
- `sudo aide -l regex` : 특정 문자열에 대해 `regex`로 검사 AIDE 범위 제한
- `sudo aide -r 리포트_주소` : 리포트 결과를 URL로 전송

## Links
- More information: https://manned.org/aide
