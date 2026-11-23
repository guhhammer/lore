# dalfox

자동화에 중점을 둔 강력한 오픈소스 XSS 스캐너.

## Usage
- `dalfox url http://example.com` : XSS 취약점에 대한 단일 URL을 스캔
- `dalfox url https://example.com -H 'X-My-Header: 123'` : 인증을 위해 헤더를 사용해 URL을 스캔
- `dalfox file 경로/대상/파일` : 파일에서 URL 목록을 스캔
- `dalfox server --host 0.0.0.0 --port 8080` : dalfox를 REST API 서버로 시작

## Links
- More information: https://dalfox.hahwul.com/reference/cli/
