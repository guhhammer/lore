# flarectl

Cloudflare 공식 CLI.

## Usage
- `flarectl firewall rules create --zone="example.com" --value="8.8.8.8" --mode="block" --notes="Block bad actor"` : 특정 IP 차단
- `flarectl dns create --zone="example.com" --name="app" --type="CNAME" --content="myapp.herokuapp.com" --proxy` : DNS 레코드 추가
- `flarectl ips --ip-type ipv4|ipv6|all` : 모든 Cloudflare IPv4/IPv6 범위 나열
- `for domain in $(cat domains.txt); do flarectl zone info --zone=$domain; done` : `domains.txt`의 이름을 사용하여 자동으로 많은 새 Cloudflare 영역을 생성
- `flarectl firewall rules list` : 모든 방화벽 규칙을 나열

## Links
- More information: https://github.com/cloudflare/cloudflare-go/blob/v0/cmd/flarectl/README.md
