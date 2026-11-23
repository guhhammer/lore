# acme.sh

`certbot`의 대안으로 ACME 클라이언트 프로토콜을 구현하는 쉘 스크립트.

## Usage
- `acme.sh --issue -d example.com -w /경로/대상/웹루트` : webroot 모드를 사용해 인증서를 발급
- `acme.sh --issue --standalone -d example.com -d www.example.com` : 80번 포트와 독립 실행형 모드를 사용해 여러 도메인에 대한 인증서를 발급
- `acme.sh --issue --alpn -d example.com` : 443번 포트와 독립 실행형 TLS 모드를 사용해 인증서 발급
- `acme.sh --issue --nginx -d example.com` : 작동하는 `nginx` 구성파일을 사용해 인증서 발급
- `acme.sh --issue --apache -d example.com` : 작동하는 Apache 구성파일을 사용해 인증서 발급
- `acme.sh --issue --dns dns_인증서 -d *.example.com` : 자동 DNS API 모드를 사용해 와일드카드 (\*) 인증서를 발급
- `acme.sh -i -d example.com --key-file /경로/대상/example.com.key --fullchain-file /경로/대상/example.com.cer --reloadcmd "systemctl force-reload nginx"` : 지정된 위치에 인증서 파일 설치 (자동 인증서 갱신에 장점이 있음)

## Links
- More information: https://github.com/acmesh-official/acme.sh#2-just-issue-a-cert
