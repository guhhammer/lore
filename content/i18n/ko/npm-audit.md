# npm audit

프로젝트 의존성에서 알려진 취약점을 스캔. 취약점을 보고하고 해결 방법을 제안.

## Usage
- `npm audit` : 프로젝트의 의존성에서 알려진 취약점을 스캔
- `npm audit fix` : 프로젝트의 의존성에서 취약점을 자동으로 수정
- `npm audit fix -f` : 취약점을 가진 의존성을 강제로 자동 수정
- `npm audit fix --package-lock-only` : `node_modules` 디렉터리를 수정하지 않고 lock 파일 업데이트
- `npm audit fix --dry-run` : 시뮬레이션 실행. 실제로 변경하지 않고 수정 과정을 시뮬레이션
- `npm audit --json` : 감사 결과를 JSON 형식으로 출력
- `npm audit --audit-level info|low|moderate|high|critical` : 특정 심각도 이상의 취약점에서만 실패하도록 감사 구성

## Links
- More information: https://docs.npmjs.com/cli/npm-audit/
