# cockpit-ws

브라우저 애플리케이션과 `cockpit-bridge`와 같은 다양한 구성 도구 및 서비스 간 통신.

## Usage
- `cockpit-ws --local-ssh` : `127.0.0.1`에서 포트 `22`로 SSH 인증을 통해 시작
- `cockpit-ws --port 포트` : 특정 포트에서 HTTP 서버 시작
- `cockpit-ws --address ip_주소` : 특정 IP 주소에 바인딩하여 시작 (기본값은 `0.0.0.0`)
- `cockpit-ws --no-tls` : TLS 없이 시작
- `cockpit-ws --help` : 도움말 표시

## Links
- More information: https://cockpit-project.org/guide/latest/cockpit-ws.8.html
