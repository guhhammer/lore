# ngrep

`regex`를 사용하여 네트워크 트래픽 패킷을 필터링.

## Usage
- `ngrep -d any` : 모든 인터페이스의 트래픽 캡처
- `ngrep -d eth0` : 특정 인터페이스의 트래픽 캡처
- `ngrep -d eth0 port 22` : 인터페이스 eth0의 포트 22를 지나는 트래픽 캡처
- `ngrep host www.example.com` : 특정 호스트로부터 또는 특정 호스트로 가는 트래픽 캡처
- `ngrep -d eth0 'User-Agent:'` : 인터페이스 eth0의 'User-Agent:' 키워드 필터링

## Links
- More information: https://github.com/jpr5/ngrep/blob/master/EXAMPLES.md
