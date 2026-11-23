# security-checker

PHP 애플리케이션이 알려진 보안 취약점을 가진 의존성을 사용하는지 확인.

## Usage
- `security-checker security:check` : 프로젝트 의존성에서 보안 문제 검색 (현재 디렉토리의 `composer.lock` 파일 기준)
- `security-checker security:check 경로/대상/composer.lock` : 특정 `composer.lock` 파일 사용
- `security-checker security:check --format=json` : 결과를 JSON 객체로 반환

## Links
- More information: https://github.com/sensiolabs/security-checker
