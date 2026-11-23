# pnpm audit

프로젝트 의존성을 스캔합니다. 설치된 패키지에서 알려진 보안 문제를 확인합니다.

## Usage
- `pnpm audit` : 프로젝트의 취약점 식별
- `pnpm audit fix` : 취약점 자동 수정
- `pnpm audit --json >경로/대상/audit-report.json` : JSON 형식의 보안 보고서 생성
- `pnpm audit -D` : dev 의존성만 감사
- `pnpm audit -P` : production 의존성만 감사
- `pnpm audit --no-optional` : 선택적 의존성을 감사에서 제외
- `pnpm audit --ignore-registry-errors` : 감사 과정에서 레지스트리 오류 무시
- `pnpm audit --audit-level 심각도` : 심각도별로 자문 필터링 (low, moderate, high, critical)

## Links
- More information: https://pnpm.io/cli/audit
