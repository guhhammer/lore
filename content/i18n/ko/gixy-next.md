# gixy-next

NGINX 설정 파일의 보안 및 성능 오설정 분석.

## Usage
- `gixy-next 경로/대상/nginx.conf` : `nginx` 설정 분석 (기본 경로: `/etc/nginx/nginx.conf`)
- `cat 경로/대상/nginx-dump.conf | gixy-next -` : `stdin` (`-`)으로 전달된 렌더링된 설정 덤프 분석
- `gixy-next --tests http_splitting,ssrf,version_disclosure 경로/대상/nginx.conf` : 특정 검사만 실행 (쉼표로 구분)
- `gixy-next --skips low_keepalive_requests,worker_rlimit_nofile_vs_connections 경로/대상/nginx.conf` : 특정 검사 건너뛰기 (쉼표로 구분)
- `gixy-next -l|-ll|-lll 경로/대상/nginx.conf` : 지정한 심각도 이상 문제만 보고
- `gixy-next -f text|json 경로/대상/nginx.conf` : 색상 없는 텍스트 또는 기계 판독용 JSON 형식으로 출력

## Links
- More information: https://gixy.io/usage/
