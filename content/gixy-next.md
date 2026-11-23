# gixy-next

Analyze NGINX configuration files for security and performance misconfigurations.

## Usage
- `gixy-next path/to/nginx.conf` : analyze `nginx` configuration (default path: `/etc/nginx/nginx.conf`)
- `cat path/to/nginx-dump.conf | gixy-next -` : analyze a rendered configuration dump via `stdin` (`-`)
- `gixy-next --tests http_splitting,ssrf,version_disclosure path/to/nginx.conf` : run only specific checks (comma-separated)
- `gixy-next --skips low_keepalive_requests,worker_rlimit_nofile_vs_connections path/to/nginx.conf` : skip specific checks (comma-separated)
- `gixy-next -l|-ll|-lll path/to/nginx.conf` : only report issues at a given severity or higher
- `gixy-next -f text|json path/to/nginx.conf` : output as uncolored text or machine-readable JSON

## Categories
files, monitoring, text

## Links
- More information: https://gixy.io/usage/
- tldr page: https://tldr.inbrowser.app/pages/common/gixy-next
