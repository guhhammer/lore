# httpx

A fast and multi-purpose HTTP toolkit written in Go to run multiple probes at once. Note: Not to be confused with the unrelated Python's HTTPX which has the same command name.

## Usage
- `httpx -probe -u url|host|ipaddress|subnet_with_cidr` : run a probe against a uRL, host, IP Address, or subnet (CIDR notation) showing probe status
- `subfinder -d example.com | httpx -sc` : run a probe against multiple hosts showing status code with input from `subfinder`
- `httpx -rl 150 -l path/to/newline_separated_hosts_list -td -rt` : run a rate limited probe against a list of hosts from a file showing technology detected and response time
- `httpx -u url -title -cdn -hash sha256` : run a probe against a uRL showing its webpage title, CDN/WAF in use, and page content hash
- `httpx -probe -u host1,host2,... -p http:80,8000-8080,https:443,8443 -timeout 10` : run a probe against a list of hosts with custom defined ports and timeout after certain seconds
- `httpx -u host1,host2,... -fc 400,401,404` : run a probe against a list of hosts filtering out codes of certain responses
- `httpx -u host1,host2,... -mc 200,301,304` : run a probe against a list of hosts matching codes of certain responses
- `httpx -u https://www.github.com -path /tldr-pages/tldr,/projectdiscovery/httpx -ss -st 10` : run a probe against a URL saving screenshots of certain paths, with screenshot timeouts (assets are saved in `output`)

## Categories
network, files

## Links
- More information: https://docs.projectdiscovery.io/opensource/httpx/running
- tldr page: https://tldr.inbrowser.app/pages/common/httpx
