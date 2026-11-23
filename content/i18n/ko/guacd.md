# guacd

Apache Guacamole 프록시 데몬. Guacamole 프로토콜과 임의의 원격 데스크톱 프로토콜(예. RDP, VNC, 기타) 간의 인터페이스를 위한 클라이언트 플러그인용 로더를 지원ㄴ.

## Usage
- `guacd -b 127.0.0.1 -l 4823` : localhost의 특정 포트에 바인딩
- `guacd -f -L debug` : 디버그 모드에서 시작하여, 프로세스를 포그라운드에 유지
- `guacd -C my-cert.crt -K my-key.pem` : TLS 지원과 함께 시작
- `guacd -p path/to/file.pid` : PID을 파일에 작성

## Links
- More information: https://manned.org/guacd
