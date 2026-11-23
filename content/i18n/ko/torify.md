# torify

네트워크 트래픽을 Tor 네트워크를 통해 라우팅. 참고: 이 명령은 더 이상 사용되지 않으며, 이제 `torsocks`의 하위 호환 래퍼입니다.

## Usage
- `torify 명령` : 트래픽을 Tor를 통해 라우팅
- `torify on|off` : 셸에서 Tor 토글
- `torify --shell` : tor 사용 셸 생성
- `torify show` : tor 사용 셸 확인
- `torify -c 설정_파일 명령` : tor 설정 파일 지정
- `torify -P 프록시 명령` : 특정 Tor SOCKS 프록시 사용
- `torify 명령 >경로/대상/출력` : 출력 결과를 파일로 리다이렉트

## Links
- More information: https://manned.org/torify
