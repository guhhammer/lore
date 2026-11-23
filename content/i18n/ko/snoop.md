# snoop

네트워크 패킷 스니퍼. `tcpdump`와 동일한 기능을 하는 SunOS 대체품.

## Usage
- `snoop -d e1000g0` : 특정 네트워크 인터페이스에서 패킷을 캡처
- `snoop -o 경로/대상/파일` : 화면에 표시하는 대신 캡처된 패킷을 파일에 저장
- `snoop -V -i 경로/대상/파일` : 파일에서 패킷의 상세 프로토콜 레이어 요약 표시
- `snoop to port 포트 from host 호스트명` : 호스트 이름에서 지정된 포트로 가는 네트워크 패킷을 캡처
- `snoop -x0 -p4 ip1 ip2` : 두 IP 주소 간에 교환되는 네트워크 패킷의 hex 덤프를 캡처하고 표시

## Links
- More information: https://www.unix.com/man-page/sunos/1m/snoop
