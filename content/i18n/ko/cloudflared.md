# cloudflared

Cloudflare 네트워크와의 지속적인 연결을 생성.

## Usage
- `cloudflared tunnel login` : cloudflare 계정의 도메인과 연결을 인증하고 연결
- `cloudflared tunnel create 이름` : 지정한 이름으로 터널을 생성
- `cloudflared tunnel list` : 계정에 있는 모든 터널 목록을 표시
- `cloudflared tunnel route dns name|uuid 호스트명` : 터널을 가리키는 DNS CNAME 레코드를 생성
- `cloudflared tunnel --loglevel panic|fatal|error|warn|info|debug --logfile 경로/대상/파일 run 이름` : 로그를 파일에 저장
- `cloudflared tunnel run 이름` : 이름이 지정된 터널을 실행 (`config.yml`에서 설정을 읽음)
- `cloudflared tunnel --url http://localhost:포트` : 로컬 서비스를 외부에 노출하기 위한 임시 터널을 시작 (계정 필요 없음)
- `cloudflared service install` : cloudflared를 시스템 서비스로 설치
