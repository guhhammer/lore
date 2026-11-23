# lychee

깨진 URL을 찾기 위한 도구.

## Usage
- `lychee https://example.com` : 웹사이트에서 깨진 링크 스캔
- `lychee -f compact|detailed|json|junit|markdown https://example.com` : 오류 유형의 세부 분류 표시
- `lychee --max-concurrency 5 links.txt` : DDOS 보호를 방지하기 위해 연결 수 제한
- `grep -r "패턴" | lychee -` : 디렉터리 구조 내 파일에서 깨진 URL 확인
- `lychee -h` : 도움말 표시

## Links
- More information: https://lychee.cli.rs/guides/cli/
