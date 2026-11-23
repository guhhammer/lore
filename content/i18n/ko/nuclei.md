# nuclei

간단한 YAML 기반 DSL을 사용하는 빠르고 커스터마이즈 가능한 취약점 스캐너.

## Usage
- `nuclei -ut` : `nuclei` templates를 최신 버전으로 update (다운로드는 `~/nuclei-templates`에 저장됨)
- `nuclei -tl -pt dns|file|http|headless|tcp|workflow|ssl|websocket|whois|code|javascript` : 특정 protocol type의 모든 templates 나열
- `nuclei -as -u scanme.nmap.org` : wappalyzer 기술 감지를 사용하여 자동 웹 스캔을 수행하고 스캔할 대상 uRL/호스트 지정
- `nuclei -severity high,critical -pt http -u http://scanme.sh -me 마크다운_디렉토리` : 높은 및 치명적 심각도의 HTTP protocol type 템플릿을 실행하고, 결과를 특정 디렉토리 내 markdown 파일로 export
- `nuclei -rl 150 -bs 25 -c 25 -silent -u http://scanme.sh` : 다른 rate limit과 최대 bulk size를 사용하여 모든 템플릿 실행하며, 조용한 출력(발견된 내용만 표시)
- `nuclei -w 경로/대상/nuclei-templates/workflows/wordpress-workflow.yaml -u https://sample.wordpress.site` : wordPress 사이트에 대해 WordPress workflow 실행
- `nuclei -t 경로/대상/nuclei-templates/http -u http://scanme.sh -v -o 결과들` : 하나 이상의 특정 templates 또는 templates가 있는 디렉토리를 실행하며 `stderr`에 verbose 출력 및 검출된 문제/취약점을 파일로 output
- `nuclei -tc "contains(tags, 'xss') && contains(tags, 'cve')" -u https://vulnerable.website` : 하나 이상의 template conditions에 기반한 스캔 실행

## Links
- More information: https://docs.projectdiscovery.io/opensource/nuclei/running
