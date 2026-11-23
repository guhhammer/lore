# bun audit

설치된 패키지의 알려진 보안 취약점을 검사합니다.

## Usage
- `bun audit` : `bun.lock` 파일이 포함된 프로젝트의 모든 의존성을 점검
- `bun audit --audit-level low|moderate|high|critical` : 지정한 보안 수준 이상의 취약점만 표시
- `bun audit --prod` : 운영 환경용 의존성만 보안 점검
- `bun audit --ignore CVE-XXXX-YYYY` : 특정 CVE ID를 무시
- `bun audit --json` : 가공되지 않은 JSON 리포트를 출력

## Links
- More information: https://bun.com/docs/pm/cli/audit
